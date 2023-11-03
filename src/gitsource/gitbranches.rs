use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use reqwest::{Method, RequestBuilder, Response};
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json;
use crate::gitsource;
use crate::gitsource::gitbranches::BranchEnum::{DEVELOP, STAGE, PREVIEW, MAIN};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Restrictions {
    users: [String ;1],
    teams: [String ;1],
    apps: [String ;1]
}

// The 'restrictions' object in the 'required_pull_request_reviews' property needs to be
// empty in the API payload of the PUT request that creates the initial branch protections.
// Otherwise, the 'Restrict who can dismiss pull request review' property will be enabled.
// If the 'restrictions' structure contains any fields which are set to empty strings in the
// payload, the above property will be enabled. Only if the 'restrictions' object is empty
// will the property be disabled.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct DismissalRestrictions {}

#[derive(Serialize, Deserialize, Debug)]
struct Checks {
    context: String,
    app_id: i16
}
#[derive(Serialize, Deserialize, Debug)]
struct RequiredPullRequestReviews {
    dismissal_restrictions: DismissalRestrictions,
    dismiss_stale_reviews: bool,
    require_code_owner_reviews: bool,
    required_approving_review_count: i8,
    require_last_push_approval: bool,
    bypass_pull_request_allowances: Restrictions
}
#[derive(Serialize, Deserialize, Debug)]
struct RequiredStatusChecks {
    strict: bool,
    checks: [Checks; 1]
}

#[derive(Serialize, Deserialize, Debug)]
struct BranchProtection {
    required_status_checks: RequiredStatusChecks,
    enforce_admins: bool,
    required_pull_request_reviews: RequiredPullRequestReviews,
    restrictions: Restrictions,
    required_linear_history: bool,
    allow_force_pushes: bool,
    allow_deletions: bool,
    block_creations: bool,
    required_conversation_resolution: bool,
    lock_branch: bool,
    allow_fork_syncing: bool
}

#[derive(Debug, Clone)]
struct BranchSpecificProtections {
    dismiss_stale_reviews: bool,
    required_approving_review_count: i8,
}

#[derive(PartialEq)]
enum BranchEnum {
    DEVELOP,
    STAGE,
    PREVIEW,
    MAIN
}

impl BranchEnum {
    fn as_str(&self) -> &'static str {
        match self {
            DEVELOP => "develop",
            STAGE => "stage",
            PREVIEW => "preview",
            MAIN => "main",
        }
    }
}

#[tokio::main]
pub async fn process_github_branches(config_yaml: &Vec<Yaml> , settings_yaml: &Vec<Yaml>) -> Result<(bool), Box<dyn Error>> {

    let mut created = false;

    let config = &config_yaml[0];
    let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();

    let setting = &settings_yaml[0];
    let github_auth_token_result = gitsource::authtoken::get_auth_token(setting);
    if !github_auth_token_result.is_ok() {
        return Result::Err(github_auth_token_result.err().unwrap());
    }
    let github_auth_token = github_auth_token_result.unwrap();

    // TODO Currently, the template repo (Test-repo) doesn't have a qa branch and therefore
    //      the tenant repo also will not have a qa branch
    for branch in [DEVELOP, STAGE, PREVIEW, MAIN] {
        println!("Adding Branch Protection for {} branch", branch.as_str());
        let github_branch_protection_api = format!("https://api.github.com/repos/Fiserv/{}/branches/{}/protection", tenant_repo, branch.as_str());
        let github_branch_protection_restrictions_api = github_branch_protection_api.clone() + "/restrictions";

        let dismiss_stale_reviews: bool;
        let required_approving_review_count: i8;

        // TODO Should there be different protections for qa, stage, and main?
        //      The Test-repo has the same branch protections for all non-develop branches
        // TODO Test-repo only includes status checks on the develop branch. Do we
        //      want status checks on all non-develop branches? (this code is adding
        //      status checks on all branches)
        if branch == DEVELOP {
            dismiss_stale_reviews = false;
            required_approving_review_count = 0;
        } else {
            dismiss_stale_reviews = true;
            required_approving_review_count = 1;
        }
        let branch_specific_protections = BranchSpecificProtections {
            dismiss_stale_reviews,
            required_approving_review_count
        };

        // These properties are not used when applying the branch protections
        // but are required by the API.
        let restrictions = Restrictions {
            users: [format!("{}", "")],
            teams: [format!("{}", "")],
            apps: [format!("{}", "")]
        };

        let checks_data = Checks {
            context: format!("{}","validator / tenant-config-validator / Tenant-Config-Action"),
            app_id: 15368
        };

        let required_status_checks = RequiredStatusChecks {
            strict: true,
            checks: [checks_data]
        };

        let required_pull_request_reviews = RequiredPullRequestReviews {
            dismissal_restrictions: DismissalRestrictions{},
            dismiss_stale_reviews: branch_specific_protections.dismiss_stale_reviews,
            require_code_owner_reviews: false,
            required_approving_review_count: branch_specific_protections.required_approving_review_count,
            require_last_push_approval: false,
            bypass_pull_request_allowances: restrictions.clone()
        };

        let branch_protection_data = BranchProtection {
            required_status_checks,
            enforce_admins: false,
            required_pull_request_reviews,
            restrictions: restrictions.clone(),
            required_linear_history: false,
            allow_force_pushes: false,
            allow_deletions: false,
            block_creations: false,
            required_conversation_resolution: false,
            lock_branch: false,
            allow_fork_syncing: false
        };

        let github_client = reqwest::Client::new();
        let create_branch_protections_req =
            create_request(reqwest::Method::PUT, github_branch_protection_api.clone(), github_auth_token.clone())
                .json(&branch_protection_data);
        let create_branch_protections_response = create_branch_protections_req.send().await?;
        println!("Branch Protection Status: {}", create_branch_protections_response.status());

        if create_branch_protections_response.status() != reqwest::StatusCode::OK {
            return Err(Box::try_from(create_branch_protections_response.status().as_str()).unwrap());
        }

        println!("Disabling Overly Restrictive Restrictions for {} branch", branch.as_str());
        let delete_restrictions_req =
            create_request(reqwest::Method::DELETE, github_branch_protection_restrictions_api.clone(), github_auth_token.clone());
        let delete_restrictions_response = delete_restrictions_req.send().await?;
        println!("Disabling Overly Restrictive Restrictions Status: {}", delete_restrictions_response.status());

        if delete_restrictions_response.status() != reqwest::StatusCode::NO_CONTENT {
            return Err(Box::try_from(delete_restrictions_response.status().as_str()).unwrap());
        }

        let res_body = create_branch_protections_response.bytes().await?;
        let str_body = res_body.to_vec();
        let str_response = String::from_utf8_lossy(&str_body);
        println!("Adding Branch Protection Response: {} ", str_response);
        created = true;
    }

    Ok(created)
}

fn create_request(method: Method, url: String, github_auth_token: String) -> RequestBuilder {
    let github_client = reqwest::Client::new();
    let req = github_client.request(method, url)
        .bearer_auth(github_auth_token.clone())
        .header("User-Agent", "branch protection")
        .header("Accept", "application/vnd.github+json")
        .timeout(Duration::from_secs(5));

    req
}
