use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json::Value;
use reqwest::{Client, Method};


const GITHUB_API:&str ="https://api.github.com/orgs/Fiserv/teams";
const GITHUB_TOKENT:&str = "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";
const GITHUB_REPO_GEN_API:&str = "https://api.github.com/repos/Fiserv/sample-tenant-repo/generate";
pub fn function() {
    println!("called `gitutils::function()`");
}
 
#[tokio::main]
pub async fn get_github_team(yaml: &Vec<Yaml>) -> Result<(), Box<dyn Error>> {

    let y = &yaml[0]; 
    let tenant_repo = y["tenantName"].as_str().unwrap();
 //   println!("tenant repo {:?}", tenant_repo);
 let github_client = reqwest::Client::new();
    let github_data = github_client
        .get(GITHUB_API)
        .bearer_auth(GITHUB_TOKENT)
        .header("User-Agent", "tenant-onbaording")
        .timeout(Duration::from_secs(3))
        .send()
        .await?
        .text()
        .await?;
                
    Ok(())
}


#[derive(Serialize, Deserialize, Debug)] 
struct Data { 
    owner:String,
    name: String,
    description: String,
    private: bool,
    include_all_branches:bool 
}

#[tokio::main]
pub async fn create_repo(yaml: &Vec<Yaml>) -> Result<(), Box<dyn Error>> {

    let y = &yaml[0]; 
    let tenant_repo = y["tenantName"].as_str().unwrap();
    let input = r#"{ 
        "owner":"Fiserv", 
        "name": "TestRepo",
        "description": "This is a test repo generated from template repository via Rust",
        "private": false,
        "include_all_branches":true  
    }"#;

    let mut object: Data = serde_json::from_str(input).unwrap();
    
    object.name = tenant_repo.to_string();
     
    println!("Setting new Tenant Repo {:#?}", object);

    let github_client = reqwest::Client::new();
    let post_req = github_client.request(Method::POST, GITHUB_REPO_GEN_API)
    .bearer_auth(GITHUB_TOKENT)
    .header("User-Agent", "tenant-onbaording")
    .header("Accept", "application/vnd.github+json")
    .timeout(Duration::from_secs(5))
    .json(&object);

    let resp_data = post_req.send().await?; 

    println!("Status {}", resp_data.status());
    //if (resp_data.status() == reqwest::StatusCode::CREATED) 
    let res_body = resp_data.bytes().await?;

    let v = res_body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("response: {} ", s);
 
    Ok(())
}
