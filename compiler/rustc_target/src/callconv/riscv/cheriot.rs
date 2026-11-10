use rustc_abi::{HasDataLayout, Primitive, Reg, TyAbiInterface};

use crate::callconv::{ArgAbi, CastTarget, FnAbi};
use crate::spec::{HasTargetSpec, LlvmAbi};

/// Explains the result of the [`classify_purecap_arg`] and [`classify_purecap_ret`] functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
enum PureCapAbiResult {
    /// Fall back to the default behaviour.
    Fallback,
    /// Don't touch this argument anymore.
    Done,
}

pub(crate) fn compute_purecap_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    compute_purecap_abi_info_inner(cx, fn_abi)
}

pub(crate) fn compute_rust_purecap_abi_info<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    /* For now we do nothing CHERIoT-specific, we just copy what super does. */

    let xlen = cx.data_layout().address_size().bits();

    for arg in fn_abi.args.iter_mut() {
        if arg.is_ignore() {
            continue;
        }

        // LLVM integers types do not differentiate between signed or unsigned integers.
        // Some RISC-V instructions do not have a `.w` suffix version, they use all the
        // XLEN bits. By explicitly setting the `signext` or `zeroext` attribute
        // according to signedness to avoid unnecessary integer extending instructions.
        //
        // See https://github.com/rust-lang/rust/issues/114508 for details.
        super::extend_integer_width(arg, xlen);
    }

    return;
}

fn compute_purecap_abi_info_inner<'a, Ty, C>(cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    let flen = match &cx.target_spec().llvm_abiname {
        LlvmAbi::Ilp32f | LlvmAbi::Lp64f => 32,
        LlvmAbi::Ilp32d | LlvmAbi::Lp64d => 64,
        _ => 0,
    };
    let xlen = cx.data_layout().address_size().bits();
    let ylen = cx.data_layout().pointer_size().bits();

    let mut avail_gprs = 8;
    let mut avail_fprs = 8;

    if !fn_abi.ret.is_ignore() {
        if classify_purecap_ret(cx, &mut fn_abi.ret, xlen, ylen, flen, &mut avail_gprs)
            == PureCapAbiResult::Fallback
        {
            if super::classify_ret(cx, &mut fn_abi.ret, xlen, flen) {
                avail_gprs -= 1;
            }
        }
    }

    for (i, arg) in fn_abi.args.iter_mut().enumerate() {
        if arg.is_ignore() {
            continue;
        }
        let is_vararg = fn_abi.c_variadic && (i >= fn_abi.fixed_count as usize);
        if classify_purecap_arg(
            cx,
            arg,
            xlen,
            ylen,
            flen,
            is_vararg,
            &mut avail_gprs,
            &mut avail_fprs,
        ) == PureCapAbiResult::Fallback
        {
            super::classify_arg(cx, arg, xlen, flen, is_vararg, &mut avail_gprs, &mut avail_fprs);
        }
    }
}

fn classify_purecap_ret<'a, Ty, C>(
    cx: &C,
    arg: &mut ArgAbi<'a, Ty>,
    xlen: u64,
    ylen: u64,
    flen: u64,
    avail_gprs: &mut u64,
) -> PureCapAbiResult
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    let mut avail_fprs = 0;
    classify_purecap_arg(cx, arg, xlen, ylen, flen, false, avail_gprs, &mut avail_fprs)
}

fn classify_purecap_arg<'a, Ty, C>(
    cx: &C,
    arg: &mut ArgAbi<'a, Ty>,
    _xlen: u64,
    ylen: u64,
    _flen: u64,
    is_vararg: bool,
    avail_gprs: &mut u64,
    _avail_fprs: &mut u64,
) -> PureCapAbiResult
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout + HasTargetSpec,
{
    // Not a CHERI target.
    if ylen == 0 {
        return PureCapAbiResult::Fallback;
    }

    if arg.is_ignore() {
        return PureCapAbiResult::Fallback;
    }

    if *avail_gprs == 0 {
        arg.make_indirect();
        return PureCapAbiResult::Done;
    }

    let arg_size = arg.layout.size;
    let arg_alignment = arg.layout.align.abi;
    let cap_reg = Reg::ptr(cx);

    // "In the pure-capability ABIs, the handling of variadic arguments is different.
    // All variadic arguments are passed on the stack, and a capability to them is passed in the x5 (t0) register.
    // See RVY Pure-Capability Calling Conventions for details."
    if is_vararg {
        arg.make_indirect();
        return PureCapAbiResult::Done;
    }

    // "YLEN wide scalars are passed in a single argument register if they are of pointer type (including (u)intptr_t)."
    if arg_size.bits() == ylen {
        if let rustc_abi::BackendRepr::Scalar(s) = arg.layout.backend_repr
            && let Primitive::Pointer(..) = s.primitive()
        {
            /* Do nothing, use the default direct mode. */
            *avail_gprs = avail_gprs.saturating_sub(1);
            return PureCapAbiResult::Done;
        }
    }

    // "Aggregates up to 2*YLEN bits are passed in YLEN argument register(s) (using the memory layout of the aggregate)
    // if they are at least YLEN-bit aligned. All less-aligned aggregates follow rules of the underlying base integer
    // or float calling convention (ILP32*/LP64* ABIs)."
    if (arg.layout.is_aggregate()
        //`.is_aggregate()` says that things that are represented as a single scalar can't be aggregates.
        // This probably makes sense for other bits of the compiler, but here we want to pick up types like `{ i64 }`,
        // which has the backend repr of a single scalar.
        || !matches!(arg.layout.fields, rustc_abi::FieldsShape::Primitive))
        && arg_size.bits() <= 2 * ylen
        && arg_alignment.bits() % ylen == 0
    {
        let num_regs = arg_size.bits() / ylen;

        match arg.layout.backend_repr {
            rustc_abi::BackendRepr::Scalar { .. }
            | rustc_abi::BackendRepr::ScalarPair { .. }
            | rustc_abi::BackendRepr::Memory { .. } => {
                if num_regs == 1 {
                    arg.cast_to(cap_reg);
                    *avail_gprs = avail_gprs.saturating_sub(1);
                } else if *avail_gprs >= 2 {
                    arg.cast_to(CastTarget::pair(cap_reg, cap_reg));
                    *avail_gprs = avail_gprs.saturating_sub(2);
                } else {
                    arg.make_indirect();
                }
                return PureCapAbiResult::Done;
            }
            _ => {}
        }
    }

    PureCapAbiResult::Fallback
}
