#![no_std]

extern crate alloc;
use alloc::{collections::BTreeMap, string::ToString, vec};

use powdr_riscv_runtime::{self, coprocessors::get_data_serde};
use serde::Deserialize;
use zeth_lib::{
    builder::{BlockBuilderStrategy, TaikoStrategy},
    consts::{ChainSpec, TKO_MAINNET_CHAIN_SPEC},
    input::Input,
    taiko::{
        host::{init_taiko, HostArgs},
        protocol_instance::{assemble_protocol_instance, EvidenceType},
        TaikoSystemInfo,
    },
    EthereumTxEssence,
};
use zeth_primitives::U256;

#[derive(Deserialize)]
struct InputData {
    host_args: HostArgs,
    l2_chain_spec: ChainSpec,
    testnet: String,
    l2_block_no: u64,
    graffiti: B256,
    prover: Address,
}

#[no_mangle]
fn main() {
    let InputData {
        host_args,
        l2_chain_spec,
        testnet,
        l2_block_no,
        graffiti,
        prover,
    } = get_data_serde::<InputData>(42);

    let (input, sys_info) = init_taiko(
        host_args,
        l2_chain_spec,
        &testnet,
        l2_block_no,
        graffiti,
        prover,
    )
    .unwrap();

    let (header, _mpt_node) = TaikoStrategy::build_from(&TKO_MAINNET_CHAIN_SPEC.clone(), input)
        .expect("Failed to build the resulting block");

    let pi = assemble_protocol_instance(&sys_info, &header)
        .expect("Failed to assemble the protocol instance");

    let pi_hash = pi.instance_hash(EvidenceType::Powdr);

    powdr_riscv_runtime::print!("{pi_hash}");
}
