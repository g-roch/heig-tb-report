use reqwest;

use client::voting_choose;
use lib::VoteOption;
use lib::BOARD_PORT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: Vec<VoteOption> = reqwest::get(format!("http://localhost:{}/options", BOARD_PORT))
        .await?
        .json()
        .await?;
    voting_choose(&options)?;
    Ok(())
}
