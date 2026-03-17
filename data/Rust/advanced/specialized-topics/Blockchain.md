# Blockchain in Rust

## Overview

Rust's performance, memory safety, and concurrency features make it excellent for blockchain development. This guide covers building blockchain components, cryptocurrency concepts, smart contracts, and decentralized applications in Rust.

---

## Blockchain Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `sha2` | Cryptographic hashing | SHA-256, SHA-512 |
| `secp256k1` | Elliptic curve cryptography | Bitcoin's ECDSA |
| `serde` | Serialization | Data encoding/decoding |
| `hex` | Hex encoding | Binary to hex conversion |
| `rand` | Random number generation | Cryptographic randomness |
| `tokio` | Async runtime | Network operations |
| `clap` | CLI framework | Command-line interface |
| `rocksdb` | Database | Persistent storage |

---

## Core Blockchain Concepts

### Cryptographic Hashing

```rust
use sha2::{Sha256, Digest};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&result);
        Hash(hash_bytes)
    }
    
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
    
    pub fn as_hex(&self) -> String {
        hex::encode(self.0)
    }
    
    pub fn from_hex(hex_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = hex::decode(hex_str)?;
        if bytes.len() != 32 {
            return Err("Invalid hash length".into());
        }
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&bytes);
        Ok(Hash(hash_bytes))
    }
    
    pub fn zero() -> Self {
        Hash([0u8; 32])
    }
    
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&b| b == 0)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}

// Merkle Tree implementation
#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: Hash,
    leaves: Vec<Hash>,
}

impl MerkleTree {
    pub fn new(data: &[Vec<u8>]) -> Self {
        let leaves: Vec<Hash> = data.iter().map(|d| Hash::new(d)).collect();
        let root = Self::build_merkle_root(&leaves);
        MerkleTree { root, leaves }
    }
    
    fn build_merkle_root(hashes: &[Hash]) -> Hash {
        if hashes.is_empty() {
            return Hash::zero();
        }
        
        if hashes.len() == 1 {
            return hashes[0].clone();
        }
        
        let mut next_level = Vec::new();
        
        for chunk in hashes.chunks(2) {
            let combined = if chunk.len() == 2 {
                let mut combined_data = chunk[0].as_bytes().to_vec();
                combined_data.extend_from_slice(chunk[1].as_bytes());
                Hash::new(&combined_data)
            } else {
                let mut combined_data = chunk[0].as_bytes().to_vec();
                combined_data.extend_from_slice(chunk[0].as_bytes());
                Hash::new(&combined_data)
            };
            next_level.push(combined);
        }
        
        Self::build_merkle_root(&next_level)
    }
    
    pub fn root(&self) -> &Hash {
        &self.root
    }
    
    pub fn verify_proof(&self, proof: &[Hash], leaf_index: usize) -> bool {
        if leaf_index >= self.leaves.len() {
            return false;
        }
        
        let mut current_hash = self.leaves[leaf_index].clone();
        let mut index = leaf_index;
        
        for proof_hash in proof {
            if index % 2 == 0 {
                let mut combined_data = current_hash.as_bytes().to_vec();
                combined_data.extend_from_slice(proof_hash.as_bytes());
                current_hash = Hash::new(&combined_data);
            } else {
                let mut combined_data = proof_hash.as_bytes().to_vec();
                combined_data.extend_from_slice(current_hash.as_bytes());
                current_hash = Hash::new(&combined_data);
            }
            index /= 2;
        }
        
        current_hash == self.root
    }
}
```

### Digital Signatures

```rust
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, Signature};
use rand::rngs::OsRng;

#[derive(Debug, Clone)]
pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut OsRng);
        
        let address = Self::generate_address(&public_key);
        
        Ok(Wallet {
            private_key,
            public_key,
            address,
        })
    }
    
    fn generate_address(public_key: &PublicKey) -> String {
        // Simplified address generation - in real blockchain, use proper address derivation
        let public_key_bytes = public_key.serialize_uncompressed();
        let hash = Hash::new(&public_key_bytes);
        format!("0x{}", &hash.as_hex()[..40]) // Use first 20 bytes
    }
    
    pub fn sign_message(&self, message: &[u8]) -> Result<Signature, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let msg = Message::from_slice(message)?;
        Ok(secp.sign(&msg, &self.private_key))
    }
    
    pub fn verify_signature(&self, message: &[u8], signature: &Signature) -> Result<bool, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let msg = Message::from_slice(message)?;
        Ok(secp.verify(&msg, signature, &self.public_key).is_ok())
    }
    
    pub fn address(&self) -> &str {
        &self.address
    }
    
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

// Transaction structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
    
    pub fn get_signing_data(&self) -> Vec<u8> {
        format!("{}{}{}{}{}", self.id, self.sender, self.recipient, 
               self.amount, self.timestamp).into_bytes()
    }
    
    pub fn sign(&mut self, wallet: &Wallet) -> Result<(), Box<dyn std::error::Error>> {
        let signing_data = self.get_signing_data();
        let signature = wallet.sign_message(&signing_data)?;
        self.signature = Some(hex::encode(signature.serialize_compact()));
        Ok(())
    }
    
    pub fn verify_signature(&self, public_key: &PublicKey) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(sig_hex) = &self.signature {
            let signature_bytes = hex::decode(sig_hex)?;
            let signature = Signature::from_compact(&signature_bytes)?;
            
            let signing_data = self.get_signing_data();
            let secp = Secp256k1::new();
            let msg = Message::from_slice(&signing_data)?;
            
            Ok(secp.verify(&msg, &signature, public_key).is_ok())
        } else {
            Ok(false)
        }
    }
}
```

---

## Blockchain Structure

### Block Implementation

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
        
        block.mine_block(4); // 4 leading zeros required
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
            
            if self.nonce % 100000 == 0 {
                println!("Mining... nonce: {}", self.nonce);
            }
        }
    }
    
    pub fn is_valid(&self) -> bool {
        if self.hash != self.calculate_hash() {
            return false;
        }
        
        // Verify all transactions
        for transaction in &self.transactions {
            if let Some(_) = &transaction.signature {
                // In a real implementation, verify each transaction signature
                // For simplicity, we'll skip this in the example
            } else {
                return false; // Unsigned transaction
            }
        }
        
        true
    }
    
    pub fn genesis_block() -> Self {
        Block::new(0, Hash::zero(), vec![])
    }
}
```

### Blockchain Implementation

```rust
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
            difficulty: 4,
            pending_transactions: Vec::new(),
            mining_reward: 100,
        };
        
        // Create genesis block
        let genesis_block = Block::genesis_block();
        blockchain.chain.push(genesis_block);
        
        blockchain
    }
    
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
    
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // In a real implementation, validate transaction before adding
        self.pending_transactions.push(transaction);
    }
    
    pub fn mine_pending_transactions(&mut self, mining_reward_address: String) {
        println!("Starting to mine transactions...");
        
        // Create coinbase transaction
        let mut coinbase_transaction = Transaction::new(
            "coinbase".to_string(),
            mining_reward_address,
            self.mining_reward,
        );
        
        // Add all pending transactions to a new block
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
        // Check genesis block
        if self.chain[0].index != 0 || !self.chain[0].previous_hash.is_zero() {
            return false;
        }
        
        // Check all blocks
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
            
            if !current_block.is_valid() {
                return false;
            }
        }
        
        true
    }
    
    pub fn get_chain_stats(&self) -> BlockchainStats {
        let total_blocks = self.chain.len();
        let total_transactions = self.chain.iter()
            .map(|block| block.transactions.len())
            .sum::<usize>();
        
        let latest_block = self.get_latest_block();
        
        BlockchainStats {
            total_blocks,
            total_transactions,
            latest_block_hash: latest_block.hash.as_hex(),
            latest_block_timestamp: latest_block.timestamp,
            difficulty: self.difficulty,
            pending_transactions: self.pending_transactions.len(),
        }
    }
}

#[derive(Debug)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub total_transactions: usize,
    pub latest_block_hash: String,
    pub latest_block_timestamp: u64,
    pub difficulty: usize,
    pub pending_transactions: usize,
}
```

---

## Mining and Consensus

### Proof of Work Mining

```rust
pub struct Miner {
    wallet: Wallet,
    blockchain: Blockchain,
}

impl Miner {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let wallet = Wallet::new()?;
        let blockchain = Blockchain::new();
        
        Ok(Miner { wallet, blockchain })
    }
    
    pub fn mine_block(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.blockchain.pending_transactions.is_empty() {
            println!("No transactions to mine");
            return Ok(());
        }
        
        println!("Mining block with {} transactions", 
                self.blockchain.pending_transactions.len());
        
        self.blockchain.mine_pending_transactions(self.wallet.address().to_string());
        
        println!("Mining completed! Reward sent to {}", self.wallet.address());
        println!("Current balance: {}", self.get_balance());
        
        Ok(())
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
    
    pub fn get_wallet(&self) -> &Wallet {
        &self.wallet
    }
}

// Mining pool simulation
pub struct MiningPool {
    miners: Vec<Miner>,
    blockchain: Blockchain,
    pool_reward_shares: HashMap<String, f64>,
}

impl MiningPool {
    pub fn new(num_miners: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let mut miners = Vec::new();
        let mut pool_reward_shares = HashMap::new();
        
        for i in 0..num_miners {
            let miner = Miner::new()?;
            pool_reward_shares.insert(miner.wallet.address().to_string(), 1.0 / num_miners as f64);
            miners.push(miner);
        }
        
        Ok(MiningPool {
            miners,
            blockchain: Blockchain::new(),
            pool_reward_shares,
        })
    }
    
    pub fn distribute_rewards(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let total_reward = self.blockchain.mining_reward;
        
        for miner in &self.miners {
            let share = self.pool_reward_shares[miner.wallet.address()];
            let reward = (total_reward as f64 * share) as u64;
            
            let reward_transaction = Transaction::new(
                "pool".to_string(),
                miner.wallet.address().to_string(),
                reward,
            );
            
            self.blockchain.add_transaction(reward_transaction);
        }
        
        Ok(())
    }
}
```

---

## Smart Contracts

### Basic Smart Contract System

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartContract {
    pub address: String,
    pub code: String,
    pub storage: HashMap<String, String>,
    pub owner: String,
}

impl SmartContract {
    pub fn new(owner: String, code: String) -> Self {
        let address = Self::generate_address(&owner, &code);
        
        SmartContract {
            address,
            code,
            storage: HashMap::new(),
            owner,
        }
    }
    
    fn generate_address(owner: &str, code: &str) -> String {
        let data = format!("{}{}{}", owner, code, 
                          std::time::SystemTime::now()
                              .duration_since(std::time::UNIX_EPOCH)
                              .unwrap()
                              .as_nanos());
        let hash = Hash::new(data.as_bytes());
        format!("0x{}", &hash.as_hex()[..40])
    }
    
    pub fn execute(&mut self, function: &str, args: &[String], sender: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Simple contract execution - in reality, this would be much more complex
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
            "transferOwnership" => {
                if args.len() != 1 {
                    return Err("transferOwnership requires new owner argument".into());
                }
                if sender != self.owner {
                    return Err("Only owner can transfer ownership".into());
                }
                self.owner = args[0].clone();
                Ok("Ownership transferred successfully".to_string())
            },
            _ => Err("Unknown function".into()),
        }
    }
}

// Contract transaction
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContractTransaction {
    pub contract_address: String,
    pub function: String,
    pub args: Vec<String>,
    pub sender: String,
    pub value: u64,
    pub gas_limit: u64,
}

impl ContractTransaction {
    pub fn new(contract_address: String, function: String, args: Vec<String>, 
               sender: String, value: u64) -> Self {
        ContractTransaction {
            contract_address,
            function,
            args,
            sender,
            value,
            gas_limit: 21000, // Default gas limit
        }
    }
}
```

---

## Decentralized Applications

### Simple DApp Framework

```rust
#[derive(Debug, Clone)]
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
    
    pub fn create_user(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let wallet = Wallet::new()?;
        let address = wallet.address().to_string();
        self.users.insert(address.clone(), wallet);
        Ok(address)
    }
    
    pub fn deploy_contract(&mut self, owner: String, code: String) -> Result<String, Box<dyn std::error::Error>> {
        let contract = SmartContract::new(owner.clone(), code);
        let address = contract.address.clone();
        self.contracts.insert(address.clone(), contract);
        Ok(address)
    }
    
    pub fn execute_contract(&mut self, contract_address: &str, function: &str, 
                           args: &[String], sender: &str) -> Result<String, Box<dyn std::error::Error>> {
        let contract = self.contracts.get_mut(contract_address)
            .ok_or("Contract not found")?;
        
        contract.execute(function, args, sender)
    }
    
    pub fn transfer_funds(&mut self, from: String, to: String, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        let from_wallet = self.users.get(&from).ok_or("Sender not found")?;
        let to_wallet = self.users.get(&to).ok_or("Recipient not found")?;
        
        let mut transaction = Transaction::new(from, to, amount);
        transaction.sign(from_wallet)?;
        
        self.blockchain.add_transaction(transaction);
        Ok(())
    }
    
    pub fn get_user_balance(&self, address: &str) -> u64 {
        self.blockchain.get_balance(address)
    }
    
    pub fn get_contract_storage(&self, contract_address: &str, key: &str) -> Option<String> {
        self.contracts.get(contract_address)?.storage.get(key).cloned()
    }
    
    pub fn mine_block(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.blockchain.mine_pending_transactions("system".to_string());
        Ok(())
    }
    
    pub fn get_stats(&self) -> DAppStats {
        DAppStats {
            total_users: self.users.len(),
            total_contracts: self.contracts.len(),
            blockchain_stats: self.blockchain.get_chain_stats(),
        }
    }
}

#[derive(Debug)]
pub struct DAppStats {
    pub total_users: usize,
    pub total_contracts: usize,
    pub blockchain_stats: BlockchainStats,
}
```

---

## Key Takeaways

- **Cryptography** is fundamental to blockchain security
- **Hashing** provides integrity and linking
- **Digital signatures** verify ownership and authenticity
- **Mining** secures the network through proof of work
- **Smart contracts** enable programmable blockchain
- **Consensus** ensures network agreement
- **Decentralization** removes single points of failure

---

## Blockchain Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Security** | Use proven cryptographic primitives | secp256k1, SHA-256 |
| **Validation** | Verify all transactions and blocks | Comprehensive checks |
| **Gas limits** | Prevent infinite loops | Resource limits |
| **Error handling** | Handle network failures gracefully | Result types |
| **Testing** | Test edge cases thoroughly | Unit and integration tests |
| **Performance** | Optimize for throughput | Parallel processing |
| **Standards** | Follow blockchain standards | Common protocols |
