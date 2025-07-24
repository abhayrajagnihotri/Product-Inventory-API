use axum::{
    
    routing::{get, put},
    Router,
};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use uuid::Uuid;
use tokio::net::TcpListener;


mod handlers;
mod models;
mod product_handlers;



use product_handlers::{
    get_products, create_product, update_product, delete_product, AppState
};

#[tokio::main]
async fn main() {
    let db_inner: Arc<Mutex<HashMap<Uuid, models::Product>>> = Arc::new(Mutex::new(HashMap::new()));
    let app_state = AppState(db_inner);

    let app = Router::new()
        .route("/products", get(get_products).post(create_product))
        .route("/products/{id}", put(update_product).delete(delete_product))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("✅ Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
