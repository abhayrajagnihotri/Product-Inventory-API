use serde::{Serialize, Deserialize};
use uuid::Uuid;

.
#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
}


#[derive(Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
}
