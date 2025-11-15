use crate::blockchain::Block;

pub struct ProofOfWork {
    pub block: Block,
    pub target: String,
}

impl ProofOfWork {
    pub fn new(block: Block, difficulty: usize) -> Self {
        let target = "0".repeat(difficulty);
        Self { block, target }
    }

    pub fn run(&mut self) -> (String, u64) {
        let mut nonce = 0u64;
        let mut hash;
        

        println!("Mining block {}...", self.block.index);

        loop {
            self.block.nonce = nonce;
            hash = self.block.calculate_hash();

            if hash.starts_with(&self.target) {
                println!("Block mined! Nonce: {}, Hash: {}", nonce, &hash[..16]);
                break;
            }

            nonce += 1;

            // Prevent infinite loop in demo
            if nonce > 1_000_000 {
                println!("Mining timeout - reducing difficulty for demo");
                self.target = "0".repeat(self.target.len().saturating_sub(1));
                if self.target.is_empty() {
                    self.target = "0".to_string();
                }
            }
        }

        (hash, nonce)
    }

    #[allow(dead_code)]
    pub fn validate(&self) -> bool {
        let hash = self.block.calculate_hash();
        hash.starts_with(&self.target) && hash == self.block.hash
    }
}
