use axum::{
    extract::{Path, State},
    response::Json,
};
use crate::api::{AppState, ApiResponse};
use crate::errors::ApiResult;

pub async fn get_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<u64>>> {
    let blockchain = state.blockchain.lock().await;
    let balance = blockchain.get_balance(&address);

    Ok(Json(ApiResponse {
        success: true,
        data: Some(balance),
        error: None,
    }))
}

pub async fn validate_chain(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<bool>>> {
    let blockchain = state.blockchain.lock().await;
    let is_valid = blockchain.is_chain_valid();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(is_valid),
        error: None,
    }))
}
