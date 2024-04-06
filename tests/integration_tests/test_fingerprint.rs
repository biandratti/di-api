use http::header::AUTHORIZATION;
use serde_json::from_slice;
use testcontainers::{clients, core::WaitFor, GenericImage};
use tokio::test;
use warp::http::{HeaderValue, Response, StatusCode};
use warp::hyper::body::Bytes;
use warp::test::request;

use fingerprint_api::adapters::api::fingerprint::fingerprint_controllers;
use fingerprint_api::adapters::api::fingerprint::fingerprint_payload::FingerprintPayload;
use fingerprint_api::adapters::api::fingerprint::fingerprint_presenters::FingerprintPresenter;
use fingerprint_api::adapters::spi::db::fingerprint_repository::MongoFingerprintRepository;
use fingerprint_api::infrastructure::mongo::MongoClient;

#[test]
async fn it_get_fingerprints() {
    let (mongo_image, db_name, exposed_port) = setup_environment().await;
    let docker = clients::Cli::default();
    let mongo_db = docker.run(mongo_image);
    let port = mongo_db.get_host_port_ipv4(exposed_port);
    let db_url = format!("mongodb://root:root@localhost:{}", port);

    let mongo_client: MongoClient = MongoClient::new(&db_url).await.unwrap();
    let repo: MongoFingerprintRepository = MongoFingerprintRepository::new(mongo_client.client, &db_name).await.unwrap();

    let auth_header_value = format!("Basic {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, "username:password"));

    let api = fingerprint_controllers::build(repo);

    let post_resp: Response<Bytes> = request()
        .method("POST")
        .path("/fingerprint")
        .json(&fingerprint_payload())
        .header(AUTHORIZATION.as_str(), HeaderValue::from_str(&auth_header_value).unwrap())
        .reply(&api)
        .await;

    assert_eq!(post_resp.status(), StatusCode::CREATED);

    let get_resp: Response<Bytes> = request()
        .method("GET")
        .path("/fingerprint")
        .header(AUTHORIZATION.as_str(), HeaderValue::from_str(&auth_header_value).unwrap())
        .reply(&api)
        .await;

    let status = get_resp.status();
    let body_bytes = get_resp.into_body().clone();

    let fingerprints: Vec<FingerprintPresenter> = from_slice(&body_bytes).unwrap();

    assert_eq!(status, StatusCode::OK);
    assert_eq!(fingerprints.len(), 1);
    assert_eq!(fingerprints[0].trace_id, "my_trace_id");
}

fn fingerprint_payload() -> FingerprintPayload {
    FingerprintPayload {
        trace_id: String::from("my_trace_id"),
        ip: None,
    }
}

async fn setup_environment() -> (GenericImage, String, u16) {
    dotenv::from_filename(".env.test").ok();
    let db_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let msg = WaitFor::message_on_stdout("server is ready");
    let exposed_port: u16 = 27017;
    let mongo_image: GenericImage = GenericImage::new("mongo", "7.0.7")
        .with_wait_for(msg.clone())
        .with_env_var("MONGO_INITDB_DATABASE", &db_name)
        .with_env_var("MONGO_INITDB_ROOT_USERNAME", "root")
        .with_env_var("MONGO_INITDB_ROOT_PASSWORD", "root")
        .with_exposed_port(exposed_port);

    (mongo_image, db_name, exposed_port)
}
