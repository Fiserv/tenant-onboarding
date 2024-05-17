import requests, base64

token = "<Github_Access_Token>"

headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github.v3+json"
}

repo_exceptions = ["tenants-data", "remote-actions", "tenant-onboarding", "tenants-doc", "TTPPackage", "TTPSampleApp", "developer-studio-support"]

def getRulesetID(org_name:str, repo:str) -> int:   
    # Fetch rulesets from the repo
    url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets"
    response = requests.get(url, headers=headers)

    # Process the response
    if response.status_code == 200:
        if len(response.json()) > 0:
            print(f"Updating {response.json()[0]['name']}")
            return response.json()[0]['id']
        print("No ruleset defined. Skipping removal.")
        return -1
    
    print(f"Failed to fetch {repo}'s ruleset.")
    print(response.json())
    return -1

def removeRuleset(org_name:str, repo:str, ruleset_id:int):   
    # Fetch repositories in the project
    url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets/{ruleset_id}"
    response = requests.delete(url, headers=headers)

    # Process the response
    if response.status_code == 204:
        print("Ruleset removed")
    else:
        print(f"Failed to remove {repo}'s ruleset.")
        print(response.json())

def createRuleset(org_name:str, repo:str):
    # Branch protection settings
    ruleset = {
        "name":"DevStudio Rules",
        "target": "branch",
        "enforcement": "active",
        "conditions": {
            "ref_name": {"include": ["refs/heads/main", "refs/heads/develop", "refs/heads/stage", "refs/heads/preview", "refs/heads/previous"], "exclude": []}
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

    url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets"
    response = requests.post(url, json=ruleset, headers=headers)

    if response.status_code == 200 or response.status_code == 201:
        print(f"GitHub ruleset created for {repo}.")
    else:
        print(f"Failed to create GitHub ruleset for {repo}.")
    print(response.json())

def commit_and_push_file(organization:str, repository:str, branch:str, file_content:str, file_path:str, commit_message:str):
    base_url = f"https://api.github.com/repos/{organization}/{repository}/contents/{file_path}?ref={branch}"

    # Check if validator file already exists
    response = requests.get(base_url, headers=headers)
    if response.status_code == 200:
        # File exists, get the current content
        response_json = response.json()
        current_sha = response_json['sha']
    elif response.status_code == 404:
        # File doesn't exist, set the current SHA to None
        current_sha = None
    else:
        # Handle other status codes as needed
        print(f"Error: {response.status_code}")
        return
    
    file_content_base64 = base64.b64encode(file_content.encode('utf-8')).decode('ascii')

    # Commit the changes
    commit_data = {
        "message": commit_message,
        "content": file_content_base64,
        "sha": current_sha,
        "branch": branch
    }
    response = requests.put(base_url, headers=headers, json=commit_data)

    # Process the response
    if response.status_code == 200:
        print(f"File {file_path} created/edited")
    else:
        print("File failed to create/update.", response.json())

def delete_file(organization:str, repository:str, branch:str, file_path:str):
    file_url = f"https://api.github.com/repos/{organization}/{repository}/contents/{file_path}?ref={branch}"
    get_file_response = requests.get(file_url, headers=headers)
    
    if get_file_response.status_code == 200:
        # File to be deleted exists, get its current content
        file_json = get_file_response.json()
        delete_sha = file_json['sha']
        
        # Delete the file
        delete_data = {
            "message": f"Delete file: {file_path}",
            "sha": delete_sha,
            "branch": branch
        }
        delete_response = requests.delete(file_url, headers=headers, json=delete_data)

        # Process the response
        if delete_response.status_code == 200:
            print(f"File {file_path} deleted")
        else:
            print("File failed to delete.", delete_response.json())
    else:
        print(f"File {file_path} not found")

def get_organization_repositories(organization):
    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github.v3+json"
    }

    url = f"https://api.github.com/orgs/{organization}/repos"
    response = requests.get(url, params={"per_page": 100}, headers=headers)

    # Process the response
    if response.status_code == 200:
        repositories = [repo['name'] for repo in response.json()]
        print(repositories)
        return repositories
    else:
        print("Failed to fetch repositories from the project.", response.json())
        exit()

if __name__ == "__main__":
    organization = "Fiserv"
    branches = [ "develop", "stage", "main", "preview" ]
    create_file_path = ".github/workflows/zip-generator.yaml"
    create_file_content = '''name: Studio Zip Generator
on:
  push:
    branches: [ develop,stage,main,preview,previous ]
    paths:
      - 'reference/**'

jobs:
  push_actions:
    uses: Fiserv/remote-actions/.github/workflows/file-update-service.yaml@main
    secrets: inherit
'''
    validator_file_path = ".github/workflows/validator.yaml"
    validator_file_content = '''name: Studio Validator
on:
  pull_request:
    branches: [ develop,stage,main,preview,previous ]

jobs:
  validator:
    uses: Fiserv/remote-actions/.github/workflows/validator-service.yaml@main
    secrets: inherit
'''

    # Get the repositories in the organization
    repositories = get_organization_repositories(organization)

    for repo in filter(lambda r: r not in repo_exceptions, repositories):
        print(f"Processing repository: {repo}")

        # Remove rulesets
        ruleID = getRulesetID(organization, repo)
        if ruleID != -1:
            removeRuleset(organization, repo, ruleID)

        for base_branch in branches:
            branch_check_url = f"https://api.github.com/repos/{organization}/{repo}/branches/{base_branch}"
            branch_check_response = requests.get(branch_check_url, headers=headers)

            if branch_check_response.status_code == 200:
                print(f"Branch '{base_branch}' exists.")
            elif branch_check_response.status_code == 404:
                print(f"Branch '{base_branch}' does not exist.\n")
                continue
            else:
                print(f"Error: {branch_check_response.status_code}")
                print(branch_check_response.text)
                continue

            # Commit and push changes to the feature branch
            commit_and_push_file(organization, repo, base_branch, create_file_content, create_file_path, "Add zip generator workflow")
            commit_and_push_file(organization, repo, base_branch, validator_file_content, validator_file_path, "Update main validator workflow")
            print()
        
        if ruleID != -1:
            # Reapply ruleset
            createRuleset(organization, repo)

        # Repo done
        print()
