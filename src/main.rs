use std::sync::Arc;

use utoipa_swagger_ui::Config;

mod domain;
mod http_utils;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let config = Arc::new(Config::from("/api-doc.json"));

    let repo: infrastructure::mongo::MongoFingerprintRepository =
        infrastructure::mongo::MongoFingerprintRepository::new("mongodb://localhost:27017")
            .await
            .unwrap();

    let routes = presentation::routes::routes_with_swagger(repo, config);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
