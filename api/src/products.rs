use std::error::Error;

use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use contracts::prelude::*;
use kafka::producer::Record;
use tracing::instrument;

use crate::AppState;

#[instrument(skip(state))]
#[debug_handler]
pub async fn get_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, StatusCode> {
    match get_products_fn(state).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => {
            log::error!(target = "api", route = "get_products"; "{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_products_fn(state: AppState) -> Result<Vec<Product>, Box<dyn Error>> {
    let res = sqlx::query_as!(Product, "select * from product")
        .fetch_all(&state.database)
        .await?;
    Ok(res)
}

#[instrument(skip(state))]
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
    let msg = AvroProduct {
        id: id as i64,
        items: product.items,
    };
    let key = bincode::serialize(&id)?;
    let value = bincode::serialize(&msg)?;
    let record = Record::from_key_value("products", key, value);
    state.producer.lock().unwrap().send(&record)?;

    Ok(())
}
