// All the constants used in the project

use std::{collections::HashMap, mem::MaybeUninit};

pub struct Constants {
    pub database_constants: DatabaseConstants,
    pub blockchain_constants: BlockchainConstants,
    pub address_constants: AddressConstants,
}

impl Constants {
    fn new() -> Constants {
        let envloader_instance = EnvLoader::get_instance();

        Constants {
            database_constants: DatabaseConstants::new(&envloader_instance.envs),
            blockchain_constants: BlockchainConstants::new(&envloader_instance.envs),
            address_constants: AddressConstants::new(&envloader_instance.envs),
        }
    }

    pub fn get_instance() -> &'static Self {
        static mut INSTANCE: MaybeUninit<Constants> = MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();
        unsafe {
            ONCE.call_once(|| {
                INSTANCE.write(Constants::new());
            });

            INSTANCE.assume_init_ref()
        }
    }
}

pub mod constants {
    #[macro_export]
    macro_rules! constants_instance {
        () => {
            constants::Constants::get_instance()
        };
    }
}

pub struct DatabaseConstants {
    pub blockchain_db_path: String,
    pub wallets_db_path: String,
    pub blocks_bucket: String,
    pub wallets_bucket: String,
}

impl DatabaseConstants {
    pub fn new(envs: &HashMap<String, String>) -> DatabaseConstants {
        let blockchain_db_path = match envs.get("BLOCKCHAIN_DB_PATH") {
            Some(path) => path.to_owned(),
            None => "blockchain".to_owned(),
        };

        let wallets_db_path = match envs.get("WALLETS_DB_PATH") {
            Some(path) => path.to_owned(),
            None => "wallets".to_owned(),
        };

        let blocks_bucket = match envs.get("BLOCKS_BUCKET") {
            Some(bucket) => bucket.to_owned(),
            None => "blocks".to_owned(),
        };

        let wallets_bucket = match envs.get("WALLETS_BUCKET") {
            Some(bucket) => bucket.to_owned(),
            None => "wallets".to_owned(),
        };

        DatabaseConstants {
            blockchain_db_path,
            wallets_db_path,
            blocks_bucket,
            wallets_bucket,
        }
    }
}

pub struct BlockchainConstants {
    pub genesis_block_data: String,
    pub coinbase_reward: u64,
    pub mining_difficulty: usize,
}

impl BlockchainConstants {
    pub fn new(envs: &HashMap<String, String>) -> BlockchainConstants {
        let genesis_block_data = match envs.get("GENESIS_BLOCK_DATA") {
            Some(data) => data.to_owned(),
            None => "Idk what to put here".to_owned(),
        };

        let coinbase_reward = match envs.get("COINBASE_REWARD") {
            Some(reward) => reward.parse::<u64>().unwrap(),
            None => 100,
        };

        let mining_difficulty = match envs.get("MINING_DIFFICULTY") {
            Some(difficulty) => difficulty.parse::<usize>().unwrap(),
            None => 0,
        };

        BlockchainConstants {
            genesis_block_data,
            coinbase_reward,
            mining_difficulty,
        }
    }
}

pub struct AddressConstants {
    pub version: u8,
    pub checksum_length: usize,
}

impl AddressConstants {
    pub fn new(envs: &HashMap<String, String>) -> AddressConstants {
        let version = match envs.get("ADDRESS_VERSION") {
            Some(version) => version.parse::<u8>().unwrap(),
            None => 0x00,
        };

        let checksum_length = match envs.get("ADDRESS_CHECKSUM_LENGTH") {
            Some(length) => length.parse::<usize>().unwrap(),
            None => 4,
        };

        AddressConstants {
            version,
            checksum_length,
        }
    }
}

struct EnvLoader {
    envs: HashMap<String, String>,
}

impl EnvLoader {
    fn new() -> EnvLoader {
        let mut envs = HashMap::new();

        if dotenv::dotenv().is_ok() {
            for (key, value) in dotenv::vars() {
                envs.insert(key, value);
            }

            return EnvLoader { envs };
        }

        EnvLoader { envs }
    }

    fn get_instance() -> &'static Self {
        static mut INSTANCE: MaybeUninit<EnvLoader> = MaybeUninit::uninit();
        static ONCE: std::sync::Once = std::sync::Once::new();
        unsafe {
            ONCE.call_once(|| {
                INSTANCE.write(EnvLoader::new());
            });

            INSTANCE.assume_init_ref()
        }
    }
}
