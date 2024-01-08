use powdr::number::GoldilocksField;
use powdr::riscv::{compile_rust, CoProcessors};
use powdr::pipeline::{
    pipeline::Pipeline,
    test_util::verify_pipeline
};
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();
    println!("Compiling Rust...");
    let (asm_file_path, asm_contents) = compile_rust(
        "./guest-powdr/Cargo.toml",
        Path::new("/tmp/test"),
        true,
        &CoProcessors::base().with_poseidon(),
        true,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();
    let pipeline: Pipeline<GoldilocksField> =
        Pipeline::default().from_asm_string(asm_contents, Some(PathBuf::from(asm_file_path)));
    verify_pipeline(pipeline, Vec::new(), Vec::new());
}
