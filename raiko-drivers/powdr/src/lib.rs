use std::{ffi::OsString, os::unix::ffi::OsStringExt, path::PathBuf};

use driver_common::{
    anyhow::{ensure, Result},
    Driver, GuestInput,
};

/// A driver for the Powdr guest prover.
pub struct PowdrDriver {
    powdr_guest_path: PathBuf,
}

impl PowdrDriver {
    /// Create a new Powdr driver given a path to the powdr guest binary.
    pub fn new(powdr_guest_path: PathBuf) -> Result<Self> {
        ensure!(
            powdr_guest_path.exists(),
            "Powdr guest binary does not exist"
        );
        Ok(Self { powdr_guest_path })
    }

    /// Run the Powdr guest binary with the given input.
    async fn run_with_input(&self, input: Vec<u8>) -> Result<String> {
        let output = tokio::process::Command::new(&self.powdr_guest_path)
            .arg(OsString::from_vec(input))
            .output()
            .await?;

        ensure!(
            output.status.success(),
            "Powdr guest binary failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl Driver for PowdrDriver {
    type Output = String;

    async fn execute(&self, input: GuestInput) -> Result<Self::Output> {
        let prover_inputs = serde_cbor::to_vec(&input).expect("Could not serialize inputs");

        self.run_with_input(prover_inputs).await?;

        Ok("Success!".to_string())
    }
}
