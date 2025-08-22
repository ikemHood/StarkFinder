use crate::libs::error::ApiError;
use starknet::core::{types::Felt, utils::normalize_address};
use std::str::FromStr;

// Validate and normalize using Starknet's canonical utility.
// Returns a 0x-prefixed lowercased address string on success.
pub fn normalize_and_validate(wallet: &str) -> Result<String, ApiError> {
    let w = wallet.trim();
    let felt = Felt::from_str(w).map_err(|_| ApiError::BadRequest("invalid wallet address"))?;
    let norm = normalize_address(felt);
    Ok(format!("{:#x}", norm))
}
