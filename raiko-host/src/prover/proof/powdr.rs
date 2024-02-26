use std::{
    path::{Path, PathBuf},
    str,
};

use powdr::{
    pipeline::test_util::verify_pipeline,
    riscv::{compile_rust, CoProcessors},
    GoldilocksField, Pipeline,
};
use serde::Serialize;
use serde_json::Value;
use tokio::process::Command;
use tracing::{debug, info};
use zeth_lib::{
    consts::{ChainSpec, TKO_MAINNET_CHAIN_SPEC},
    input::Input,
    taiko::{host::HostArgs, TaikoSystemInfo},
    EthereumTxEssence,
};
use zeth_primitives::{Address, B256};

use crate::prover::{
    context::Context,
    error::{Error, Result},
    request::SgxRequest,
};

#[derive(Serialize, Default)]
struct InputData {
    host_args: HostArgs,
    l2_chain_spec: ChainSpec,
    testnet: String,
    l2_block_no: u64,
    graffiti: B256,
    prover: Address,
}

pub async fn execute_powdr(ctx: &Context, req: &SgxRequest) -> Result<(), Error> {
    println!("Compiling Rust...");
    let (asm_file_path, asm_contents) = compile_rust(
        "/raiko-guests/powdr/Cargo.toml",
        Path::new("/tmp/test"),
        true,
        &CoProcessors::base().with_poseidon(),
        // use bootloader
        false,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();
    println!("Compilation done.");

    println!("Creating pipeline...");
    let pipeline: Pipeline<GoldilocksField> = Pipeline::default()
        .from_asm_string(asm_contents, Some(PathBuf::from(asm_file_path)))
        .with_prover_inputs(vec![]);
    println!("Pipeline done.");

    println!("Passing inputs...");
    let prover_inputs = serde_cbor::to_vec(&InputData {
        host_args: HostArgs {
            l1_cache: ctx.l1_cache_file.clone(),
            l1_rpc: Some(req.l1_rpc),
            l2_cache: ctx.l2_cache_file.clone(),
            l2_rpc: Some(req.l2_rpc),
        },
        l2_chain_spec: TKO_MAINNET_CHAIN_SPEC.clone(),
        testnet: ctx.l2_chain,
        l2_block_no: req.block,
        graffiti: req.graffiti,
        prover: req.prover,
    })
    .unwrap_or_else(|| vec!["could not serialize inputs".to_string()]);
    pipeline.add_data(42, &prover_inputs);

    println!("Verifying pipeline...");
    verify_pipeline(pipeline);

    println!("Verification done.");
    Ok(())
}
// phoebe@cecilia-gz:~/projects/zeth$
//  cargo +nightly build --release -Z build-std=core,alloc --target
// riscv32imac-unknown-none-elf --lib --manifest-path ./raiko-guest/Cargo.toml
