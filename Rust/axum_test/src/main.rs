use axum::{
    extract::Path,
    routing::get,
    Router,
};

async fn test() -> &'static str {
    "Hello, World! from AXUM !"
}

async fn test_name(Path(name): Path<String>) -> String {
    format!("Hello {}",name)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
            .route("/", get( test))
            .route("/:name", get(test_name));

    // run it with hyper on localhost:3000
    println!("running with hyper on http://127.0.0.1:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}