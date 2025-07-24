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


pub type ProductDb = Arc<Mutex<HashMap<Uuid, Product>>>;


#[derive(Clone)]
pub struct AppState(pub ProductDb);


impl FromRef<AppState> for ProductDb {
    fn from_ref(state: &AppState) -> ProductDb {
        
        state.0.clone()
    }
}



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
