#![no_std]

use ruint::uint;
use zeth_lib::{
    builder::{BlockBuilderStrategy, EthereumStrategy, 
        ChainSpec, Eip1559Constants, ForkCondition},
    /* consts::ETH_MAINNET_CHAIN_SPEC,  */
    input::Input,
};
use zeth_primitives::transactions::signature::TxSignature;
pub use zeth_primitives::transactions::{ethereum::{EthereumTxEssence, TxEssenceEip1559}, Transaction};
use revm::primitives::SpecId;

extern crate alloc;
use alloc::{collections::BTreeMap, vec};
// use powdr::{print, coprocessors::{get_data, get_data_len}};

#[no_mangle]
fn main() {
    // Build the resulting block
    let eth_mainnet = ChainSpec {
        chain_id: 1,
        hard_forks: BTreeMap::from([
            (SpecId::FRONTIER, ForkCondition::Block(0)),
            // previous versions not supported
            (SpecId::MERGE, ForkCondition::Block(15537394)),
            (SpecId::SHANGHAI, ForkCondition::Block(17034870)),
            (SpecId::CANCUN, ForkCondition::TBD),
        ]),
        eip_1559_constants: Eip1559Constants {
            base_fee_change_denominator: uint!(8_U256),
            base_fee_max_increase_denominator: uint!(8_U256),
            base_fee_max_decrease_denominator: uint!(8_U256),
            elasticity_multiplier: uint!(2_U256),
        },
    };
    let input = Input::<EthereumTxEssence> {
        transactions: vec![
            Transaction {
                essence: EthereumTxEssence::Eip1559(TxEssenceEip1559::default()),
                signature: TxSignature::default(),
            }
        ],
        parent_header: Default::default(),
        beneficiary: Default::default(),
        gas_limit: Default::default(),
        timestamp: Default::default(),
        extra_data: Default::default(),
        mix_hash: Default::default(),
        withdrawals: Default::default(),
        parent_state_trie: Default::default(),
        parent_storage: Default::default(),
        contracts: Default::default(),
        ancestor_headers: Default::default(),
    };
    let (header, state) = EthereumStrategy::build_from(&eth_mainnet, input)
        .expect("Failed to build the resulting block");
}

