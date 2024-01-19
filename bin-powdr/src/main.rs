use powdr::number::GoldilocksField;
use powdr::riscv::{compile_rust, CoProcessors};
use powdr::pipeline::{
    pipeline::Pipeline,
    test_util::verify_pipeline
};
use std::path::{Path, PathBuf};

// Step1: set PILCOM to path (https://github.com/0xPolygonHermez/pilcom)
// Step2: cargo run --package bin-powdr --bin bin-powdr --all-features 
///
/// Note:
/// revm dependencies located in workspace as well as in the guest-powdr/Cargo.toml
/// revm = { git = "some-repo"}
/// revm-primitives = { git = "some-repo",  package = "revm-primitives" }
/// "some-repo" has to be the same.
fn main() {
    env_logger::init();
    println!("Compiling Rust...");
    let (asm_file_path, asm_contents) = compile_rust(
        "./guest-powdr/Cargo.toml",
        Path::new("/tmp/test"),
        true,
        &CoProcessors::base().with_poseidon(),
        /*use bootloader*/ false,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();
    println!("Compilation done.");
    println!("Creating pipeline...");
    let pipeline: Pipeline<GoldilocksField> =
        Pipeline::default().from_asm_string(asm_contents, Some(PathBuf::from(asm_file_path)));
    println!("Pipeline done.");
    println!("Verifying pipeline...");
    verify_pipeline(pipeline, Vec::new(), Vec::new());
    println!("Verification done.");
}
