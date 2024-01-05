#![no_std]

use zeth_lib::{
    builder::{BlockBuilderStrategy, EthereumStrategy},
    /* consts::ETH_MAINNET_CHAIN_SPEC,  */input::Input,
};
// use powdr::{print, coprocessors::{get_data, get_data_len}};

#[no_mangle]
fn main() {
    // Build the resulting block
    // let (header, state) = EthereumStrategy::build_from(&ETH_MAINNET_CHAIN_SPEC, Input::default())
    //     .expect("Failed to build the resulting block");
}

