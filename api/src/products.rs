use std::error::Error;

use axum::{
    extract::State,
    http::StatusCode,
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
pub async fn new_product(
    State(state): State<AppState>,
    Json(product): Json<CreateProduct>,
) -> Result<(), StatusCode> {
    match new_product_fn(state, product).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(target = "api", route = "new_product"; "{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn new_product_fn(state: AppState, product: CreateProduct) -> Result<(), Box<dyn Error>> {
    let id = sqlx::query!("insert into product (items) values (?)", &product.items)
        .execute(&state.database)
        .await?
        .last_insert_id();
    let msg = Product {
        id: id as i64,
        items: product.items,
    };
    let key = bincode::serialize(&id)?;
    let value = bincode::serialize(&msg)?;
    let record = Record::from_key_value("products", key, value);
    state.producer.lock().unwrap().send(&record)?;

    Ok(())
}
