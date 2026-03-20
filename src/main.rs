mod api;
mod cli;
mod commands;
mod error;
mod models;

use commands::search::run_search;
use error::KitaabeError;

#[tokio::main]
async fn main() -> Result<(), KitaabeError> {
    let query = cli::get_query()?;
    run_search(&query).await?;
    Ok(())
}
