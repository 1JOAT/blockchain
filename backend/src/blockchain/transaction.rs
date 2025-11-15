use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: DateTime<Utc>,
    pub signature: String, // Placeholder for digital signature
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            sender,
            receiver,
            amount,
            timestamp: Utc::now(),
            signature: String::new(), // Mock unsigned
        }
    }

    pub fn calculate_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}{}",
            self.sender,
            self.receiver,
            self.amount,
            self.timestamp.timestamp(),
            self.signature
        ));
        format!("{:x}", hasher.finalize())
    }
}
