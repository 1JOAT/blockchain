use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::blockchain::Blockchain;
use crate::handlers::{block, transaction, node};

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
        .route("/blocks", get(block::get_blocks))
        .route("/block/{index}", get(block::get_block))
        .route("/transaction", post(transaction::add_transaction))
        .route("/mine", post(block::mine_block))
        .route("/balance/{address}", get(node::get_balance))
        .route("/chain/valid", get(node::validate_chain))
}
