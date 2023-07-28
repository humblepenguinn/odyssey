use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::{
    proof_of_work::proof_of_work, transactions::Transaction, utils::create_bigint_from_hash,
};

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub timestamp: BigInt,
    pub transactions: Vec<Transaction>,
    pub prev_hash: BigInt,
    pub hash: BigInt,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_hash: BigInt) -> Self {
        let mut block = Block {
            timestamp: BigInt::from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            transactions,
            prev_hash,
            hash: BigInt::default(),
            nonce: 0,
        };

        proof_of_work(&mut block);
        block
    }

    pub fn get_headers(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(self.prev_hash.to_bytes_be().1.iter());

        let buffer = bincode::serialize(&self.transactions).unwrap();

        bytes.extend(buffer);

        bytes.extend(self.timestamp.to_bytes_be().1.iter());

        bytes
    }

    pub fn hash(&mut self) -> BigInt {
        create_bigint_from_hash(&digest(self.get_headers().as_slice())).unwrap()
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block {{\n\ttimestamp: {},\n\ttransactions: {:?},\n\tprev_hash: {:064x},\n\thash: {:064x}\n}}\n",
            0, self.transactions, self.prev_hash, self.hash
        )
    }
}
