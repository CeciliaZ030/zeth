use powdr::riscv::{compile_rust, CoProcessors};
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();
    println!("Compiling Rust...");
    let (asm_file_path, asm_contents) = compile_rust(
        "./guest-powdr",
        Path::new("/tmp/test"),
        true,
        &CoProcessors::base().with_poseidon(),
        true,
    )
    .ok_or_else(|| vec!["could not compile rust".to_string()])
    .unwrap();
}
