/// Keccak256 Hash Function
/// Isolates work with Solana's hash function
pub trait HashAdapter {
    fn hash(input: &[u8]) -> [u8; 32];
}

/// Allows you to call [`solana_program::hash::hash`] via [`HashAdapter`] trait
pub struct SolanaKeccak256;

impl HashAdapter for SolanaKeccak256 {
    fn hash(input: &[u8]) -> [u8; 32] {
        solana_program::hash::hash(input).to_bytes()
    }
}

#[cfg(not(target_arch = "bpf"))]
impl HashAdapter for sha3::Keccak256 {
    fn hash(input: &[u8]) -> [u8; 32] {
        use sha3::Digest;
        sha3::Keccak256::digest(input).try_into().unwrap()
    }
}
