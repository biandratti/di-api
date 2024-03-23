use std::net::SocketAddrV4;

pub mod adapters;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub async fn run(socket_addr: SocketAddrV4, db_url: &str) {
    infrastructure::server::run(socket_addr, db_url).await;
}
