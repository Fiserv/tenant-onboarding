import requests

''' --- Variables --- '''

# Github Access Token
token = "<API_TOKEN_GITHUB>"

# Organization Name
org_name = "Fiserv"

# List of branches to apply protection settings to
branches = ["refs/heads/main", "refs/heads/develop", "refs/heads/stage", "refs/heads/preview", "refs/heads/previous"]

# Branch protection settings
ruleset = {
    "name":"DevStudio Rules",
    "target": "branch",
    "enforcement": "active",
    "conditions": {
        "ref_name": {"include": branches, "exclude": []}
    },
    "rules": [
        {"type": "deletion"},
        {"type": 
            "pull_request", 
            "parameters": {
                "dismiss_stale_reviews_on_push": False,
                "require_code_owner_review": False,
                "require_last_push_approval": False,
                "required_approving_review_count": 0, 
                "required_review_thread_resolution": True
            }
        },
        {"type": "required_status_checks", "parameters": {
            "strict_required_status_checks_policy": False, 
            "required_status_checks": [
                {"context": "validator / api_validator / api_validator_actions"},
                {"context": "validator / tenant-config-validator / Tenant-Config-Action"}
        ]}},
        {"type": "non_fast_forward"}
    ]
}

headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28"
}

''' --- Script Execution --- '''

# Fetch repositories in the project
url = f"https://api.github.com/orgs/{org_name}/repos"
response = requests.get(url, params={"per_page": 100}, headers=headers)

# Process the response
if response.status_code == 200:
    repositories = [repo['name'] for repo in response.json()]
    print(repositories)
else:
    print("Failed to fetch repositories from the project.")
    exit()

# Apply Ruleset to each repository in the list
for repo in repositories:
    url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets"
    response = requests.post(url, json=ruleset, headers=headers)

    if response.status_code == 200 or response.status_code == 201:
        print(f"GitHub ruleset created for {repo}.")
    else:
        print(f"Failed to create GitHub ruleset for {repo}.")
        print(response.json())

# curl command to apply ruleset to a single repository; for testing
'''
curl -L \
  -X POST \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <Github_Access_Token>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/Fiserv/<Repo_Name>/rulesets \
  -d '{"name":"DevStudio Rules", "target": "branch", "enforcement": "active", "conditions": {"ref_name": {"include": ["refs/heads/main", "refs/heads/develop", "refs/heads/stage", "refs/heads/preview", "refs/heads/previous"], "exclude": []}}, "rules": [{"type": "deletion"}, {"type": "pull_request", "parameters": {"dismiss_stale_reviews_on_push": false, "require_code_owner_review": false, "require_last_push_approval": false, "required_approving_review_count": 0, "required_review_thread_resolution": true}}, {"type": "required_status_checks", "parameters": {"strict_required_status_checks_policy": true, "required_status_checks": [{"context": "validator / api_validator / api_validator_actions"}, {"context": "validator / tenant-config-validator / Tenant-Config-Action"}]}}, {"type": "non_fast_forward"}]}'
'''
