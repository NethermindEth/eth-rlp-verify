use eth_rlp_verify::test_helpers::create_test_block_header_pectra;
use eth_rlp_verify::{verify_block, CHAIN_ID_SEPOLIA};

fn main() {
    let block_header = create_test_block_header_pectra();

    let block_hash = "0xbfa14ad39de89b0de89a0d9e78efebae792eae93ee46414ed98ee790ce8ed8b3";

    let is_valid = verify_block(7839744, block_header, block_hash, CHAIN_ID_SEPOLIA);
    println!("Pectra era Sepolia block verification result: {}", is_valid);
}
