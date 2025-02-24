use crate::{proof_of_work, transaction::Transaction};
use serde::{Deserialize, Serialize};
use log::info;

/// Represents a block in the blockchain.
///
/// A block contains metadata such as a timestamp, a reference to the previous block's hash,
/// a list of transactions, a nonce (used in Proof of Work), and the block's height
/// in the blockchain.
///
/// # Example
///
/// ```
/// use your_crate::Block;
/// use your_crate::Transaction;
///
/// let tx = Transaction::new("Send 10 BTC to Alice".to_string());
/// let genesis_block = Block::generate_genesis_block(&tx);
/// println!("Genesis Block Hash: {}", genesis_block.get_hash());
/// ```
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    /// The timestamp indicating when the block was created.
    timestamp: i64,

    /// The hash of the previous block in the chain.
    pre_block_hash: String,

    /// The hash of the current block (computed via Proof of Work).
    hash: String,

    /// A list of transactions included in this block.
    transactions: Vec<Transaction>,

    /// The nonce found by the Proof of Work algorithm.
    nonce: i64,

    /// The height of the block in the blockchain.
    height: usize,
}

impl Block {
    /// Creates a new block and runs Proof of Work to compute its hash.
    ///
    /// # Arguments
    ///
    /// * `pre_block_hash` - The hash of the previous block.
    /// * `transactions` - A slice of transactions to include in the block.
    /// * `height` - The height of the block in the blockchain.
    ///
    /// # Returns
    ///
    /// A new `Block` with a valid Proof of Work.
    ///
    /// # Example
    /// ```
    /// let tx = Transaction::new("Send 5 BTC".to_string());
    /// let block = Block::new_block("prev_hash".to_string(), &[tx], 1);
    /// println!("New Block Hash: {}", block.get_hash());
    /// ```
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::util::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };

        // Run Proof of Work to find a valid nonce and compute the block hash
        info!("Start Mining!");
        let pow = proof_of_work::ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        info!("Blocked mined successfully!");
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    /// Generates the genesis block (the first block in the blockchain).
    ///
    /// # Arguments
    ///
    /// * `transaction` - The initial transaction to include in the genesis block.
    ///
    /// # Returns
    ///
    /// A genesis `Block` with height 0.
    ///
    /// # Example
    /// ```
    /// let tx = Transaction::new("Genesis Transaction".to_string());
    /// let genesis = Block::generate_genesis_block(&tx);
    /// assert_eq!(genesis.get_height(), 0);
    /// ```
    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(String::from("None"), &transactions, 0)
    }

    /// Computes the combined hash of all transactions in the block.
    ///
    /// This is typically used as part of the Proof of Work process.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` representing the SHA-256 digest of all transaction IDs.
    ///
    /// # Example
    /// ```
    /// let tx = Transaction::new("Sample Data".to_string());
    /// let block = Block::generate_genesis_block(&tx);
    /// let tx_hash = block.hash_transactions();
    /// println!("Transactions Hash: {:?}", tx_hash);
    /// ```
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::util::sha256_digest(txhashs.as_slice())
    }

    /// Returns a reference to the list of transactions in the block.
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    /// Returns the hash of the previous block.
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    /// Returns the hash of the current block.
    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    /// Returns the timestamp when the block was created.
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    /// Returns the height of the block in the blockchain.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Returns the nonce used to solve the Proof of Work.
    pub fn get_nonce(&self) -> i64 {
        self.nonce
    }
}
