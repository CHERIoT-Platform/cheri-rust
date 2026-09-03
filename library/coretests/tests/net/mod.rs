use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[cfg(any(not(target_abi = "cheriot"), feature = "test_net_ip_addr"))]
mod ip_addr;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_net_parser"))]
mod parser;
#[cfg(any(not(target_abi = "cheriot"), feature = "test_net_socket_addr"))]
mod socket_addr;

pub fn sa4(a: Ipv4Addr, p: u16) -> SocketAddr {
    SocketAddr::V4(SocketAddrV4::new(a, p))
}

pub fn sa6(a: Ipv6Addr, p: u16) -> SocketAddr {
    SocketAddr::V6(SocketAddrV6::new(a, p, 0, 0))
}
