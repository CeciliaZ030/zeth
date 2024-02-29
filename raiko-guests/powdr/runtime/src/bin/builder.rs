use std::{env, fs, path::Path};

use powdr::riscv::{compile_rust, CoProcessors};

fn main() {
    let mut args = env::args();
    args.next();
    let path_to_cargo_toml = args
        .next()
        .unwrap_or_else(|| "../artifact/Cargo.toml".to_string());
    let output_dir = args.next().unwrap_or_else(|| "../".to_string());

    let path = Path::new(&path_to_cargo_toml);
    dbg!(path);
    dbg!(path.exists());
    let st = fs::read_to_string(path).unwrap();
    dbg!(st);

    dbg!("Compiling Rust...");
    let Some(_) = compile_rust(
        &path_to_cargo_toml,
        Path::new(&output_dir),
        true,
        &CoProcessors::base().with_poseidon(),
        // use bootloader
        false,
    ) else {
        panic!("Could not compile Rust");
    };
    dbg!("Compilation done.");
}
