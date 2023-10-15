use std::error::Error;

use axum::{extract::{State, Query}, http::StatusCode, Json};
use axum_macros::debug_handler;
use contracts::prelude::*;
use kafka::producer::Record;
use tracing::instrument;
use crate::AppState;
use serde::Deserialize;

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

#[derive(Deserialize, Debug)]
pub struct Params {
    pub id: i64
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn replace_product(
    State(state): State<AppState>,
    Query(params): Query<Params>,
    Json(product): Json<Product>,
) -> Result<(), StatusCode> {
    match replace_product_fn(state, params.id, product).await {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(target = "api", route = "replace_product"; "{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn replace_product_fn(state: AppState, id: i64, product: Product) -> Result<(), Box<dyn Error>> {
    sqlx::query!("update product set items = ? where id = ?", &product.items, id)
        .execute(&state.database)
        .await?;
    let msg = AvroProduct {
        id: product.id,
        items: product.items,
    };
    let key = bincode::serialize(&id)?;
    let value = bincode::serialize(&msg)?;
    let record = Record::from_key_value("products", key, value);
    state.producer.lock().unwrap().send(&record)?;

    Ok(())
}

pub async fn delete_product(State(state): State<AppState>, Query(params): Query<Params>) -> Result<(), StatusCode> {
     let res = sqlx::query!("delete from product where id = ?", params.id)
        .execute(&state.database)
        .await;
    match res {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!(target = "api", route = "delete_product"; "{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

