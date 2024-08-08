import re, requests, base64

token = "<Github_Access_Token>"

headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github.v3+json"
}

github_direct_link_regex = r'(http[s]?://?raw.githubusercontent.*)/assets/'
devstudio_backend_repos = ["tenants-data", "remote-actions", "tenant-onboarding", "sample-tenant-repo", "developer-studio-support", "TTPPackage", "TTPSampleApp", "tenants-doc", "Support", "mdncontent"]

def get_organization_repositories(organization):
    url = f"https://api.github.com/orgs/{organization}/repos"
    response = requests.get(url, params={"per_page": 100}, headers=headers)

    if response.status_code == 200:
        repositories = [repo['name'] for repo in response.json()]
        print(repositories)
        return repositories
    else:
        print("Failed to fetch repositories from the project.", response.json())
        exit()

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
    if response.status_code == 200 or response.status_code == 201:
        print(f"Branch {new_branch} created from {base_branch}")
    elif response.status_code == 422:
        print(f"Branch '{new_branch}' already exists.")
    else:
        print(response.status_code, "Branch not created.", response.json())

def check_file_content(contents) -> tuple:
    has_docs, changed_files = False, False
    for content_file in contents:
        if type(content_file) is not dict:
            continue
        
        download_url = content_file['url']
        response = requests.get(download_url, headers=headers)
        file_details = response.json()
        
        if content_file['type'] == 'file':
            has_docs = True
            if response.status_code == 200:
                file_content = base64.b64decode(file_details['content']).decode('utf-8')
            else:
                continue

            link = re.search('raw.githubusercontent', file_content)
            if link:
                replace_links(content_file['path'], file_content)
                changed_files = True
        elif content_file['type'] == 'dir':
            result = check_file_content(file_details)
            has_docs = result[0] or has_docs
            changed_files = result[1] or changed_files
    return (has_docs, changed_files)

def replace_links(file_path:str, file_content:str):
    updated_content = re.sub(github_direct_link_regex, '/assets/', file_content)
    commit_and_push_file(organization, repo, feature_branch, updated_content, file_path, "Fix githubusercontent links")

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
        "message": f"Updating {file_path}'s github links",
        "content": file_content_base64,
        "sha": current_sha,
        "branch": branch
    }
    response = requests.put(base_url, headers=headers, json=commit_data)

    # Process the response
    if response.status_code == 200:
        print(f"File {file_path} edited")
    else:
        print("File failed to create/update.", response.json())

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
    if response.status_code == 200 or response.status_code == 201:
        print(f"PR Created at {response.json()['url']}")
    elif response.status_code == 422:
        print(f"A pull request already exists for {head_branch}")
    else:
        print(response.status_code, "PR failed to be created.", response.json())

if __name__ == "__main__":
    organization = "Fiserv"

    for repo in get_organization_repositories(organization):
        if repo in devstudio_backend_repos:
            continue
        print(f"Repo: {repo}")
        for branch in ["develop"]:
            feature_branch = f"{branch}-link-update"
            pull_request_title = f"Github Direct Link Update - {branch}"
            pull_request_body = f"Replaced all githubusercontent links in documents for {branch} branch. Please make sure to update other branches and similar links as necessary."
            changed = False

            create_branch(organization, repo, branch, feature_branch)

            for folder in ["config", "docs", "reference"]:
                contents_url = f'https://api.github.com/repos/{organization}/{repo}/contents/{folder}?ref={feature_branch}'
                contents_response = requests.get(contents_url, headers=headers).json()

                result = check_file_content(contents_response)
                if not result[0]:
                    print(f"Error: {repo} is not a normal product")
                    break
                if not result[1]:
                    print(f"No changes made for {repo} - {folder}")
                    continue
                changed = True
            if changed:
                create_pull_request(organization, repo, branch, feature_branch, pull_request_title, pull_request_body)
