use ring::digest::{Context, SHA256};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current timestamp in milliseconds since the UNIX epoch.
///
/// This function is commonly used for timestamping blockchain blocks,
/// transactions, or any time-based operations.
///
/// # Returns
///
/// An `i64` representing the number of milliseconds since **January 1, 1970 (UTC)**.
///
/// # Panics
///
/// This function will panic if the system clock is set before the UNIX epoch,
/// which is highly unlikely on modern systems.
///
/// # Example
/// ```
/// use your_crate::current_timestamp;
///
/// let timestamp = current_timestamp();
/// println!("Current Timestamp: {}", timestamp);
/// ```
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

/// Computes the SHA-256 hash of the provided data.
///
/// This function is used for hashing data securely, commonly applied
/// in blockchain applications for hashing transactions, blocks, and other data.
///
/// # Arguments
///
/// * `data` - A byte slice (`&[u8]`) representing the data to hash.
///
/// # Returns
///
/// A `Vec<u8>` containing the SHA-256 digest of the input data.
///
/// # Example
/// ```rust
/// use your_crate::sha256_digest;
///
/// let data = b"Hello, Blockchain!";
/// let hash = sha256_digest(data);
/// println!("SHA-256 Hash: {:x?}", hash);
/// ```
///
/// # Notes
/// - The output is in raw bytes. If you want a hexadecimal string, use formatting like:
/// ```
/// let hex_string = hex::encode(hash);
/// ```
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);
    let digest = context.finish();
    digest.as_ref().to_vec()
}
