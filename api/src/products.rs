use axum::{
    response::{IntoResponse, Response},
    Json,
};
use contracts::prelude::*;

pub async fn get_producs() -> Response {
    let response = Vec::<Product>::new();
    Json(response).into_response()
}

#[axum_macros::debug_handler]
pub async fn new_product(Json(product): Json<CreateProduct>) {}
