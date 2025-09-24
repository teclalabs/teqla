use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub id: String,
    pub from: String,
    pub to: String,
    pub value: u64,
    pub nonce: u64,
    pub prevs: Vec<String>,   // references to tips
    pub blob_id: String,      // PoUW task identifier
    pub nonce_work: u64,      // nonce for PoUW
    pub pouw: String,         // SHA3 result
    // pub sig_pqc: Vec<u8>,  // TODO: PQC signature
}

/// Compute cumulative weight (placeholder)
pub fn cumulative_weight(_txid: &str) -> u64 {
    // TODO: sum direct + indirect references
    1
}

/// Select random tips (placeholder)
pub fn select_random_tips(_k: usize) -> Vec<String> {
    vec![]
}
