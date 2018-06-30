#![allow(unused_variables)]
fn main() {
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let dom = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
