#![deny(unused_crate_dependencies)]

pub mod constants;
pub mod eras;
pub mod test_helpers;
pub mod traits;
use eth_rlp_types::BlockHeader as VerifiableBlockHeader;
use tracing::error;

pub const CHAIN_ID_MAINNET: u64 = 1;
pub const CHAIN_ID_SEPOLIA: u64 = 11155111;

pub fn are_blocks_and_chain_valid(block_headers: &[VerifiableBlockHeader], chain_id: u64) -> bool {
    for (i, block) in block_headers.iter().enumerate() {
        let block_hash = block.block_hash.clone();
        let parent_hash = block.parent_hash.clone().unwrap_or_default();
        let block_number = block.number;

        let is_valid = verify_block(block_number as u64, block.clone(), &block_hash, chain_id);

        if !is_valid {
            return false;
        }

        if i != 0 {
            let previous_block = &block_headers[i - 1];
            let previous_block_hash = previous_block.block_hash.clone();

            if parent_hash != previous_block_hash {
                error!(
                    "Chain validation failed: parent hash mismatch at block {}: expected {}, got {}",
                    block_number, previous_block_hash, parent_hash
                );
                return false;
            }
        }
    }

    true
}

pub fn are_blocks_valid(block_headers: &[VerifiableBlockHeader], chain_id: u64) -> bool {
    for block in block_headers.iter() {
        let block_hash = block.block_hash.clone();
        let block_number = block.number;

        let is_valid = verify_block(block_number as u64, block.clone(), &block_hash, chain_id);

        if !is_valid {
            return false;
        }
    }

    true
}

/// Verifies the validity of an Ethereum block header based on the block number and expected hash.
///
/// This function determines the appropriate Ethereum era based on the block number, retrieves the corresponding
/// verification function, and verifies the block header by comparing its computed hash with the expected block hash.
/// The verification process ensures that the block is authentic and belongs to the correct place in the blockchain.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the block number of the block being verified.
/// - `block_header`: A `VerifiableBlockHeader` struct containing the block header data that needs to be verified.
/// - `block_hash`: A string slice representing the expected hash of the block.
///
/// # Returns
///
/// A `bool` indicating whether the block header is valid.
pub fn verify_block(
    block_number: u64,
    block_header: VerifiableBlockHeader,
    block_hash: &str,
    chain_id: u64,
) -> bool {
    match eras::determine_era(block_number, chain_id) {
        Some(verify_fn) => verify_fn(block_hash.to_string(), block_header),
        None => false,
    }
}

/// Encodes an Ethereum block header into RLP format.
///
/// This function determines the correct era based on the block number and encodes
/// the block header accordingly.
///
/// # Returns
///
/// An `Option<Vec<u8>>` containing the RLP-encoded block header data if successful.
pub fn encode_block_header(
    block_number: u64,
    block_header: VerifiableBlockHeader,
    chain_id: u64,
) -> Option<Vec<u8>> {
    eras::determine_era_encoder(block_number, chain_id).map(|encoder| encoder(block_header))
}

/// Decodes an RLP-encoded block header based on the block number.
///
/// This function determines the correct era for the given block number and decodes the RLP-encoded
/// data into a `VerifiableBlockHeader`. If the block number is not within a recognized era or decoding
/// fails, it returns `None`.
///
/// # Arguments
///
/// - `block_number`: A `u64` representing the block number of the block being decoded.
/// - `encoded`: A byte slice containing the RLP-encoded block header data.
///
/// # Returns
///
/// An `Option<VerifiableBlockHeader>` containing the decoded block header if successful.
pub fn decode_block_header(
    block_number: u64,
    encoded: &[u8],
    chain_id: u64,
) -> Option<VerifiableBlockHeader> {
    eras::determine_era_decoder(block_number, chain_id).and_then(|decoder| decoder(encoded).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::create_test_block_header_london; // Adjust import as needed

    #[test]
    #[ignore]
    fn test_block_header_encoding_decoding_debug() {
        // Create the test block header for London era
        let original_header = create_test_block_header_london();
        let block_number = original_header.number as u64;

        println!("Original Header: {:#?}", original_header);

        // Step 1: Encode the block header
        let encoded = encode_block_header(block_number, original_header.clone(), CHAIN_ID_MAINNET)
            .expect("Encoding failed");

        println!("Encoded Bytes: {:?}", encoded);

        // Debug: Reprint encoded bytes as hex
        let encoded_hex: String = encoded.iter().map(|b| format!("{:02x}", b)).collect();
        println!("Encoded Hex: {}", encoded_hex);

        // Step 2: Decode the block header
        let decoded_header =
            decode_block_header(block_number, &encoded, CHAIN_ID_MAINNET).expect("Decoding failed");

        println!("Decoded Header: {:#?}", decoded_header);

        // Step 3: Compare individual fields for discrepancies
        if original_header.parent_hash != decoded_header.parent_hash {
            println!(
                "Mismatch in parent_hash:\nOriginal: {}\nDecoded: {}",
                original_header.parent_hash.clone().unwrap_or_default(),
                decoded_header.parent_hash.clone().unwrap_or_default()
            );
        }
        assert_eq!(
            original_header.parent_hash, decoded_header.parent_hash,
            "Mismatch in parent_hash"
        );

        assert_eq!(
            original_header.nonce, decoded_header.nonce,
            "Mismatch in nonce"
        );

        assert_eq!(
            original_header.base_fee_per_gas, decoded_header.base_fee_per_gas,
            "Mismatch in base_fee_per_gas"
        );

        assert_eq!(
            original_header, decoded_header,
            "The original and decoded headers do not match"
        );
    }
}
