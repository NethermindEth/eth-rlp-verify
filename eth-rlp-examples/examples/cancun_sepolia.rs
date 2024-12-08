use eth_rlp_verify::test_helpers::create_test_block_header_cancun_sepolia;
use eth_rlp_verify::{verify_block, CHAIN_ID_SEPOLIA};

fn main() {
    let block_header = create_test_block_header_cancun_sepolia();

    let block_hash = "0x3919e733febe493f7c84bcc03629dada9c1995ab45a92503747a52feef24ac31";

    let is_valid = verify_block(5187062, block_header, block_hash, CHAIN_ID_SEPOLIA);
    println!("Cancun era Sepolia block verification result: {}", is_valid);
}
