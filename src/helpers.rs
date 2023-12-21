extern crate tiny_keccak;
use tiny_keccak::{Hasher, Keccak};

use num_bigint::BigUint;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sort_characters(input: &str) -> Result<String, serde_json::Error> {
    let mut cvec = input.chars().collect::<Vec<char>>();
    cvec.sort_unstable();

    Ok(cvec.iter().collect())
}

pub fn keccak256(input: &str) -> BigUint {
    let mut hasher = Keccak::v256();

    hasher.update(input.as_bytes());
    let mut output = [0u8; 32];

    hasher.clone().finalize(&mut output);

    return BigUint::from_bytes_be(&output);
}

pub fn get_current_timestamp() -> Option<u64> {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Some(duration.as_secs()),
        Err(_) => None,
    }
}
