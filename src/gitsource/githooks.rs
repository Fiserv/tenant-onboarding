use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json::Value;
use reqwest::{Client, Method};

const GITHUB_TOKENT:&str = "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";
const GITHUB_REPO_HOOKS_API:&str = "https://api.github.com/repos/Fiserv/SampleOnBoardingTenant/hooks";
 
 
#[derive(Serialize, Deserialize, Debug)] 
struct RepoHooksInfo { 
    name:String,
    active: bool,
    events:[String ;1],
    config:HooksConfig
}

#[derive(Serialize, Deserialize, Debug)]
struct HooksConfig{
        url:String,
        content_type:String,
        insecure_ssl:String,
        secret:String
}

#[tokio::main]
pub async fn add_hooks_repo(yaml: &Vec<Yaml>) -> Result<(), Box<dyn Error>> {

    let y = &yaml[0]; 
    let tenant_repo = y["github"]["repoName"].as_str().unwrap();
    println!("wehbooks_tenant_repo {:#?}", tenant_repo);

 let hook_config_data = HooksConfig{
    url:"https://qa-developer.fiserv.com/api/github-push".to_string(),
    content_type:"json".to_string(),
    insecure_ssl:"0".to_string(),
    secret:"secret123".to_string()
 };

 let repo_hook_data = RepoHooksInfo {
        name: "web".to_string(),
        active:true, 
        events: ["push".to_string()],
        config: hook_config_data
 };      
  
    let github_client = reqwest::Client::new();
    let post_req = github_client.request(Method::POST, GITHUB_REPO_HOOKS_API)
    .bearer_auth(GITHUB_TOKENT)
    .header("User-Agent", "tenant-onbaording")
    .header("Accept", "application/vnd.github+json")
    .timeout(Duration::from_secs(5))
    .json(&repo_hook_data);

    let resp_data = post_req.send().await?; 

    println!("Status {}", resp_data.status());
    //if (resp_data.status() == reqwest::StatusCode::CREATED) 
    let res_body = resp_data.bytes().await?;

    let v = res_body.to_vec();
    let s = String::from_utf8_lossy(&v);
    println!("response: {} ", s);
 
    Ok(())
}

