use std::error::Error;
use std::fs::File;

fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1".parse().unwrap();

    dbg!(home);
}
