#![deny(deprecated)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use tokio::net::TcpStream;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let mut stream = TcpStream::connect(addr).await.unwrap();

}
