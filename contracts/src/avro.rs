
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Product {
    pub id: i64,
    pub items: String,
}
