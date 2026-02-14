use axum::{
    extract::State,
    response::Json,
};
use crate::api::{AppState, ApiResponse, TransactionRequest};
use crate::blockchain::Transaction;
use crate::errors::{ApiResult, ApiError};

pub async fn add_transaction(
    State(state): State<AppState>,
    Json(tx_req): Json<TransactionRequest>,
) -> ApiResult<Json<ApiResponse<String>>> {
    let mut blockchain = state.blockchain.lock().await;
    let transaction = Transaction::new(tx_req.sender, tx_req.receiver, tx_req.amount);

    match blockchain.add_transaction(transaction) {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: Some("Transaction added to pending transactions".to_string()),
            error: None,
        })),
        Err(e) => Err(ApiError::TransactionError(e)),
    }
}
