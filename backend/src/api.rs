use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::blockchain::{Blockchain, Transaction};

#[derive(Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<Blockchain>>,
}

#[derive(Deserialize)]
pub struct TransactionRequest {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/blocks", get(get_blocks))
        .route("/block/{index}", get(get_block))
        .route("/transaction", post(add_transaction))
        .route("/mine", post(mine_block))
        .route("/balance/{address}", get(get_balance))
        .route("/chain/valid", get(validate_chain))
}

async fn get_blocks(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let blockchain = state.blockchain.lock().await;
    let blocks: Vec<serde_json::Value> = blockchain
        .chain
        .iter()
        .map(|block| serde_json::to_value(block).unwrap())
        .collect();

    Json(ApiResponse {
        success: true,
        data: Some(blocks),
        error: None,
    })
}

async fn get_block(
    Path(index): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let blockchain = state.blockchain.lock().await;

    if let Some(block) = blockchain.chain.get(index as usize) {
        Ok(Json(ApiResponse {
            success: true,
            data: Some(serde_json::to_value(block).unwrap()),
            error: None,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn add_transaction(
    State(state): State<AppState>,
    Json(tx_req): Json<TransactionRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let mut blockchain = state.blockchain.lock().await;
    let transaction = Transaction::new(tx_req.sender, tx_req.receiver, tx_req.amount);

    match blockchain.add_transaction(transaction) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Transaction added to pending transactions".to_string()),
            error: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e),
        })),
    }
}

async fn mine_block(
    State(state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut blockchain = state.blockchain.lock().await;
    let miner_address = "miner-address".to_string(); 
    let new_block = blockchain.mine_pending_transactions(miner_address);

    // Save blockchain to file after mining
    if let Err(e) = blockchain.save_to_file("blockchain.json") {
        eprintln!("Failed to save blockchain: {}", e);
    }

    Json(ApiResponse {
        success: true,
        data: Some(serde_json::to_value(&new_block).unwrap()),
        error: None,
    })
}

async fn get_balance(
    Path(address): Path<String>,
    State(state): State<AppState>,
) -> Json<ApiResponse<u64>> {
    let blockchain = state.blockchain.lock().await;
    let balance = blockchain.get_balance(&address);

    Json(ApiResponse {
        success: true,
        data: Some(balance),
        error: None,
    })
}

async fn validate_chain(
    State(state): State<AppState>,
) -> Json<ApiResponse<bool>> {
    let blockchain = state.blockchain.lock().await;
    let is_valid = blockchain.is_chain_valid();

    Json(ApiResponse {
        success: true,
        data: Some(is_valid),
        error: None,
    })
}
