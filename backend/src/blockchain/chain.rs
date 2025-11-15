use super::block::Block;
use super::transaction::Transaction;
use crate::storage::FileStore;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        Self::new_with_difficulty(2)
    }

    pub fn new_with_difficulty(difficulty: usize) -> Self {
        let mut blockchain = Self {
            chain: vec![],
            difficulty,
            pending_transactions: vec![],
            mining_reward: 100,
        };
        blockchain.chain.push(Block::genesis());
        blockchain
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), String> {
        if transaction.sender.is_empty() || transaction.receiver.is_empty() {
            return Err("Transaction must include sender and receiver".to_string());
        }

        if transaction.amount <= 0 {
            return Err("Transaction amount must be greater than 0".to_string());
        }

        if !transaction.sender.is_empty() && self.get_balance(&transaction.sender) < transaction.amount {
            return Err("Not enough balance".to_string());
        }

        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) -> Block {
        let reward_tx = Transaction::new(
            String::new(),
            miner_address,
            self.mining_reward,
        );
        self.pending_transactions.push(reward_tx);

        let mut block = Block::new(
            self.chain.len() as u64,
            self.pending_transactions.clone(),
            self.chain.last().unwrap().hash.clone(),
        );

        block.mine_block(self.difficulty);
        self.chain.push(block.clone());
        self.pending_transactions.clear();

        block
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0u64;

        // Add or subtract from confirmed transaction in blocks
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.sender == address {
                    balance = balance.saturating_sub(transaction.amount);
                }
                if transaction.receiver == address {
                    balance = balance.saturating_add(transaction.amount);
                }
            }
        }

        // Subtract pending outgoing transaction
        for pending_tx in &self.pending_transactions {
            if pending_tx.sender == address {
                balance = balance.saturating_sub(pending_tx.amount);
            }
        }

        balance
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }

            if !current.hash.starts_with(&"0".repeat(self.difficulty)) {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    pub fn get_all_transactions(&self) -> Vec<&Transaction> {
        self.chain.iter()
            .flat_map(|block| &block.transactions)
            .collect()
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let store = FileStore::new(file_path.to_string());
        store.save_blockchain(self)
    }

    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let store = FileStore::new(file_path.to_string());
        store.load_blockchain()
    }
}
