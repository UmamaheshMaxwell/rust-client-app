use reqwest;
use tokio::time::Duration;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let doge = client
        .get("https://node-api-service-3imv474m7a-uc.a.run.app/api/user")
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("{:}", doge);
    Ok(())
}