use std::fs;
use std::path::Path;
use serde_json;
use crate::blockchain::Blockchain;

pub struct FileStore {
    file_path: String,
}

impl FileStore {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn save_blockchain(&self, blockchain: &Blockchain) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(blockchain)?;
        fs::write(&self.file_path, data)?;
        Ok(())
    }

    pub fn load_blockchain(&self) -> Result<Blockchain, Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Blockchain::new());
        }

        let data = fs::read_to_string(&self.file_path)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }
}
