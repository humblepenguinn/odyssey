use num_bigint::BigInt;
use sha256::digest;

use crate::{block::Block, utils::create_bigint_from_hash};


macro_rules! DIFFICULTY {
    () => {
        crate::constants::Constants::get_instance()
            .blockchain_constants
            .mining_difficulty
    };
}

#[inline]
fn get_target(difficulty: usize) -> BigInt {
    let mut bytes: [u8; 32] = [0; 32];
    bytes[difficulty / 8] = 1 << (7 - difficulty % 8);

    BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes)
}

pub fn proof_of_work(block: &mut Block) {
    let mut hash: BigInt;
    let mut nonce = 0;
    let target = get_target(DIFFICULTY!());

    while nonce < u64::MAX {
        let mut data = block.get_headers();
        data.extend(&nonce.to_be_bytes());

        hash = match create_bigint_from_hash(&digest(data.as_slice())) {
            Ok(big_int) => big_int,
            Err(_) => continue,
        };

        if hash.cmp(&target) == std::cmp::Ordering::Less {
            block.hash = hash;

            block.nonce = nonce;
            return;
        }

        nonce += 1;
    }
}

pub fn validate(block: &mut Block) -> Result<(), &'static str> {
    let mut data = block.get_headers();
    data.extend(&block.nonce.to_be_bytes());

    let hash = match create_bigint_from_hash(&digest(data.as_slice())) {
        Ok(big_int) => big_int,
        Err(_) => {
            return Err("Could not create BigInt from hash!");
        }
    };

    if hash.cmp(&get_target(DIFFICULTY!())) == std::cmp::Ordering::Less {
        return Ok(());
    }

    Err("Block hash is not valid!")
}
