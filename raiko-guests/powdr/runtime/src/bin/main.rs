use std::{env, path::Path};

use common::CHANNEL;
use powdr::{
    pipeline::test_util::verify_pipeline,
    riscv::{compile_rust, CoProcessors},
    GoldilocksField, Pipeline,
};

fn main() {
    let mut args = env::args();
    let input = args.next().map_or_else(
        || panic!("Did not receive any input"),
        |s| s.as_bytes().to_vec(),
    );

    dbg!("Compiling Rust...");
    // TODO: Petar - this could probably be precompiled so that we don't have to compile a
    // binary each run. how can we make this more ergonomic? maybe adding a build.rs which
    // compiles the artifact to a predetermined location and we just read from there?
    let (asm_file_path, asm_contents) = compile_rust(
        "../../artifact/Cargo.toml",
        Path::new("/tmp/test"),
        true,
        &CoProcessors::base().with_poseidon(),
        // use bootloader
        false,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();
    dbg!("Compilation done.");

    dbg!("Creating pipeline...");
    let pipeline =
        Pipeline::<GoldilocksField>::default().from_asm_string(asm_contents, Some(asm_file_path));
    dbg!("Pipeline done.");

    dbg!("Adding prover inputs to pipeline...");
    let pipeline = pipeline.add_data(CHANNEL, &input);

    dbg!("Verifying pipeline...");
    verify_pipeline(pipeline);

    dbg!("Verification done.");

    // TODO: Petar - where does the proof go? we should print it to stdout so that the driver
    // can consume it
    println!("Success!");
}
