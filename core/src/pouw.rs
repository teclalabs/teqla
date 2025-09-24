use sha3::{Digest, Sha3_256};
use hex;

/// Compute PoUW hash over a small blob slice + nonce + suffix
pub fn compute_pouw(blob: &[u8], nonce: u64, suffix: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(blob);
    hasher.update(&nonce.to_le_bytes());
    hasher.update(suffix);
    let out = hasher.finalize();
    hex::encode(out)
}

/// Check difficulty target by counting leading zero bits (placeholder)
pub fn meets_difficulty(_hash_hex: &str, _target_zero_bits: u8) -> bool {
    // TODO: implement bit-level difficulty check
    true
}
