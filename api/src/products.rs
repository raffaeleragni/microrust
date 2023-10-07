use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use axum_macros::debug_handler;
use contracts::prelude::*;
use kafka::producer::Record;

use crate::AppState;

#[tracing::instrument]
pub async fn get_producs() -> Response {
    let response = Vec::<Product>::new();
    Json(response).into_response()
}

#[debug_handler]
pub async fn new_product(State(state): State<AppState>, Json(product): Json<CreateProduct>) {
    let id = sqlx::query!("insert into product (items) values (?)", &product.items)
        .execute(&state.database)
        .await
        .unwrap()
        .last_insert_id();
    let msg = bincode::serialize(&Product {
        id: id as i64,
        items: product.items,
    })
    .unwrap();
    state
        .producer
        .lock()
        .unwrap()
        .send(&Record::from_value("products", msg))
        .unwrap();
}
