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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert!(genesis.transactions.is_empty());
    }

    #[test]
    fn test_calculate_hash() {
        let block = Block::new(1, vec![], "prev_hash".to_string());
        let hash1 = block.calculate_hash();
        let hash2 = block.calculate_hash();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_unique_hash_with_transactions() {
        let tx1 = Transaction::new("a".to_string(), "b".to_string(), 10);
        let tx2 = Transaction::new("a".to_string(), "b".to_string(), 20);
        
        let block1 = Block::new(1, vec![tx1], "prev".to_string());
        let block2 = Block::new(1, vec![tx2], "prev".to_string());
        
        assert_ne!(block1.hash, block2.hash);
    }
}
