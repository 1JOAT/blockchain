use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use super::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now();
        let mut block = Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn genesis() -> Self {
        Self::new(0, vec![], String::from("0"))
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let tx_hashes: String = self.transactions.iter()
            .map(|tx| tx.calculate_hash())
            .collect::<Vec<_>>()
            .join("");

        hasher.update(format!("{}{}{}{}{}{}",
            self.index,
            self.timestamp.timestamp(),
            tx_hashes,
            self.previous_hash,
            self.nonce,
            "" // root placeholder
        ));
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        use super::proof_of_work::ProofOfWork;
        let mut pow = ProofOfWork::new(self.clone(), difficulty);
        let (hash, nonce) = pow.run();
        self.hash = hash;
        self.nonce = nonce;
    }
}
