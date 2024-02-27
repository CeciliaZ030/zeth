pub use anyhow;
pub use zeth_lib::taiko::GuestInput;

/// A driver is a trait that defines the interface for a prover driver.
/// The driver is responsible for handling the I/O between the host and the prover,
/// as well as for handling the execution of the prover guest in order to provide a
/// output to the host.
///
/// # NOTE
/// The input paramater should be a [zeth_lib::taiko::GuestInput] type, which is a type
/// alias for
///
/// # Examples
/// Each driver should be callable like:
///
/// ```rust
/// use example_driver::ExampleDriver;
/// use raiko_drivers::Driver;
///
/// ...
/// let output = driver.execute(input).await?;
/// ...
/// ```
pub trait Driver {
    type Output;

    /// Execute the prover guest with the given input and return the output.
    ///
    /// [input] is a [zeth_lib::taiko::GuestInput] type, which is a wrapper
    /// around the minimum required data for block creation [zeth_lib::input::Input]
    /// and the Taiko system information [zeth_lib::taiko::TaikoSystemInfo],
    /// where the input generic is
    /// [zeth_primitives::transactions::ethereum::EthereumTxEssence].
    async fn execute(&self, input: GuestInput) -> anyhow::Result<Self::Output>;
}
