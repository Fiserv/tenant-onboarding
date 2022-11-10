use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use reqwest;
use std::error::Error;
use std::time::Duration;

pub fn function() {
    println!("called `gitutils::function()`");
}
 
#[tokio::main]
pub async fn get_github_team() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let github_data = client
        .get("https://api.github.com/orgs/Fiserv/teams")
        .bearer_auth("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")
        .header("User-Agent", "tenant-onbaording")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
    println!("Github Team --- {:}", github_data);
    Ok(())
}