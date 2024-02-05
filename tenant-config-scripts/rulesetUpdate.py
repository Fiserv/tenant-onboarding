import requests

''' --- Variables --- '''

# Github Access Token
token = "<Github_Access_Token>"

# Organization Name
org_name = "Fiserv"

# Branch protection settings
ruleset = {
  "rules": [
    {
      "type": "deletion",
      "parameters": None
    },
    {
      "type": "non_fast_forward",
      "parameters": None
    },
    {
      "type": "pull_request",
      "parameters": {
        "dismiss_stale_reviews_on_push": False,
        "require_code_owner_review": False,
        "require_last_push_approval": True,
        "required_approving_review_count": 0,
        "required_review_thread_resolution": True
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "strict_required_status_checks_policy": False,
        "required_status_checks": [
          {
            "context": "validator / api_validator / api_validator_actions"
          },
          {
            "context": "validator / tenant-config-validator / Tenant-Config-Action"
          }
        ]
      }
    }
  ]
}

headers = {
  "Authorization": f"token {token}",
  "Accept": "application/vnd.github+json",
  "X-GitHub-Api-Version": "2022-11-28"
}

def getRepos() -> list:  
# Fetch repositories in the project
  url = f"https://api.github.com/orgs/{org_name}/repos"
  response = requests.get(url, params={"per_page": 100}, headers=headers)

  # Process the response
  if response.status_code == 200 and len(response.json()) > 0:
    repositories = [repo['name'] for repo in response.json()]
    print(repositories)
    return repositories
  else:
    print("Failed to fetch repositories from the project.")
    exit()

def getRulesetID(repo:str) -> int:   
  # Fetch repositories in the project
  url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets"
  response = requests.get(url, headers=headers)

  # Process the response
  if response.status_code == 200:
    print(f"Updating {response.json()[0]['name']}")
    return response.json()[0]['id']
  else:
    print(f"Failed to fetch {repo}'s ruleset.")
    print(response.json())

''' --- Script Execution --- '''

repositories = getRepos()
for repo in repositories:
  ruleset_id = getRulesetID(repo)
  if ruleset_id is None:
    continue

  url = f"https://api.github.com/repos/{org_name}/{repo}/rulesets/{ruleset_id}"
  response = requests.put(url, json=ruleset, headers=headers)

  if response.status_code == 200 or response.status_code == 201:
    print(f"GitHub ruleset updated for {repo}.")
  else:
    print(f"Failed to update GitHub ruleset for {repo}.")
    print(response.json())