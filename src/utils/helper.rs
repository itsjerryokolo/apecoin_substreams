pub fn append_0x(i: &str) -> String {
    format!("0x{}", i)
}
pub fn generate_id(tx_hash: &str, log_index: &str) -> String {
    format!("{}-{}", tx_hash, log_index)
}
