use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProduct {
    pub items: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: i64,
    pub items: String
}
