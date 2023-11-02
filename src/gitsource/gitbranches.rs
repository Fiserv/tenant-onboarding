use serde_derive::{Deserialize, Serialize};
use reqwest;
use std::error::Error;
use std::time::Duration;
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use serde_json;
use crate::gitsource;
use crate::gitsource::gitbranches::BranchEnum::{DEVELOP, QA, STAGE, MAIN};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Restrictions {
    users: [String ;1],
    teams: [String ;1],
    apps: [String ;1]
}

#[derive(Serialize, Deserialize, Debug)]
struct Checks {
    context: String,
    app_id: i16
}
#[derive(Serialize, Deserialize, Debug)]
struct RequiredPullRequestReviews {
    dismissal_restrictions: Restrictions,
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
    QA,
    STAGE,
    MAIN
}

impl BranchEnum {
    fn as_str(&self) -> &'static str {
        match self {
            DEVELOP => "develop",
            QA => "qa",
            STAGE => "stage",
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
    let github_branch_protection_api_template = setting["github"]["gitHubBranchProtectionAPITemplate"].as_str().unwrap();

    let github_auth_token_result = gitsource::authtoken::get_auth_token(setting);
    if !github_auth_token_result.is_ok() {
        return Result::Err(github_auth_token_result.err().unwrap());
    }
    let github_auth_token = github_auth_token_result.unwrap();

    // TODO Currently, the template repo (Test-repo) doesn't have a qa branch and therefore
    //      the tenant repo also will not have a qa branch
    for branch in [DEVELOP, STAGE, MAIN] {
        println!("Adding Branch Protection for {} branch", branch.as_str());
        let github_branch_protection_api = format!("https://api.github.com/repos/Fiserv/{}/branches/{}/protection", tenant_repo, branch.as_str());

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

        // TODO These properties are not used when setting the branch protections
        //      but the API returns 422 when they are not included.
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
            dismissal_restrictions: restrictions.clone(),
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
        let put_req = github_client.request(reqwest::Method::PUT, github_branch_protection_api.clone())
            .bearer_auth(github_auth_token.clone())
            .header("User-Agent", "branch protection")
            .header("Accept", "application/vnd.github+json")
            .timeout(Duration::from_secs(5))
            .json(&branch_protection_data);
        let resp_data = put_req.send().await?;
        println!("Branch Protection Status {}", resp_data.status());

        if resp_data.status() == reqwest::StatusCode::OK {
            let res_body = resp_data.bytes().await?;
            let str_body = res_body.to_vec();
            let str_response = String::from_utf8_lossy(&str_body);
            println!("Adding Branch Protection Response: {} ", str_response);
            created = true;
        } else {
            return Err(Box::try_from(resp_data.status().as_str()).unwrap());
        }
    }

    Ok(created)
}
