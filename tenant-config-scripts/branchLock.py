import requests

# Personal Access Token
token = "<Github_Access_Token>"

headers = {
    "Authorization": f"token {token}",
    "Accept": "application/vnd.github+json",
    "X-GitHub-Api-Version": "2022-11-28"
}

# Organization Name
org_name = "Fiserv"

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
print()

# Branches to lock
branches = ["main"]

# Branch lock settings
branch_lock = {
    "lock_branch": True,
    "enforce_admins": True,
    "required_pull_request_reviews": None,
    "required_status_checks": None, 
    "restrictions": None
}

# for repo in repositories:
for repo in repositories:
    for branch in branches:
        url = f"https://api.github.com/repos/{org_name}/{repo}/branches/{branch}/protection"
        response = requests.put(url, json=branch_lock, headers=headers)

        if response.status_code == 200 or response.status_code == 201:
            print(f"'{branch}' branch locked for {repo}.")
        else:
            print(f"Failed lock '{branch}' branch for {repo}.")
            print(response.json())

'''
curl -L \
  -X PUT \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <Github_Access_Token>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/Fiserv/<Repo_Name>/branches/<branch_name>/protection \
  -d '{"lock_branch": true, "enforce_admins": true, "required_pull_request_reviews": null, "required_status_checks": null, "restrictions": null}'
'''
