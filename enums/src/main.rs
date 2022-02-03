enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrEnum {
    V4(String),
    V6(String),
}

enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home2 = IpAddrEnum::V4(String::from("127.0.0.1"));
    let _six2 = IpAddrEnum::V6(String::from("::1"));

    let _home3 = IpAddrEnum2::V4(127, 0, 0, 1);
    let _six3 = IpAddrEnum2::V6(String::from("::1"));
}
