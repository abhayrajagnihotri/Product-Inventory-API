use axum::{
    http::StatusCode,
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::models::{Product, CreateProduct};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use axum::extract::FromRef;

// Define ProductDb *first* as the concrete type for our in-memory store.
pub type ProductDb = Arc<Mutex<HashMap<Uuid, Product>>>;

// Then, define AppState as the struct that wraps our ProductDb.
// This AppState is what will be passed as the router's overall state.
#[derive(Clone)] // AppState needs to be Clone because the router's state requires it.
pub struct AppState(pub ProductDb);

// Implement FromRef for ProductDb, allowing it to be extracted from AppState.
// This tells Axum: "If the overall application state (S) is `AppState`,
// here's how you can provide a `ProductDb` (T) to a handler that requests it."
impl FromRef<AppState> for ProductDb {
    fn from_ref(state: &AppState) -> ProductDb {
        // We access the inner ProductDb through the tuple struct's .0 field.
        state.0.clone()
    }
}

// Handlers are correctly defined to request `State<ProductDb>`.
// Axum will now use the `FromRef` implementation above to fulfill this request.

pub async fn get_products(State(db): State<ProductDb>) -> Json<Vec<Product>> {
    let db = db.lock().unwrap();
    let products = db.values().cloned().collect();
    Json(products)
}

pub async fn create_product(
    State(db): State<ProductDb>,
    Json(payload): Json<CreateProduct>,
) -> (StatusCode, Json<Product>) {
    let mut db = db.lock().unwrap();
    let new_product = Product {
        id: Uuid::new_v4(),
        name: payload.name,
        price: payload.price,
    };
    db.insert(new_product.id, new_product.clone());
    (StatusCode::CREATED, Json(new_product))
}

pub async fn update_product(
    State(db): State<ProductDb>,
    Path(id): Path<Uuid>,
    Json(payload): Json<Product>,
) -> (StatusCode, Json<String>) {
    let mut db = db.lock().unwrap();

    if db.contains_key(&id) {
        let updated_product = Product {
            id,
            name: payload.name,
            price: payload.price,
        };
        db.insert(id, updated_product);
        (StatusCode::OK, Json("Product updated.".to_string()))
    } else {
        (StatusCode::NOT_FOUND, Json("Product not found.".to_string()))
    }
}

pub async fn delete_product(
    State(db): State<ProductDb>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<String>) {
    let mut db = db.lock().unwrap();

    if db.remove(&id).is_some() {
        (StatusCode::OK, Json("Product deleted.".to_string()))
    } else {
        (StatusCode::NOT_FOUND, Json("Product not found.".to_string()))
    }
}
