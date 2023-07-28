pub mod block;
pub mod constants;
pub mod proof_of_work;
pub mod transactions;
pub mod utils;
pub mod wallet;

use secp256k1::SecretKey;

use block::Block;
use num_bigint::{BigInt, Sign};
use proof_of_work::{proof_of_work, validate};
use std::{collections::HashMap, mem::MaybeUninit};
use transactions::Transaction;
use wallet::Wallets;

macro_rules! BLOCK_DB_PATH {
    () => {
        constants::Constants::get_instance()
            .database_constants
            .blockchain_db_path
    };
}

macro_rules! BLOCKS_BUCKET {
    () => {
        constants::Constants::get_instance()
            .database_constants
            .blocks_bucket
            .as_str()
    };
}

macro_rules! GENESIS_BLOCK_DATA {
    () => {
        constants::Constants::get_instance()
            .blockchain_constants
            .genesis_block_data
            .to_owned()
    };
}

pub struct BlockChain {
    pub apex_hash: BigInt,
    pub db: sled::Db,
}

pub struct BlockChainIterator {
    pub current_hash: BigInt,
    pub db: sled::Db,
}

impl BlockChain {
    fn new() -> Result<BlockChain, String> {
        let res = sled::open(&BLOCK_DB_PATH!());

        let db = match res {
            Ok(db) => db,
            Err(_) => {
                return Err("Failed to open database".to_owned());
            }
        };

        let bucket: sled::Tree;

        if db
            .tree_names()
            .contains(&sled::IVec::from(BLOCKS_BUCKET!()))
        {
            bucket = match db.open_tree(BLOCKS_BUCKET!()) {
                Ok(bucket) => bucket,
                Err(_) => {
                    return Err("Failed to open blocks bucket".to_owned());
                }
            };

            let apex_hash = match bucket.get("apex") {
                Ok(apex) => match apex {
                    Some(apex) => BigInt::from_bytes_be(Sign::Plus, &apex),
                    None => {
                        return Err("Failed to get node from database".to_owned());
                    }
                },
                Err(_) => {
                    return Err("Failed to get node from database".to_owned());
                }
            };

            return Ok(BlockChain { apex_hash, db });
        }

        let wallets = Wallets::fetch_wallets();
        let genesis_wallet_address = wallets.create_wallet().unwrap();

        let res =
            transactions::new_coinbase_transaction(&genesis_wallet_address, GENESIS_BLOCK_DATA!());

        if let Err(e) = res {
            return Err(e);
        }

        let coinbase_transaction = res.unwrap();

        let genesis_block = block::Block::new(vec![coinbase_transaction], BigInt::from(0));
        bucket = db.open_tree(BLOCKS_BUCKET!()).unwrap();

        let buffer = match bincode::serialize(&genesis_block) {
            Ok(buffer) => buffer,
            Err(_) => {
                return Err("Failed to serialize genesis block".to_owned());
            }
        };

        if let Err(e) = bucket.insert(genesis_block.hash.to_bytes_be().1, buffer) {
            return Err(format!(
                "Failed to insert genesis block into database: {}",
                e
            ));
        }

        if let Err(e) = bucket.insert("apex", genesis_block.hash.to_bytes_be().1) {
            return Err(format!("Failed to insert last into database: {}", e));
        }

        Ok(BlockChain {
            apex_hash: genesis_block.hash,
            db,
        })
    }

    pub fn fetch_blockchain() -> &'static mut Self {
        static mut INSTANCE: MaybeUninit<BlockChain> = MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                INSTANCE.write(BlockChain::new().unwrap());
            });

            INSTANCE.assume_init_mut()
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), String> {
        for transaction in &transactions {
            if !transaction.is_coinbase() {
                if !transaction.verify(self.get_prev_transactions(transaction))? {
                    return Err("Invalid transaction!".to_owned());
                }
            }
        }

        let mut block = block::Block::new(transactions, self.apex_hash.clone());
        proof_of_work(&mut block);

        if let Err(e) = validate(&mut block) {
            return Err(e.to_owned());
        }

        let buffer = match bincode::serialize(&block) {
            Ok(buffer) => buffer,
            Err(_) => {
                return Err("Failed to serialize block!".to_owned());
            }
        };

        let bucket = match self.db.open_tree(BLOCKS_BUCKET!()) {
            Ok(bucket) => bucket,
            Err(_) => {
                return Err("Failed to open blocks bucket".to_owned());
            }
        };

        if let Err(e) = bucket.insert(block.hash.to_bytes_be().1, buffer) {
            return Err(format!("Failed to insert block into database: {}", e));
        }

        if let Err(e) = bucket.insert("apex", block.hash.to_bytes_be().1) {
            return Err(format!("Failed to insert last into database: {}", e));
        }

        self.apex_hash = block.hash;

        self.db.flush().unwrap();

        Ok(())
    }

    pub fn send(
        &mut self,
        sender: &str,
        reciever: &str,
        amount: u64,
        wallets: &Wallets,
    ) -> Result<(), String> {
        let transaction = transactions::new_transaction(sender, reciever, amount, self, wallets);

        if let Err(e) = transaction {
            return Err(e.to_owned());
        }

        self.add_block(vec![transaction.unwrap()])
    }

    pub fn get_iterator(&self) -> BlockChainIterator {
        let current_hash = self.apex_hash.clone();
        BlockChainIterator {
            current_hash,
            db: self.db.clone(),
        }
    }

    pub fn get_unspent_transactions(&self, public_key_hash: &Vec<u8>) -> Vec<Transaction> {
        let mut unspent_transactions = vec![];
        let mut spent_outputs: HashMap<String, Vec<usize>> = HashMap::new();

        let mut iterator = self.get_iterator();

        let mut skip_output = false;

        loop {
            let block = iterator.next_block();

            for transaction in block.transactions {
                let transaction_id = transaction.id.to_string();

                for (index, output) in transaction.outputs.clone().iter().enumerate() {
                    if spent_outputs.contains_key(&transaction_id) {
                        let spent_outputs_index = spent_outputs.get(&transaction_id).unwrap();

                        for spent_output in spent_outputs_index {
                            if *spent_output == index {
                                skip_output = true;
                            }
                        }
                    }

                    if skip_output {
                        skip_output = false;
                        continue;
                    }

                    if output.is_locked_with_key(public_key_hash) {
                        unspent_transactions.push(transaction.clone());
                    }
                }

                if !transaction.is_coinbase() {
                    for input in transaction.inputs {
                        if input.uses_key(public_key_hash) {
                            let transaction_id = input.transaction_id.to_string();

                            if let Some(spent_outputs) = spent_outputs.get_mut(&transaction_id) {
                                spent_outputs.push(input.output_index);
                            } else {
                                spent_outputs.insert(transaction_id, vec![input.output_index]);
                            }
                        }
                    }
                }
            }

            if block.prev_hash == BigInt::from(0) {
                break;
            }
        }

        unspent_transactions
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0;
        let public_key_hash = bs58::decode(address).into_vec().unwrap()[1..21].to_vec();
        let unspent_transactions = self.get_unspent_transactions(&public_key_hash);

        for transaction in unspent_transactions {
            for output in transaction.outputs {
                if output.is_locked_with_key(&public_key_hash) {
                    balance += output.value;
                }
            }
        }

        balance
    }

    pub fn get_spendable_outputs(
        &self,
        public_key_hash: Vec<u8>,
        amount: u64,
    ) -> (u64, HashMap<BigInt, Vec<usize>>) {
        let mut unspent_outputs: HashMap<BigInt, Vec<usize>> = HashMap::new();
        let unspent_transactions = self.get_unspent_transactions(&public_key_hash);

        let mut accumulated = 0;

        for transaction in unspent_transactions {
            for (index, output) in transaction.outputs.iter().enumerate() {
                if output.is_locked_with_key(&public_key_hash) && accumulated < amount {
                    accumulated += output.value;

                    if let Some(vec) = unspent_outputs.get_mut(&transaction.id) {
                        vec.push(index);
                    } else {
                        unspent_outputs.insert(transaction.id.clone(), vec![index]);
                    }
                }

                if accumulated >= amount {
                    break;
                }
            }
        }

        (accumulated, unspent_outputs)
    }

    pub fn find_transaction(&self, id: &BigInt) -> Option<Transaction> {
        let mut iterator = self.get_iterator();

        loop {
            let block = iterator.next_block();

            for transaction in block.transactions {
                if transaction.id == *id {
                    return Some(transaction);
                }
            }

            if block.prev_hash == BigInt::from(0) {
                break;
            }
        }

        None
    }

    fn get_prev_transactions(&self, transaction: &Transaction) -> HashMap<BigInt, Transaction> {
        let mut prev_transactions: HashMap<BigInt, Transaction> = HashMap::new();

        for input in transaction.inputs.clone() {
            let prev_transaction = self.find_transaction(&input.transaction_id).unwrap();
            prev_transactions.insert(prev_transaction.id.clone(), prev_transaction);
        }

        prev_transactions
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction, private_key: &SecretKey) {
        transaction.sign(private_key, self.get_prev_transactions(transaction));
    }

    pub fn verify_transaction(&self, transaction: &mut Transaction) -> Result<bool, String> {
        transaction.verify(self.get_prev_transactions(transaction))
    }
}

impl std::fmt::Debug for BlockChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iterator = self.get_iterator();
        loop {
            let block = iterator.next_block();
            write!(f, "{:?}", block)?;

            if block.prev_hash == BigInt::from(0) {
                break;
            }
        }

        Ok(())
    }
}

impl BlockChainIterator {
    pub fn next_block(&mut self) -> Block {
        let bucket = self.db.open_tree(BLOCKS_BUCKET!()).unwrap();

        let buffer = bucket
            .get(self.current_hash.to_bytes_be().1)
            .unwrap()
            .unwrap();

        let block: block::Block = bincode::deserialize(&buffer).unwrap();

        self.current_hash = block.prev_hash.clone();

        block
    }
}
