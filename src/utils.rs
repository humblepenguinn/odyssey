use num_bigint::BigInt;

#[inline]
pub fn create_bigint_from_hash(hashed_string: &str) -> Result<BigInt, &'static str> {
    // Parse the hashed string as a base-16 (hexadecimal) value
    let parsed_value = BigInt::parse_bytes(hashed_string.as_bytes(), 16);

    match parsed_value {
        Some(big_int) => Ok(big_int),
        None => Err("Failed to parse the hashed string as a BigInt"),
    }
}

#[inline]
pub fn create_bigint_from_bytes(bytes: &[u8]) -> BigInt {
    BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes)
}

#[inline]
pub fn bigint_to_string(big_int: &BigInt) -> String {
    big_int.to_string()
}

#[cfg(debug_assertions)]
#[inline]
pub fn print_hash(hash: &BigInt) {
    println!("{:064x}", hash);
}
