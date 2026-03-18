// blockchain.rs
// Blockchain implementation examples in Rust

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Simplified hash implementation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: &[u8]) -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        Hash(hash_bytes)
    }
    
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    pub fn zero() -> Self {
        Hash([0u8; 32])
    }
}

// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub timestamp: u64,
    pub signature: Option<String>,
}

impl Transaction {
    pub fn new(sender: String, recipient: String, amount: u64) -> Self {
        let id = Self::generate_id(&sender, &recipient, amount);
        
        Transaction {
            id,
            sender,
            recipient,
            amount,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            signature: None,
        }
    }
    
    fn generate_id(sender: &str, recipient: &str, amount: u64) -> String {
        let data = format!("{}{}{}{}", sender, recipient, amount, 
                          std::time::SystemTime::now()
                              .duration_since(std::time::UNIX_EPOCH)
                              .unwrap()
                              .as_nanos());
        Hash::new(data.as_bytes()).as_hex()
    }
}

// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: Hash,
}

impl Block {
    pub fn new(index: u64, previous_hash: Hash, transactions: Vec<Transaction>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            transactions,
            nonce: 0,
            hash: Hash::zero(),
        };
        
        block.mine_block(2); // 2 leading zeros for demo
        block
    }
    
    fn calculate_hash(&self) -> Hash {
        let data = format!("{}{}{}{}{}",
                          self.index,
                          self.timestamp,
                          self.previous_hash.as_hex(),
                          serde_json::to_string(&self.transactions).unwrap(),
                          self.nonce);
        Hash::new(data.as_bytes())
    }
    
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!("Mining block {}...", self.index);
        
        loop {
            let hash = self.calculate_hash();
            let hash_str = hash.as_hex();
            
            if hash_str.starts_with(&target) {
                self.hash = hash;
                println!("Block mined: {}", self.hash);
                break;
            }
            
            self.nonce += 1;
            
            if self.nonce % 10000 == 0 {
                println!("Mining... nonce: {}", self.nonce);
            }
        }
    }
    
    pub fn genesis_block() -> Self {
        Block::new(0, Hash::zero(), vec![])
    }
}

// Blockchain structure
#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 2,
            pending_transactions: Vec::new(),
            mining_reward: 100,
        };
        
        let genesis_block = Block::genesis_block();
        blockchain.chain.push(genesis_block);
        
        blockchain
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }
    
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        println!("Starting to mine transactions...");
        
        let coinbase_transaction = Transaction::new(
            "coinbase".to_string(),
            mining_reward_address,
            self.mining_reward,
        );
        
        let mut block_transactions = vec![coinbase_transaction];
        block_transactions.extend(self.pending_transactions.drain(..));
        
        let new_block = Block::new(
            self.chain.len() as u64,
            self.get_latest_block().hash.clone(),
            block_transactions,
        );
        
        self.chain.push(new_block);
        println!("Block mined and added to chain!");
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0u64;
        
        for block in &self.chain {
            for transaction in &block.transactions {
                if transaction.recipient == address {
                    balance += transaction.amount;
                }
                
                if transaction.sender == address {
                    balance -= transaction.amount;
                }
            }
        }
        
        balance
    }
    
    pub fn is_valid(&self) -> bool {
        if self.chain[0].index != 0 || !self.chain[0].previous_hash.is_zero() {
            return false;
        }
        
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        
        true
    }
}

// Simple wallet
#[derive(Debug)]
pub struct Wallet {
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let address = format!("0x{:x}", rand::random::<u64>());
        Wallet { address }
    }
    
    pub fn address(&self) -> &str {
        &self.address
    }
}

// Miner
pub struct Miner {
    wallet: Wallet,
    blockchain: Blockchain,
}

impl Miner {
    pub fn new() -> Self {
        let wallet = Wallet::new();
        let blockchain = Blockchain::new();
        Miner { wallet, blockchain }
    }
    
    pub fn mine_block(&mut self) {
        if self.blockchain.pending_transactions.is_empty() {
            println!("No transactions to mine");
            return;
        }
        
        println!("Mining block with {} transactions", 
                self.blockchain.pending_transactions.len());
        
        self.blockchain.mine_pending_transactions(self.wallet.address().to_string());
        
        println!("Mining completed! Reward sent to {}", self.wallet.address());
        println!("Current balance: {}", self.get_balance());
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.blockchain.add_transaction(transaction);
    }
    
    pub fn get_balance(&self) -> u64 {
        self.blockchain.get_balance(self.wallet.address())
    }
    
    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }
}

// Simple smart contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartContract {
    pub address: String,
    pub storage: HashMap<String, String>,
    pub owner: String,
}

impl SmartContract {
    pub fn new(owner: String) -> Self {
        let address = Self::generate_address(&owner);
        
        SmartContract {
            address,
            storage: HashMap::new(),
            owner,
        }
    }
    
    fn generate_address(owner: &str) -> String {
        let data = format!("{}{}", owner, 
                          std::time::SystemTime::now()
                              .duration_since(std::time::UNIX_EPOCH)
                              .unwrap()
                              .as_nanos());
        let hash = Hash::new(data.as_bytes());
        format!("0x{}", &hash.as_hex()[..40])
    }
    
    pub fn execute(&mut self, function: &str, args: &[String], sender: &str) -> Result<String, Box<dyn std::error::Error>> {
        match function {
            "setValue" => {
                if args.len() != 2 {
                    return Err("setValue requires key and value arguments".into());
                }
                self.storage.insert(args[0].clone(), args[1].clone());
                Ok("Value set successfully".to_string())
            },
            "getValue" => {
                if args.len() != 1 {
                    return Err("getValue requires key argument".into());
                }
                match self.storage.get(&args[0]) {
                    Some(value) => Ok(value.clone()),
                    None => Ok("Key not found".to_string()),
                }
            },
            _ => Err("Unknown function".into()),
        }
    }
}

// Simple DApp
pub struct DApp {
    blockchain: Blockchain,
    contracts: HashMap<String, SmartContract>,
    users: HashMap<String, Wallet>,
}

impl DApp {
    pub fn new() -> Self {
        DApp {
            blockchain: Blockchain::new(),
            contracts: HashMap::new(),
            users: HashMap::new(),
        }
    }
    
    pub fn create_user(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.address().to_string();
        self.users.insert(address.clone(), wallet);
        address
    }
    
    pub fn deploy_contract(&mut self, owner: String) -> String {
        let contract = SmartContract::new(owner.clone());
        let address = contract.address.clone();
        self.contracts.insert(address.clone(), contract);
        address
    }
    
    pub fn execute_contract(&mut self, contract_address: &str, function: &str, 
                           args: &[String], sender: &str) -> Result<String, Box<dyn std::error::Error>> {
        let contract = self.contracts.get_mut(contract_address)
            .ok_or("Contract not found")?;
        
        contract.execute(function, args, sender)
    }
    
    pub fn transfer_funds(&mut self, from: String, to: String, amount: u64) {
        let transaction = Transaction::new(from, to, amount);
        self.blockchain.add_transaction(transaction);
    }
    
    pub fn get_user_balance(&self, address: &str) -> u64 {
        self.blockchain.get_balance(address)
    }
    
    pub fn mine_block(&mut self) {
        self.blockchain.mine_pending_transactions("system".to_string());
    }
}

// Main demonstration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BLOCKCHAIN DEMONSTRATIONS ===\n");
    
    // Create a miner
    let mut miner = Miner::new();
    println!("Created miner with address: {}", miner.wallet.address());
    
    // Create some transactions
    let alice = Wallet::new();
    let bob = Wallet::new();
    let charlie = Wallet::new();
    
    println!("Created users:");
    println!("  Alice: {}", alice.address());
    println!("  Bob: {}", bob.address());
    println!("  Charlie: {}", charlie.address());
    
    // Add transactions
    let tx1 = Transaction::new(alice.address().to_string(), bob.address().to_string(), 50);
    let tx2 = Transaction::new(bob.address().to_string(), charlie.address().to_string(), 25);
    let tx3 = Transaction::new(charlie.address().to_string(), alice.address().to_string(), 10);
    
    miner.add_transaction(tx1);
    miner.add_transaction(tx2);
    miner.add_transaction(tx3);
    
    println!("Added 3 transactions to pending pool");
    
    // Mine a block
    miner.mine_block();
    
    // Check balances
    println!("\nBalances after mining:");
    println!("  Alice: {}", miner.get_balance());
    println!("  Bob: {}", miner.blockchain.get_balance(bob.address()));
    println!("  Charlie: {}", miner.blockchain.get_balance(charlie.address()));
    
    // Verify blockchain
    println!("\nBlockchain validity: {}", miner.blockchain.is_valid());
    
    // Create a DApp
    println!("\n=== DAPP DEMONSTRATION ===");
    
    let mut dapp = DApp::new();
    
    // Create users
    let user1 = dapp.create_user();
    let user2 = dapp.create_user();
    
    println!("Created DApp users: {} and {}", user1, user2);
    
    // Deploy a contract
    let contract_address = dapp.deploy_contract(user1.clone());
    println!("Deployed contract at: {}", contract_address);
    
    // Execute contract functions
    let result = dapp.execute_contract(&contract_address, "setValue", 
                                     &vec!["name".to_string(), "Alice".to_string()], 
                                     &user1)?;
    println!("Contract execution result: {}", result);
    
    let value = dapp.execute_contract(&contract_address, "getValue", 
                                      &vec!["name".to_string()], 
                                      &user2)?;
    println!("Contract value: {}", value);
    
    // Transfer funds
    dapp.transfer_funds(user1.clone(), user2.clone(), 75);
    dapp.mine_block();
    
    println!("Transferred 75 from {} to {}", user1, user2);
    println!("User1 balance: {}", dapp.get_user_balance(&user1));
    println!("User2 balance: {}", dapp.get_user_balance(&user2));
    
    // Show blockchain stats
    let blockchain = miner.get_blockchain();
    println!("\nBlockchain stats:");
    println!("  Total blocks: {}", blockchain.chain.len());
    println!("  Latest block: {}", blockchain.get_latest_block().hash.as_hex());
    
    println!("\n=== BLOCKCHAIN DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Cryptographic hashing (SHA-256)");
    println!("- Transaction creation and management");
    println!("- Block mining with proof-of-work");
    println!("- Blockchain validation");
    println!("- Simple smart contracts");
    println!("- Decentralized application framework");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_creation() {
        let data = b"test data";
        let hash = Hash::new(data);
        assert!(!hash.as_hex().is_empty());
        assert_eq!(hash.as_hex().len(), 64); // 32 bytes * 2 hex chars
    }
    
    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("alice".to_string(), "bob".to_string(), 100);
        assert_eq!(tx.sender, "alice");
        assert_eq!(tx.recipient, "bob");
        assert_eq!(tx.amount, 100);
        assert!(!tx.id.is_empty());
    }
    
    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1); // Genesis block
        assert_eq!(blockchain.chain[0].index, 0);
        assert!(blockchain.chain[0].previous_hash.is_zero());
    }
    
    #[test]
    fn test_blockchain_validation() {
        let blockchain = Blockchain::new();
        assert!(blockchain.is_valid());
    }
    
    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert!(!wallet.address().is_empty());
        assert!(wallet.address().starts_with("0x"));
    }
    
    #[test]
    fn test_smart_contract() {
        let owner = "alice".to_string();
        let mut contract = SmartContract::new(owner.clone());
        
        let result = contract.execute("setValue", &vec!["key".to_string(), "value".to_string()], &owner).unwrap();
        assert_eq!(result, "Value set successfully");
        
        let value = contract.execute("getValue", &vec!["key".to_string()], &owner).unwrap();
        assert_eq!(value, "value");
    }
}
