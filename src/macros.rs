#[macro_export]
/// construct Ipv4Addr or Ipv6Addr, depending on the path
macro_rules! addr {
    (($a:literal, $b:literal, $c:literal, $d:literal):$port:literal) => {{
        use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new($a, $b, $c, $d), $port))
    }};
    (($a:literal, $b:literal, $c:literal, $d:literal, $e:literal, $f:literal, $g:literal, $h:literal) : $port:literal) => {{
        use std::net::{Ipv6Addr, SocketAddr, SocketAddrV6};
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new($a, $b, $c, $d, $e, $f, $g, $h),
            $port,
            0,
            0,
        ))
    }};
}

#[macro_export]
/// Call [$call], if it returns false, return InternalServerError
macro_rules! or500 {
    ($call:expr) => {
        if !$call {
            return $crate::Status::InternalServerError;
        }
    };
}
