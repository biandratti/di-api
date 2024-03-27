use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};

use fingerprint_api::run;

#[tokio::main]
async fn main() {
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".env.{}", e);
    } else {
        environment_file = String::from(".env");
    }
    dotenv::from_filename(environment_file).ok();

    let server_port = dotenv::var("SERVER_PORT").expect("SERVER_PORT must be set").parse::<u16>().unwrap();
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), server_port);
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    run(socket_addr, &database_url).await;
}
