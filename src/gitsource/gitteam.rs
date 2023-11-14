use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json::Value;
use reqwest::{Client, Method};
use crate::gitsource;

#[derive(Serialize, Deserialize, Debug)] 
struct TeamInfo { 
    name: String,
    description: String,
    permission:String,
    privacy:String,
    repo_names:[String ;1],
    maintainers:Vec<std::string::String>
}

#[derive(Serialize, Deserialize, Debug)]
struct TeamInfoWm { 
    name: String,
    description: String,
    permission:String,
    privacy:String,
    repo_names:[String ;1]
}


#[tokio::main]
pub async fn process_github_team(config_yaml: &Vec<Yaml> , settings_yaml: &Vec<Yaml>) -> Result<(bool), Box<dyn Error>> {

    let mut team_added = false;
    
    let config = &config_yaml[0]; 
    let tenant_team = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap(); 
    let tenant_members = config["GitHub_essentials"]["Team_Members"].as_str().unwrap();
   
    let setting = &settings_yaml[0];
    let github_api = setting["github"]["gitHubAPIUrl"].as_str().unwrap();
    let github_auth_token_result = gitsource::authtoken::get_auth_token(setting);
    if !github_auth_token_result.is_ok() {
        return Result::Err(github_auth_token_result.err().unwrap());
    }
    let github_auth = github_auth_token_result.unwrap();
    let github_owner = setting["github"]["gitHubSourceOwner"].as_str().unwrap();

    let github_teams_api = format!("{}/{}", github_api.to_string(), tenant_team.to_string().to_lowercase());

    let github_client = reqwest::Client::new();

    let get_req = github_client.request(Method::GET, github_teams_api.clone())
                             .bearer_auth(github_auth.clone())
                             .header("User-Agent", "tenant-onbaording")
                             .header("Accept", "application/vnd.github+json")
                             .header("X-GitHub-Api-Version" , "2022-11-28")
                             .timeout(Duration::from_secs(3));

     let github_data = get_req.send().await?; 

     let mut my_vec = Vec::<std::string::String>::new();  
     let maintainerslist: Vec<&str> = tenant_members.split(",").collect(); 
     for item in maintainerslist.clone() {
        my_vec.push(item.trim().to_string()); 
     } 

    let teams_data = TeamInfo { 
        name: tenant_team.to_string(),
        description: "A new team group generated by DevStudio team".to_string() ,
        permission:"push".to_string(),  
        privacy:"closed".to_string(),
        repo_names: [format!("{}/{}", github_owner, tenant_repo).to_string()],
        maintainers :my_vec,
        };
        println!("Team stats {} " , github_data.status());
      /* Checking: if Team already exists in the GitHub: If yes then just add into the Tenant Repo */
    if (github_data.status()  == reqwest::StatusCode::OK){
        // Just add team to the Tenant repo
       let put_req_api = format!("{}/{}/{}/{}/{}" , github_api , tenant_team.to_string().to_lowercase(),"repos" , github_owner,tenant_repo );
      
        let put_req = github_client.request(Method::PUT, put_req_api)
                                    .bearer_auth(github_auth.clone())
                                    .header("User-Agent", "tenant-onbaording")
                                    .header("Accept", "application/vnd.github+json")
                                    .header("X-GitHub-Api-Version" , "2022-11-28")
                                    .timeout(Duration::from_secs(5));

        let github_data_stats = put_req.send().await?;  
        
        if (github_data_stats.status() == reqwest::StatusCode::NO_CONTENT){
            println!(" github_data_stats : {} " , github_data_stats.status()); 
            team_added = true; 
        }else{
            println!(" Unable to add Team : {} " , github_data_stats.status()); 
        }

   }else{
        //Create new Team and then add team to the Tenant repo
       
        let post_req = github_client.request(Method::POST, github_api)
            .bearer_auth(github_auth.clone())
            .header("User-Agent", "tenant-onbaording")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version" , "2022-11-28")
            .timeout(Duration::from_secs(5))
            .json(&teams_data);    
            
        let post_resp_data = post_req.send().await?; 
            println!("Team Creation Request: {}", post_resp_data.status());
 
            if (post_resp_data.status() == reqwest::StatusCode::CREATED) 
            {
                let post_res_body = post_resp_data.bytes().await?; 
                let str_post_body = post_res_body.to_vec();
                let str_response = String::from_utf8_lossy(&str_post_body);
                println!("Teams Response: {} ", str_response);
                team_added = true;
            }else if (post_resp_data.status() == reqwest::StatusCode::UNPROCESSABLE_ENTITY){
                
           /* Creating and adding team without team member cause of validation failure */
                let teams_dataw = TeamInfoWm { 
                    name: tenant_team.to_string(),
                    description: "A new team group generated by DevStudio team".to_string() ,
                    permission:"push".to_string(),  
                    privacy:"closed".to_string(),
                    repo_names: [format!("{}/{}", github_owner, tenant_repo).to_string()] 
                    };
 
                    let post_req_w = github_client.request(Method::POST, github_api)
                    .bearer_auth(github_auth )
                    .header("User-Agent", "tenant-onbaording")
                    .header("Accept", "application/vnd.github+json")
                    .header("X-GitHub-Api-Version" , "2022-11-28")
                    .timeout(Duration::from_secs(5))
                    .json(&teams_dataw); 

                    let post_resp_dataw = post_req_w.send().await?; 
                    println!("Team Creation Request: {}", post_resp_dataw.status());
                    if (post_resp_dataw.status() == reqwest::StatusCode::CREATED) 
                    {
                        team_added = true;
                    } 
            }else{
                team_added = false;
            }
    }
    Ok((team_added))
}