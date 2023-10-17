
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AvroProduct {
    pub id: i64,
    pub items: String,
}
