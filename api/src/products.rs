use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use contracts::prelude::*;
use sqlx::MySqlPool;

#[tracing::instrument]
pub async fn get_producs() -> Response {
    let response = Vec::<Product>::new();
    Json(response).into_response()
}

#[axum_macros::debug_handler]
pub async fn new_product(State(pool): State<MySqlPool>, Json(product): Json<CreateProduct>) {
    let id = sqlx::query!("insert into product (items) values (?)", &product.items)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_id();
    let _item = Product {
        id: id as i64,
        items: product.items,
    };
}
