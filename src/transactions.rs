use std::collections::HashMap;

use hex_slice::AsHex;
use num_bigint::BigInt;
use secp256k1::ecdsa::Signature;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::utils::create_bigint_from_hash;
use crate::wallet::{hash_public_key, Wallets};
use crate::BlockChain;

macro_rules! COINBASE_REWARD {
    () => {
        crate::constants::Constants::get_instance()
            .blockchain_constants
            .coinbase_reward
    };
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: BigInt,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut inputs: Vec<String> = vec![];
        let mut outputs: Vec<String> = vec![];

        for input in &self.inputs {
            inputs.push(format!("{:?}", input));
        }

        for output in &self.outputs {
            outputs.push(format!("{:?}", output));
        }

        write!(
            f,
            "Transaction {{\n\tid: {:064x},\n\tinputs: {:?},\n\toutputs: {:?}\n}}\n",
            self.id, inputs, outputs
        )
    }
}
impl Transaction {
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Result<Self, &'static str> {
        let transaction = Transaction {
            id: BigInt::from(0),
            inputs,
            outputs,
        };

        Ok(transaction)
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1
            && self.inputs[0].transaction_id == BigInt::from(0)
            && self.inputs[0].output_index == 0
    }

    pub fn hash(&self) -> Result<BigInt, &'static str> {
        let serialized_data = bincode::serialize(&self).unwrap();

        create_bigint_from_hash(&digest(serialized_data.as_slice()))
    }

    pub fn sign(
        &mut self,
        private_key: &SecretKey,
        prev_transactions: HashMap<BigInt, Transaction>,
    ) {
        if self.is_coinbase() {
            return;
        }

        let mut transaction_copy = self.trimmed_copy();

        for (input_index, input) in self.trimmed_copy().inputs.iter().enumerate() {
            let prev_transaction = prev_transactions.get(&input.transaction_id).unwrap();

            transaction_copy.inputs[input_index].signature = vec![];

            transaction_copy.inputs[input_index].public_key = prev_transaction.outputs
                [input.output_index]
                .public_key_hash
                .clone();

            transaction_copy.id = transaction_copy.hash().unwrap();

            transaction_copy.inputs[input_index].public_key = vec![];

            let secp = Secp256k1::new();

            let signature = secp.sign_ecdsa(
                &Message::from_hashed_data::<secp256k1::hashes::sha256::Hash>(
                    transaction_copy.id.to_bytes_be().1.as_slice(),
                ),
                private_key,
            );

            self.inputs[input_index].signature = signature.serialize_der().to_vec();
        }
    }

    pub fn verify(
        &self,
        previous_transactions: HashMap<BigInt, Transaction>,
    ) -> Result<bool, String> {
        let mut transaction_copy = self.trimmed_copy();

        for (input_index, input) in self.inputs.iter().enumerate() {
            let previous_transaction = match previous_transactions.get(&input.transaction_id) {
                Some(transaction) => transaction,
                None => return Err("Previous transaction is not correct".to_string()),
            };

            transaction_copy.inputs[input_index].signature = vec![];

            transaction_copy.inputs[input_index].public_key = previous_transaction.outputs
                [input.output_index]
                .public_key_hash
                .clone();

            transaction_copy.id = transaction_copy.hash()?;

            transaction_copy.inputs[input_index].public_key = vec![];

            let secp = Secp256k1::new();

            let signature = match Signature::from_der(&input.signature) {
                Ok(signature) => signature,
                Err(msg) => return Err(msg.to_string()),
            };

            if secp
                .verify_ecdsa(
                    &Message::from_hashed_data::<secp256k1::hashes::sha256::Hash>(
                        transaction_copy.id.to_bytes_be().1.as_slice(),
                    ),
                    &signature,
                    &PublicKey::from_slice(&input.public_key).unwrap(),
                )
                .is_err()
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn trimmed_copy(&self) -> Transaction {
        let mut inputs: Vec<Input> = Vec::new();
        let mut outputs: Vec<Output> = Vec::new();

        for input in &self.inputs {
            inputs.push(Input {
                transaction_id: input.transaction_id.clone(),
                output_index: input.output_index,
                signature: vec![],
                public_key: vec![],
            });
        }

        for output in &self.outputs {
            outputs.push(Output {
                value: output.value,
                public_key_hash: output.public_key_hash.clone(),
            });
        }

        Transaction {
            id: self.id.clone(),
            inputs,
            outputs,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Output {
    pub value: u64,
    pub public_key_hash: Vec<u8>,
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ value: {}, public_key_hash: {:x} }}",
            self.value,
            self.public_key_hash.as_hex()
        )
    }
}

impl Output {
    pub fn new(value: u64, address: &str) -> Self {
        let mut output = Output {
            value,
            public_key_hash: vec![],
        };

        output.lock(address.to_string());

        output
    }

    pub fn lock(&mut self, address: String) {
        let public_key_hash = bs58::decode(address)
            .into_vec()
            .unwrap()
            .get(1..21)
            .unwrap()
            .to_vec();

        self.public_key_hash = public_key_hash;
    }


    pub fn is_locked_with_key(&self, public_key_hash: &Vec<u8>) -> bool {
        &self.public_key_hash == public_key_hash
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Input {
    pub transaction_id: BigInt,
    pub output_index: usize,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ transaction_id: {:064x}, output_index: {}, signature: {:x}, public_key: {:x} }}",
            self.transaction_id,
            self.output_index,
            self.signature.as_hex(),
            self.public_key.as_hex()
        )
    }
}

impl Input {
    pub fn new(
        transaction_id: BigInt,
        output_index: usize,
        signature: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Self {
        Input {
            transaction_id,
            output_index,
            signature,
            public_key,
        }
    }

    pub fn uses_key(&self, public_key_hash: &Vec<u8>) -> bool {
        &hash_public_key(&PublicKey::from_slice(&self.public_key).unwrap()) == public_key_hash
    }
}

pub fn new_coinbase_transaction(receiver: &str, mut data: String) -> Result<Transaction, String> {
    if data.is_empty() {
        data = format!("Reward to '{}'", receiver);
    }

    let mut transaction = Transaction::new(
        vec![Input {
            transaction_id: BigInt::from(0),
            output_index: 0,
            signature: vec![],
            public_key: vec![],
        }],
        vec![Output::new(COINBASE_REWARD!(), receiver)],
    )?;

    transaction.id = transaction.hash()?;

    Ok(transaction)
}

pub fn new_transaction(
    sender: &str,
    receiver: &str,
    amount: u64,
    block_chain: &BlockChain,
    wallets: &Wallets,
) -> Result<Transaction, &'static str> {
    let mut inputs: Vec<Input> = vec![];
    let mut outputs: Vec<Output> = vec![];

    let sender_wallet = match wallets.get_wallet(sender) {
        Some(wallet) => wallet,
        None => return Err("Sender wallet not found"),
    };

    let public_key_hash = hash_public_key(&sender_wallet.public_key);

    let (acc, valid_outputs) = block_chain.get_spendable_outputs(public_key_hash, amount);

    if acc < amount {
        return Err("Not enough funds");
    }

    for (transaction_id, output_indexes) in valid_outputs {
        for output_index in output_indexes {
            inputs.push(Input {
                transaction_id: transaction_id.clone(),
                output_index,
                signature: vec![],
                public_key: sender_wallet.public_key.serialize().to_vec(),
            });
        }
    }

    let mut output = Output {
        value: amount,
        public_key_hash: vec![],
    };

    output.lock(receiver.to_string());
    outputs.push(output);

    if acc > amount {
        let mut output = Output {
            value: acc - amount,
            public_key_hash: vec![],
        };

        output.lock(sender.to_string());
        outputs.push(output);
    }

    let mut transaction = match Transaction::new(inputs, outputs) {
        Ok(transactions) => transactions,
        Err(msg) => return Err(msg),
    };

    transaction.id = transaction.hash()?;
    block_chain.sign_transaction(&mut transaction, &sender_wallet.private_key);

    Ok(transaction)
}
