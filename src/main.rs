mod config;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::from_env()?;
    println!("{:#?}", cfg);
    // Later: pass cfg into GitHub client and metrics modules
    Ok(())
}