use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProduct {
    pub items: String,
}
