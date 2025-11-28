use std::fmt;

use rustc_ast::Mutability;
use rustc_macros::HashStable;
use rustc_type_ir::elaborate;

use crate::mir::interpret::{
    AllocId, AllocInit, Allocation, CTFE_ALLOC_SALT, Pointer, Scalar, alloc_range,
};
use crate::ty::{self, Instance, TraitRef, Ty, TyCtxt};

#[derive(Clone, Copy, PartialEq, HashStable)]
pub enum VtblEntry<'tcx> {
    /// destructor of this type (used in vtable header)
    MetadataDropInPlace,
    /// layout size of this type (used in vtable header)
    MetadataSize,
    /// layout align of this type (used in vtable header)
    MetadataAlign,
    /// non-dispatchable associated function that is excluded from trait object
    Vacant,
    /// dispatchable associated function
    Method(Instance<'tcx>),
    /// pointer to a separate supertrait vtable, can be used by trait upcasting coercion
    TraitVPtr(TraitRef<'tcx>),
}

impl<'tcx> VtblEntry<'tcx> {
    /// Return the [`rustc_abi::Size`] it takes to represent in-memory the given entry kind.
    pub fn memory_size(&self, ctxt: &impl rustc_abi::HasDataLayout) -> rustc_abi::Size {
        let dl = ctxt.data_layout();
        match self {
            VtblEntry::MetadataSize | VtblEntry::MetadataAlign => dl.pointer_offset(),
            VtblEntry::MetadataDropInPlace
            | VtblEntry::Method(_)
            | VtblEntry::TraitVPtr(_)
            | VtblEntry::Vacant => dl.pointer_size(),
        }
    }

    /// Return the [`rustc_abi::Size`] of the data the given entry kind can contain.
    pub fn data_size(&self, ctxt: &impl rustc_abi::HasDataLayout) -> rustc_abi::Size {
        ctxt.data_layout().pointer_offset()
    }
}

impl<'tcx> fmt::Debug for VtblEntry<'tcx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We want to call `Display` on `Instance` and `PolyTraitRef`,
        // so we implement this manually.
        match self {
            VtblEntry::MetadataDropInPlace => write!(f, "MetadataDropInPlace"),
            VtblEntry::MetadataSize => write!(f, "MetadataSize"),
            VtblEntry::MetadataAlign => write!(f, "MetadataAlign"),
            VtblEntry::Vacant => write!(f, "Vacant"),
            VtblEntry::Method(instance) => write!(f, "Method({instance})"),
            VtblEntry::TraitVPtr(trait_ref) => write!(f, "TraitVPtr({trait_ref})"),
        }
    }
}

// Needs to be associated with the `'tcx` lifetime
impl<'tcx> TyCtxt<'tcx> {
    pub const COMMON_VTABLE_ENTRIES: &'tcx [VtblEntry<'tcx>] =
        &[VtblEntry::MetadataDropInPlace, VtblEntry::MetadataSize, VtblEntry::MetadataAlign];
}

pub const COMMON_VTABLE_ENTRIES_DROPINPLACE: usize = 0;
pub const COMMON_VTABLE_ENTRIES_SIZE: usize = 1;
pub const COMMON_VTABLE_ENTRIES_ALIGN: usize = 2;

// Note that we don't have access to a self type here, this has to be purely based on the trait (and
// supertrait) definitions. That means we can't call into the same vtable_entries code since that
// returns a specific instantiation (e.g., with Vacant slots when bounds aren't satisfied). The goal
// here is to do a best-effort approximation without duplicating a lot of code.
//
// This function is used in layout computation for e.g. &dyn Trait, so it's critical that this
// function is an accurate approximation. We verify this when actually computing the vtable below.
pub(crate) fn vtable_min_entries<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_ref: Option<ty::ExistentialTraitRef<'tcx>>,
) -> usize {
    let mut count = TyCtxt::COMMON_VTABLE_ENTRIES.len();
    let Some(trait_ref) = trait_ref else {
        return count;
    };

    // This includes self in supertraits.
    for def_id in elaborate::supertrait_def_ids(tcx, trait_ref.def_id) {
        count += tcx.own_existential_vtable_entries(def_id).len();
    }

    count
}

/// Retrieves an allocation that represents the contents of a vtable.
/// Since this is a query, allocations are cached and not duplicated.
///
/// This is an "internal" `AllocId` that should never be used as a value in the interpreted program.
/// The interpreter should use `AllocId` that refer to a `GlobalAlloc::VTable` instead.
/// (This is similar to statics, which also have a similar "internal" `AllocId` storing their
/// initial contents.)
pub(super) fn vtable_allocation_provider<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: (Ty<'tcx>, Option<ty::ExistentialTraitRef<'tcx>>),
) -> AllocId {

    // FIXME(xdoardo): Figure out to remove the overhead for the computations relative to the sizes
    // of the entries happens only for targets that need it (i.e. those where sizeof(usize) !=
    // sizeof(ptr)

    let (ty, poly_trait_ref) = key;

    let vtable_entries = if let Some(poly_trait_ref) = poly_trait_ref {
        let trait_ref = poly_trait_ref.with_self_ty(tcx, ty);
        let trait_ref = tcx.erase_and_anonymize_regions(trait_ref);

        tcx.vtable_entries(trait_ref)
    } else {
        TyCtxt::COMMON_VTABLE_ENTRIES
    };

    // This confirms that the layout computation for &dyn Trait has an accurate sizing.
    assert!(vtable_entries.len() >= vtable_min_entries(tcx, poly_trait_ref));

    let layout = match tcx.layout_of(ty::TypingEnv::fully_monomorphized().as_query_input(ty)) {
        Ok(layout) => layout,
        Err(e) => tcx.dcx().emit_fatal(e.into_diagnostic()),
    };
    assert!(layout.is_sized(), "can't create a vtable for an unsized type");
    let size = layout.size.bytes();
    let align = layout.align.bytes();

    let usize_size = tcx.data_layout.pointer_offset();
    let ptr_align = tcx.data_layout.pointer_align().abi;
    let size_zero = rustc_abi::Size::from_bits(0);

    let entries_memory_size =
        vtable_entries.iter().map(|e| e.memory_size(&tcx)).collect::<Vec<_>>();
    let entries_data_size = vtable_entries.iter().map(|e| e.data_size(&tcx)).collect::<Vec<_>>();
    let vtable_size = entries_memory_size.iter().fold(size_zero, |a, s| a + *s);
    let mut vtable = Allocation::new(vtable_size, ptr_align, AllocInit::Uninit, ());
    let mut field_offset = size_zero;

    // We must check that the offsetting in steps of `usizes` does not break the alignment
    // requirements of the other entries.
    assert!(
        usize_size.bits() % ptr_align.bits() == 0 || ptr_align.bits() % usize_size.bits() == 0,
        "usize_size: {usize_size:?}, ptr_alignment: {ptr_align:?}"
    );

    for (idx, entry) in vtable_entries.iter().enumerate() {
        let scalar = match *entry {
            VtblEntry::MetadataDropInPlace => {
                if ty.needs_drop(tcx, ty::TypingEnv::fully_monomorphized()) {
                    let instance = ty::Instance::resolve_drop_in_place(tcx, ty);
                    let fn_alloc_id = tcx.reserve_and_set_fn_alloc(instance, CTFE_ALLOC_SALT);
                    let fn_ptr = Pointer::from(fn_alloc_id);
                    Scalar::from_pointer(fn_ptr, &tcx)
                } else {
                    Scalar::from_maybe_pointer(Pointer::null(), &tcx)
                }
            }
            VtblEntry::MetadataSize => Scalar::from_uint(size, usize_size),
            VtblEntry::MetadataAlign => Scalar::from_uint(align, usize_size),
            VtblEntry::Vacant => {
                field_offset += entries_memory_size[idx];
                continue;
            }
            VtblEntry::Method(instance) => {
                // Prepare the fn ptr we write into the vtable.
                let fn_alloc_id = tcx.reserve_and_set_fn_alloc(instance, CTFE_ALLOC_SALT);
                let fn_ptr = Pointer::from(fn_alloc_id);
                Scalar::from_pointer(fn_ptr, &tcx)
            }
            VtblEntry::TraitVPtr(trait_ref) => {
                let super_trait_ref = ty::ExistentialTraitRef::erase_self_ty(tcx, trait_ref);
                let supertrait_alloc_id = tcx.vtable_allocation((ty, Some(super_trait_ref)));
                let vptr = Pointer::from(supertrait_alloc_id);
                Scalar::from_pointer(vptr, &tcx)
            }
        };

        let current_offset = field_offset;
        let field_data_size = entries_data_size[idx];
        field_offset += entries_memory_size[idx];

        vtable
            .write_scalar(&tcx, alloc_range(current_offset, field_data_size), scalar)
            .expect("failed to build vtable representation");
    }

    vtable.mutability = Mutability::Not;
    tcx.reserve_and_set_memory_alloc(tcx.mk_const_alloc(vtable))
}
