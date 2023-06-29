use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Apecoin", "abi/apecoin.json")?
        .generate()?
        .write_to_file("src/abi/apecoin.rs")?;

    Ok(())
}
