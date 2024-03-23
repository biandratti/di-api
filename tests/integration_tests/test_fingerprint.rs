use testcontainers::{clients, core::WaitFor, GenericImage};
use tokio::test;
use warp::http::StatusCode;
use warp::test::request;

use fingerprint_api::adapters::api::fingerprint::fingerprint_controllers;
use fingerprint_api::adapters::api::fingerprint::fingerprint_payload::FingerprintPayload;
use fingerprint_api::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use fingerprint_api::infrastructure::mongo::MongoClient;

#[test]
async fn it_get_fingerprints() {
    dotenv::from_filename(".env.test").ok();
    let db_name = &dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let docker = clients::Cli::default();
    let msg = WaitFor::message_on_stdout("server is ready");
    let port: u16 = 27017;
    let generic = GenericImage::new("mongo", "6.0.7")
        .with_wait_for(msg.clone())
        .with_env_var("MONGO_INITDB_DATABASE", db_name)
        .with_env_var("MONGO_INITDB_ROOT_USERNAME", "root")
        .with_env_var("MONGO_INITDB_ROOT_PASSWORD", "root")
        .with_exposed_port(port);

    let node = docker.run(generic);
    let port = node.get_host_port_ipv4(port);

    let db_url = format!("mongodb://root:root@localhost:{}", port);
    let mongo_client: MongoClient = MongoClient::new(&db_url).await.unwrap();
    let repo: MongoFingerprintRepository =
        MongoFingerprintRepository::new(mongo_client.client, &db_name)
            .await
            .unwrap();

    let api = fingerprint_controllers::build(repo);

    let resp = request()
        .method("POST")
        .path("/fingerprint")
        .json(&fingerprint_payload())
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}

fn fingerprint_payload() -> FingerprintPayload {
    FingerprintPayload {
        trace_id: String::from("my_trace_id"),
        ip: None,
    }
}
