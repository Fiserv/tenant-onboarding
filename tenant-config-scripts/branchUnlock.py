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
    print(response.json())
    exit()
print()

branches = ["main", "preview"]

for repo in repositories:
    for branch in branches:
        url = f"https://api.github.com/repos/{org_name}/{repo}/branches/{branch}/protection"
        response = requests.delete(url, headers=headers)

        if response.status_code == 200 or response.status_code == 204:
            print(f"'{branch}' branch unlocked for {repo}.")
        else:
            print(f"Failed unlock '{branch}' branch for {repo}.")
            print(response.json())

'''
curl -L \
  -X DELETE \
  -H "Accept: application/vnd.github+json" \
  -H "Authorization: Bearer <Github_Access_Token>" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  https://api.github.com/repos/Fiserv/Test-repo/branches/main/protection \
'''