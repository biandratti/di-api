use std::net::SocketAddrV4;

mod adapters;
mod application;
mod domain;
mod infrastructure;

pub async fn run(socket_addr: SocketAddrV4, db_url: &str) {
    infrastructure::server::run(socket_addr, db_url).await;
}
