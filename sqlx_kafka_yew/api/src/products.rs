use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use contracts::prelude::*;
use kafka::producer::Record;
use serde::Deserialize;
use tracing::instrument;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Response<T> = std::result::Result<T, StatusCode>;

#[derive(Deserialize, Debug)]
pub struct Params {
    pub id: i64,
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn get_products(State(state): State<AppState>) -> Response<Json<Vec<Product>>> {
    response_from_result(get_products_fn(state).await.map(|it| Json(it)))
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn new_product(
    State(state): State<AppState>,
    Json(product): Json<CreateProduct>,
) -> Response<()> {
    response_from_result(new_product_fn(state, product).await)
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn replace_product(
    State(state): State<AppState>,
    Query(params): Query<Params>,
    Json(product): Json<Product>,
) -> Response<()> {
    response_from_result(replace_product_fn(state, params.id, product).await)
}

#[instrument(skip(state))]
#[debug_handler]
pub async fn delete_product(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> Response<()> {
    let _ = sqlx::query!("delete from product where id = ?", params.id)
        .execute(&state.database)
        .await;
    response_from_result(Ok(()))
}

fn response_from_result<T>(result: Result<T>) -> Response<T> {
    match result {
        Ok(response) => Ok(response),
        Err(err) => {
            log::error!(target = "api"; "{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn get_products_fn(state: AppState) -> Result<Vec<Product>> {
    let res = sqlx::query_as!(Product, "select * from product")
        .fetch_all(&state.database)
        .await?;
    Ok(res)
}

async fn new_product_fn(state: AppState, product: CreateProduct) -> Result<()> {
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

async fn replace_product_fn(state: AppState, id: i64, product: Product) -> Result<()> {
    sqlx::query!(
        "update product set items = ? where id = ?",
        &product.items,
        id
    )
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

