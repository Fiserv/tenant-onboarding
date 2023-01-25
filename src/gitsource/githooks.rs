use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json::Value;
use reqwest::{Client, Method};

//const GITHUB_TOKENT:&str = "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";
//const GITHUB_REPO_HOOKS_API:&str = "https://api.github.com/repos/Fiserv/SampleOnBoardingTenant/hooks";
 
 
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
pub async fn add_hooks_repo(config_yaml: &Vec<Yaml>, settings_yaml: &Vec<Yaml>) -> Result<(bool), Box<dyn Error>> {

    let mut added = false;
    //let mut github_repo_hooks_api = String::new();

    let config = &config_yaml[0]; 
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();
 
    let setting = &settings_yaml[0]; 
    let github_token = setting["github"]["gitHubAuthToken"].as_str().unwrap();
   
    let dev_hook = setting["github"]["gitHubDevHook"].as_str().unwrap();
    let dev_hook_key = setting["github"]["gitHubDevHookKey"].as_str().unwrap();

    let qa_hook = setting["github"]["gitHubQAHook"].as_str().unwrap();
    let qa_hook_key = setting["github"]["gitHubQAHookKey"].as_str().unwrap();

    let stage_hook = setting["github"]["gitHubStageHook"].as_str().unwrap();
    let stage_hook_key = setting["github"]["gitHubStageHookKey"].as_str().unwrap();

    let prod_hook = setting["github"]["gitHubProdHook"].as_str().unwrap();
    let prod_hook_key = setting["github"]["gitHubProdHookKey"].as_str().unwrap();
       
    added =  add_hooks(dev_hook , dev_hook_key , tenant_repo, settings_yaml).await?; 

    added =  add_hooks(qa_hook , qa_hook_key ,tenant_repo, settings_yaml).await?; 

    added =  add_hooks(stage_hook , stage_hook_key  ,tenant_repo, settings_yaml).await?; 

    added =  add_hooks(prod_hook , prod_hook_key  ,tenant_repo, settings_yaml).await?; 
 
    Ok((added))
}

//#[tokio::main]
async fn add_hooks(path: &str , key: &str ,tenant_repo: &str, setting_yaml: &Vec<Yaml>) ->  Result<(bool), Box<dyn Error>> {


    let setting = &setting_yaml[0]; 
    let github_token = setting["github"]["gitHubAuthToken"].as_str().unwrap();
    let github_api = setting["github"]["gitHubAPIRepo"].as_str().unwrap(); 
    let github_repo_hooks_api = format!("{}{}{}", github_api.to_string(), tenant_repo.to_string() , "/hooks".to_string());
 
    //println!("github_repo_hooks_api: {} ", github_repo_hooks_api);
    let mut check = false;
    let github_client = reqwest::Client::new();

    let hoook_config = HooksConfig{
        url:          path.to_string(),
        content_type: "json".to_string(),
        insecure_ssl: "0".to_string(),
        secret:       key.to_string()
    };

    let repo_hook_data = RepoHooksInfo {
            name:   "web".to_string(),
            active: true, 
            events: ["push".to_string()],
            config: hoook_config
    };      


    let post_req = github_client.request(Method::POST, github_repo_hooks_api)
    .bearer_auth(github_token)
    .header("User-Agent", "tenant-onbaording")
    .header("Accept", "application/vnd.github+json")
    .timeout(Duration::from_secs(5))
    .json(&repo_hook_data);

    let resp_data = post_req.send().await?; 

   // println!("resp_data: {:#?} ", resp_data);
       
    if (resp_data.status() == reqwest::StatusCode::CREATED) {
        let res_body = resp_data.bytes().await?; 
        let vec_body = res_body.to_vec();
        let res_str = String::from_utf8_lossy(&vec_body);
        println!("{} : Hook Added", path );
        check = true;
    }
  

    Ok((check))
}

