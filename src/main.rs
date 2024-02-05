mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    let repo: infrastructure::mongo::MongoFingerprintRepository =
        infrastructure::mongo::MongoFingerprintRepository::new("mongodb://localhost:27017")
            .await
            .unwrap();

    let routes = presentation::routes::routes(repo);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
