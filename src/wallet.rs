use std::collections::HashMap;
use std::mem::MaybeUninit;

use ripemd::{Digest, Ripemd160};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha256::digest;

macro_rules! DB_PATH {
    () => {
        crate::constants::Constants::get_instance()
            .database_constants
            .wallets_db_path
    };
}

macro_rules! WALLETS_BUCKET {
    () => {
        crate::constants::Constants::get_instance()
            .database_constants
            .wallets_bucket
    };
}

macro_rules! CHECKSUM_LENGTH {
    () => {
        crate::constants::Constants::get_instance()
            .address_constants
            .checksum_length
    };
}

macro_rules! VERSION {
    () => {
        crate::constants::Constants::get_instance()
            .address_constants
            .version
    };
}

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let (private_key, public_key) = secp.generate_keypair(&mut OsRng);

        Wallet {
            private_key,
            public_key,
        }
    }

    pub fn get_address(&self) -> String {
        let mut payload = vec![VERSION!()];
        payload.extend(hash_public_key(&self.public_key));
        payload.extend(checksum(&payload));

        bs58::encode(payload).into_string()
    }
}

pub fn hash_public_key(public_key: &PublicKey) -> Vec<u8> {
    let mut hasher = Ripemd160::new();

    hasher.update(&digest(&public_key.serialize()));
    hasher.finalize().to_vec()
}

pub fn checksum(payload: &[u8]) -> Vec<u8> {
    digest(digest(payload)).into_bytes()[..CHECKSUM_LENGTH!()].to_vec()
}

pub struct Wallets {
    pub wallets: HashMap<String, Wallet>,
    db: sled::Db,
}

impl Wallets {
    fn new() -> Result<Self, String> {
        let res = sled::open(&DB_PATH!());

        let db = match res {
            Ok(db) => db,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let bucket: sled::Tree;

        if db
            .tree_names()
            .contains(&sled::IVec::from(&*WALLETS_BUCKET!()))
        {
            bucket = match db.open_tree(&WALLETS_BUCKET!()) {
                Ok(bucket) => bucket,
                Err(_) => {
                    return Err("Failed to open blocks bucket".to_owned());
                }
            };

            let mut wallets: HashMap<String, Wallet> = HashMap::new();

            for res in bucket.iter() {
                let (key, value) = match res {
                    Ok((key, value)) => (key, value),
                    Err(_) => {
                        return Err("Failed to iterate over blocks bucket".to_owned());
                    }
                };

                let wallet: Wallet = match bincode::deserialize(&value) {
                    Ok(wallet) => wallet,
                    Err(_) => {
                        return Err("Failed to deserialize wallet".to_owned());
                    }
                };

                wallets.insert(String::from_utf8(key.to_vec()).unwrap(), wallet);
            }

            return Ok(Wallets { wallets, db });
        }

        Ok(Wallets {
            wallets: HashMap::new(),
            db,
        })
    }

    pub fn fetch_wallets() -> &'static mut Self {
        static mut INSTANCE: MaybeUninit<Wallets> = MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();

        unsafe {
            ONCE.call_once(|| {
                INSTANCE.write(Wallets::new().unwrap());
            });

            INSTANCE.assume_init_mut()
        }
    }

    pub fn create_wallet(&mut self) -> Result<String, String> {
        let wallet = Wallet::new();
        let address = wallet.get_address();

        let bucket = match self.db.open_tree(&WALLETS_BUCKET!()) {
            Ok(bucket) => bucket,
            Err(_) => {
                return Err("Failed to open blocks bucket".to_owned());
            }
        };

        let serialized_wallet = match bincode::serialize(&wallet) {
            Ok(serialized_wallet) => serialized_wallet,
            Err(_) => {
                return Err("Failed to serialize wallet".to_owned());
            }
        };

        match bucket.insert(address.clone(), serialized_wallet) {
            Ok(_) => {}
            Err(_) => {
                return Err("Failed to insert wallet into blocks bucket".to_owned());
            }
        };

        self.wallets.insert(address.clone(), wallet);

        Ok(address)
    }

    pub fn get_addresses(&self) -> Vec<String> {
        let mut addresses = Vec::new();

        for (address, _) in &self.wallets {
            addresses.push(address.clone());
        }

        addresses
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }
}
