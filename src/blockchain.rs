use crate::block::Block;
use crate::transaction::Transaction;
use log::{info, error};

/// Represents the blockchain, a distributed ledger composed of sequential blocks.
///
/// Each block contains transactions and references the previous block's hash, ensuring
/// immutability and consistency across the chain.
///
/// # Example
/// ```
/// use your_crate::Blockchain;
/// use your_crate::Transaction;
///
/// let mut blockchain = Blockchain::new();
/// let tx = Transaction::new("Send 5 BTC to Alice".to_string());
/// blockchain.add_create_block(vec![tx]);
/// assert!(blockchain.is_valid());
/// ```
#[derive(Debug)]
pub struct Blockchain {
    /// The list of blocks forming the blockchain.
    blocks: Vec<Block>,
}

impl Blockchain {
    /// Initializes a new blockchain with a genesis block.
    ///
    /// # Returns
    ///
    /// A `Blockchain` instance containing the genesis block.
    ///
    /// # Example
    /// ```
    /// let blockchain = Blockchain::new();
    /// assert_eq!(blockchain.get_blocks().len(), 1); // Only genesis block
    /// ```
    pub fn new() -> Self {
        let coinbase_tx = Transaction::new("Genesis Block".to_string());
        let genesis_block = Block::generate_genesis_block(&coinbase_tx);

        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    /// Creates a blockchain from an existing list of blocks.
    ///
    /// Useful for loading chains from storage or during synchronization.
    ///
    /// # Arguments
    ///
    /// * `data` - A `Vec<Block>` representing the chain.
    ///
    /// # Returns
    ///
    /// A `Blockchain` initialized with the given blocks.
    pub fn from_blocks(data: Vec<Block>) -> Self {
        Blockchain { blocks: data }
    }

    /// Adds a block to the blockchain after validating its integrity.
    ///
    /// This method checks:
    /// - The previous block hash matches.
    /// - The Proof of Work is valid.
    ///
    /// # Arguments
    ///
    /// * `block` - The new block to be added.
    ///
    /// # Returns
    ///
    /// `true` if the block was successfully added, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let tx = Transaction::new("Send 10 BTC".to_string());
    /// let new_block = Block::new_block("prev_hash".to_string(), &[tx], 1);
    /// blockchain.add_block(new_block);
    /// ```
    pub fn add_block(&mut self, block: Block) -> bool {
        if let Some(last_block) = self.get_last_block() {
            // Validate previous block hash
            if block.get_pre_block_hash() != last_block.get_hash() {
                error!("Block rejected: Invalid previous hash.");
                return false;
            }

            // Validate Proof of Work
            let pow = crate::proof_of_work::ProofOfWork::new_proof_of_work(block.clone());
            let (_, calculated_hash) = pow.run();

            if block.get_hash() != calculated_hash {
                error!("Block rejected: Invalid proof of work.");
                return false;
            }

            self.blocks.push(block);
            info!("Block successfully added.");
            true
        } else {
            error!("Blockchain is empty. Cannot add block.");
            false
        }
    }

    /// Creates a new block with the given transactions and adds it to the chain.
    ///
    /// # Arguments
    ///
    /// * `transactions` - A vector of `Transaction` instances to include in the block.
    pub fn add_create_block(&mut self, transactions: Vec<Transaction>) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new_block(
            prev_block.get_hash().to_string(),
            &transactions,
            prev_block.get_height() + 1,
        );
        self.blocks.push(new_block);
    }

    /// Retrieves the entire blockchain as a vector of blocks.
    ///
    /// # Returns
    ///
    /// A reference to the list of blocks.
    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    /// Retrieves the last block in the blockchain.
    ///
    /// # Returns
    ///
    /// An `Option<&Block>` containing the last block if it exists.
    pub fn get_last_block(&self) -> Option<&Block> {
        self.blocks.last()
    }

    /// Validates the entire blockchain to ensure its integrity.
    ///
    /// This method checks that:
    /// - Each block references the correct previous hash.
    /// - Proof of Work is valid for each block.
    ///
    /// # Returns
    ///
    /// `true` if the entire chain is valid, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let blockchain = Blockchain::new();
    /// assert!(blockchain.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.get_pre_block_hash() != previous.get_hash() {
                return false;
            }

            let pow = crate::proof_of_work::ProofOfWork::new_proof_of_work(current.clone());
            let (_, calculated_hash) = pow.run();
            if current.get_hash() != calculated_hash {
                return false;
            }
        }
        true
    }

    /// Returns an iterator over the blocks in the blockchain.
    pub fn iter(&self) -> std::slice::Iter<Block> {
        self.blocks.iter()
    }
}
