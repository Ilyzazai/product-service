use warp::Filter;        // Warp web framework
use std::env;            // Environment variables
use dotenv;              // Load .env file

#[tokio::main]
async fn main() {
    // Load environment variables from .env (development only)
    dotenv::dotenv().ok();

    // Read PORT from environment or default to 3030
    let port: u16 = env::var("PORT")
        .unwrap_or("3030".to_string())
        .parse()
        .unwrap();

    // Enable CORS
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"]);

    // /products endpoint
    let products = warp::path("products")
        .map(|| {
            warp::reply::json(&vec![
                serde_json::json!({ "id": 1, "name": "Dog Food", "price": 19.99 }),
                serde_json::json!({ "id": 2, "name": "Cat Food", "price": 34.99 }),
                serde_json::json!({ "id": 3, "name": "Bird Seeds", "price": 10.99 }),
            ])
        })
        .with(cors);

    // Start server
    warp::serve(products)
        .run(([0, 0, 0, 0], port))
        .await;
}
