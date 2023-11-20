use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json;
use crate::gitsource;

// The 'restrictions' object in the 'required_pull_request_reviews' property needs to be
// empty in the API payload of the PUT request that creates the initial branch protections.
// Otherwise, the 'Restrict who can dismiss pull request review' property will be enabled.
// If the 'restrictions' structure contains any fields which are set to empty strings in the
// payload, the above property will be enabled. Only if the 'restrictions' object is empty
// will the property be disabled.

const _MAX_ITERATIONS: i8 = 4;
const _MAX_RETRIES: i8 = 3;
const _INITIAL_RETRY_MS: u64 = 200;

#[tokio::main]
pub async fn process_github_branches(config_yaml: &Vec<Yaml> , settings_yaml: &Vec<Yaml>, execute: bool) -> Result<(bool), Box<dyn Error>> {
    let config = &config_yaml[0];
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();

    let setting = &settings_yaml[0];
    let github_auth_token_result = gitsource::authtoken::get_auth_token(setting);
    if !github_auth_token_result.is_ok() {
        return Result::Err(github_auth_token_result.err().unwrap());
    }
    let github_auth_token = github_auth_token_result.unwrap();
    let github_rulesets_api = format!("https://api.github.com/repos/Fiserv/{}/rulesets", tenant_repo);

    println!("Adding Branch Protection for {}", tenant_repo);

    let branch_protection_data = r#"{
        "name": "DevStudio Rules",
        "target": "branch",
        "enforcement": "active",
        "conditions": {
            "ref_name": {
                "include": [
                    "refs/heads/main",
                    "refs/heads/develop",
                    "refs/heads/stage",
                    "refs/heads/preview",
                    "refs/heads/previous"
                ],
                "exclude": []
            }
        },
        "rules": [
            {"type": "deletion"},
            {"type": "non_fast_forward"},
            {
                "type": "pull_request",
                "parameters": {
                    "dismiss_stale_reviews_on_push": false,
                    "require_code_owner_review": false,
                    "require_last_push_approval": false,
                    "required_approving_review_count": 0,
                    "required_review_thread_resolution": true
                }
            },
            {
                "type": "required_status_checks",
                "parameters": {
                    "strict_required_status_checks_policy": true,
                    "required_status_checks": [
                        {"context": "validator / api_validator / api_validator_actions"},
                        {"context": "validator / tenant-config-validator / Tenant-Config-Action"}
                    ]
                }
            }
        ]
    }"#;
    let branch_protection_data_json: serde_json::Value = serde_json::from_str(&branch_protection_data).unwrap();

    let github_client = reqwest::Client::new();
    let mut iterations = 1;
    let mut delay_ms = _INITIAL_RETRY_MS;
    let mut rulesets_created = false;
    if (execute) {
        while iterations <= _MAX_ITERATIONS && !rulesets_created {
            let create_rulesets_request =
                create_request(reqwest::Method::POST, github_rulesets_api.clone(), github_auth_token.clone())  
                    .json(&branch_protection_data_json);
            let create_rulesets_response = create_rulesets_request.send().await?;

            let status = create_rulesets_response.status();
            println!("Rulesets creation status: {}", status);
            if status != StatusCode::OK && status != StatusCode::CREATED {
                if (status == StatusCode::UNPROCESSABLE_ENTITY) {
                    println!("Ruleset with same name may already exist. Please check repository.");
                    break;
                }
                if status != StatusCode::NOT_FOUND {
                    return Err(Box::try_from(create_rulesets_response.status().as_str()).unwrap());
                }

                if iterations > _MAX_RETRIES {
                    println!("aborting");
                    break;
                }

                println!("Retrying with {}ms delay", delay_ms);
                sleep(Duration::from_millis(delay_ms));
                iterations += 1;
                delay_ms = delay_ms * 2;
                continue;
            }

            let res_body = create_rulesets_response.bytes().await?;
            let str_body = res_body.to_vec();
            let str_response = String::from_utf8_lossy(&str_body);
            println!("Response: {} ", str_response);
            rulesets_created = true;
        }
    } else {
        println!("JSON data to be sent to {}:\n{:#?}", github_rulesets_api, branch_protection_data_json);
    }

    Ok(rulesets_created)
}

fn create_request(method: Method, url: String, github_auth_token: String) -> RequestBuilder {
    let github_client = reqwest::Client::new();
    let req = github_client.request(method, url)
        .bearer_auth(github_auth_token.clone())
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "tenant-onboarding")
        .timeout(Duration::from_secs(5));

    req
}
