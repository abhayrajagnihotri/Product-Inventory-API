use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Represents the full Product structure, including its unique ID.
#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}

// Represents the data structure for creating a new product.
// Clients will send this, and the server will generate the `id`.
#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
}
