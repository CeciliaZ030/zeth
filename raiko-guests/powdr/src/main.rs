#![no_std]

extern crate alloc;

use powdr_riscv_runtime::{self, coprocessors::get_data_serde};
use zeth_lib::{
    builder::{BlockBuilderStrategy, TaikoStrategy},
    consts::TKO_MAINNET_CHAIN_SPEC,
    taiko::{
        protocol_instance::{assemble_protocol_instance, EvidenceType},
        GuestInput,
    },
};

#[no_mangle]
fn main() {
    let GuestInput { input, sys_info } = get_data_serde::<GuestInput>(42);

    let (header, _mpt_node) = TaikoStrategy::build_from(&TKO_MAINNET_CHAIN_SPEC.clone(), input)
        .expect("Failed to build the resulting block");

    let pi = assemble_protocol_instance(&sys_info, &header)
        .expect("Failed to assemble the protocol instance");

    let pi_hash = pi.instance_hash(EvidenceType::Powdr);

    powdr_riscv_runtime::print!("{pi_hash}");
}
