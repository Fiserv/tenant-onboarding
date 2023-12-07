import requests, base64

token = "<Github_Access_Token>"

headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github.v3+json"
}

def create_branch(organization:str, repository:str, base_branch:str, new_branch:str):
    base_url = f"https://api.github.com/repos/{organization}/{repository}/git/refs/heads/{base_branch}"
    
    # Get the latest commit SHA of the base branch
    response = requests.get(base_url, headers=headers)
    response_json = response.json()
    latest_sha = response_json['object']['sha']

    # Create a new branch pointing to the latest commit of the base branch
    new_branch_url = f"https://api.github.com/repos/{organization}/{repository}/git/refs"
    new_branch_data = {
        "ref": f"refs/heads/{new_branch}",
        "sha": latest_sha
    }
    response = requests.post(new_branch_url, headers=headers, json=new_branch_data)

    # Process the response
    if response.status_code == 200:
        print(f"Branch {new_branch} created from {base_branch}")
    else:
        print("Branch not created.", response.json())

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

    response = requests.put(commit_url, headers=headers, json=commit_data)

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

def create_pull_request(username, repository, base_branch, head_branch, title, body):
    base_url = f"https://api.github.com/repos/{username}/{repository}/pulls"

    # Create a pull request
    pull_request_data = {
        "title": title,
        "body": body,
        "head": head_branch,
        "base": base_branch
    }

    response = requests.post(base_url, headers=headers, json=pull_request_data)
    
    # Process the response
    if response.status_code == 200:
        print(f"PR Created at {response.json()['url']}")
    else:
        print("PR failed to be created.", response.json())

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
        print("Failed to fetch repositories from the project.")
        exit()

if __name__ == "__main__":
    organization = "Fiserv"
    base_branch = "preview"
    feature_branch = "devstudio-preview-validator-workflow-update"
    create_file_path = ".github/workflows/validator.yaml"
    create_file_content = '''name: Studio Validator
on:
  # Triggers the workflow on push or pull request events for the dev, stage, main, preview, and previous branch.
  push:
    branches: [ develop,stage,main,preview,previous ]
  pull_request:
    branches: [ develop,stage,main,preview,previous ]

jobs:
  validator:
    uses: Fiserv/remote-actions/.github/workflows/validator-service.yaml@main
    secrets: inherit
'''
    commit_message = f"Fix validator workflow for {base_branch} branch"
    delete_files = [".github/workflows/api-validator.yaml", ".github/workflows/zip-generator.yaml"]
    pull_request_title = f"{base_branch.capitalize()} Branch Validator Update"
    pull_request_body = f"Updating the workflow logic of the {base_branch} branch. Please make sure to include same settings in future if pushing from other branches."

    # Get the repositories in the organization
    repositories = get_organization_repositories(organization)

    for repo in repositories:
        print(f"Processing repository: {repo}")

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

        # Create a feature branch
        create_branch(organization, repo, base_branch, feature_branch)

        # Commit and push changes to the feature branch
        commit_and_push_file(organization, repo, feature_branch, create_file_path, create_file_path, commit_message)

        # Delete old validator files
        for old_file in delete_files:
            delete_file(organization, repo, feature_branch, old_file)

        # Create a pull request
        create_pull_request(organization, repo, base_branch, feature_branch, pull_request_title, pull_request_body)
        print()
