mod api;
mod cli;
mod commands;
mod error;
mod models;

use commands::search::run_search;
use error::KitaabeError;

#[tokio::main]
async fn main() -> Result<(), KitaabeError> {
    let query = match cli::get_query() {
        Ok(q) => q,
        Err(_) => {
            println!("Usage: kitaabe \"search query\"");
            return Ok(());
        }
    };

    run_search(&query).await?;
    Ok(())
}
