use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("graph", "abi/graph.json")?
        .generate()?
        .write_to_file("src/abi/graph.rs")?;
    Abigen::new("rewards", "abi/rewards.json")?
        .generate()?
        .write_to_file("src/abi/rewards.rs")?;
    Abigen::new("transfer", "abi/transfer.json")?
        .generate()?
        .write_to_file("src/abi/transfer.rs")?;
    Ok(())
}