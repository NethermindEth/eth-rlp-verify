use eth_rlp_verify::test_helpers::create_test_block_header_cancun;
use eth_rlp_verify::{verify_block, CHAIN_ID_MAINNET};

fn main() {
    let block_header = create_test_block_header_cancun();

    let block_hash = "0xc30bad27d3bcaece0a3676bbf0cfd3f2711d8e9bf82b03d6c8eaf7d38cc26218";

    let is_valid = verify_block(21360407, block_header, block_hash, CHAIN_ID_MAINNET);
    println!("Cancun era Mainnet block verification result: {}", is_valid);
}
