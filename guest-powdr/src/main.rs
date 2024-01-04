use zeth_lib::{
    builder::{BlockBuilderStrategy, EthereumStrategy},
    consts::ETH_MAINNET_CHAIN_SPEC,
};
pub fn main() {
    // Build the resulting block
    let (header, state) = EthereumStrategy::build_from(&ETH_MAINNET_CHAIN_SPEC, Input::default())
        .expect("Failed to build the resulting block");
}

