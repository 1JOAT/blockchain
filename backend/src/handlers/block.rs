use axum::{
    extract::{Path, State},
    response::Json,
};
use serde_json::Value;
use crate::api::{AppState, ApiResponse};
use crate::errors::{ApiResult, ApiError};

pub async fn get_blocks(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Vec<Value>>>> {
    let blockchain = state.blockchain.lock().await;
    let blocks: Vec<Value> = blockchain
        .chain
        .iter()
        .map(|block| serde_json::to_value(block).unwrap())
        .collect();

    Ok(Json(ApiResponse {
        success: true,
        data: Some(blocks),
        error: None,
    }))
}

pub async fn get_block(
    Path(index): Path<u64>,
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Value>>> {
    let blockchain = state.blockchain.lock().await;

    if let Some(block) = blockchain.chain.get(index as usize) {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(serde_json::to_value(block).unwrap()),
            error: None,
        }))
    } else {
        Err(ApiError::NotFound(format!("Block at index {} not found", index)))
    }
}

pub async fn mine_block(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Value>>> {
    let mut blockchain = state.blockchain.lock().await;
    let miner_address = "miner-address".to_string(); 
    let new_block = blockchain.mine_pending_transactions(miner_address);

    if let Err(e) = blockchain.save_to_file("blockchain.json") {
        return Err(ApiError::StorageError(format!("Failed to save blockchain: {}", e)));
    }

    Ok(Json(ApiResponse {
        success: true,
        data: Some(serde_json::to_value(&new_block).unwrap()),
        error: None,
    }))
}
