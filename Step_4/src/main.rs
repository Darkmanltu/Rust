enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


// rust does nto have null so we have to encode like this
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    println!("Hello, world!");
}
