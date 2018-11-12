use std::net::IpAddr;

pub fn is_ip(ip: &str) -> bool {
    !ip.parse::<IpAddr>().is_err()
}

#[test]
fn test_ip() {
    let ip = "0.0.0.0";
    let bool_is_ip = ::common::is_ip(&ip);
    println!("{},{:?}", ip, bool_is_ip);
}
