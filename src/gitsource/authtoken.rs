use std::env;
use std::error::Error;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

pub fn get_auth_token(settings_yaml: &Yaml) -> Result<String, Box<dyn Error>> {
    let github_auth_token_env_var = settings_yaml["github"]["gitHubAuthTokenEnvVar"].as_str().unwrap();
    let github_auth_token: String;
    let github_auth_token_env_var_contents = env::var(github_auth_token_env_var);
    if !github_auth_token_env_var_contents.is_ok() {
        let error_string = format!("{} environment variable not found", github_auth_token_env_var);
        return Err(Box::try_from(error_string).unwrap());
    }
    github_auth_token = github_auth_token_env_var_contents.unwrap();
    Ok(github_auth_token)
}

pub fn get_webhook_key(env: String, settings_yaml: &Yaml) -> Result<String, Box<dyn Error>> {
    let mut webhook_secret_env_var = "";
    match env.trim() { 
        "stage" => webhook_secret_env_var = settings_yaml["github"]["gitHubStageHookKey"].as_str().unwrap(), 
        "production" => webhook_secret_env_var = settings_yaml["github"]["gitHubProdHookKey"].as_str().unwrap(), 
        _ => return Ok(settings_yaml["github"]["gitHubDevHookKey"].as_str().unwrap()), 
    };

    let github_webhook_secret: String;
    let webhook_secret_env_var_contents = env::var(webhook_secret_env_var);
    if !webhook_secret_env_var_contents.is_ok() {
        let error_string = format!("environment variable '{}' not found", webhook_secret_env_var);
        return Err(Box::try_from(error_string).unwrap());
    }
    github_webhook_secret = webhook_secret_env_var_contents.unwrap();
    Ok(github_webhook_secret)
}