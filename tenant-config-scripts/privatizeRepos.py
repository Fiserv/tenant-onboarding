import requests

''' --- Variables --- '''

# Github Access Token
token = "<Github_Access_Token>"

# Organization Name
org_name = "Fiserv"

prod_tenants = ["AccessOne", "BackOffice", "Commerce-Hub", "SnapPay", "Support", "acceptance-solutions-apac", "ai-center", "alldata", "banking-hub", "cloud-acceleration-center","design-center", "digital-disbursements", 
                "exchange", "firstvision-apac", "firstvision-emea", "firstvision-latam", "ipg-na", "issuer-solutions", "merchant-acquiring-latam", "reporting", "signature-international", "silvercore"]

devstudio_backend_repos = ["tenants-data", "remote-actions", "sample-tenant-repo", "TTPPackage", "TTPSampleApp", "developer-studio-support", "tenants-doc"]

headers = {
  "Authorization": f"token {token}",
  "Accept": "application/vnd.github+json",
  "X-GitHub-Api-Version": "2022-11-28"
}

# Fetch repositories in the project
def get_repos() -> list:  
  url = f"https://api.github.com/orgs/{org_name}/repos"
  response = requests.get(url, params={"per_page": 200}, headers=headers)

  # Process the response
  if response.status_code == 200 and len(response.json()) > 0:
    repositories = [repo['name'] for repo in response.json()]
    # print(repositories)
    return repositories
  else:
    print(f"Failed to fetch repositories from the project. Error: {response.status_code}\n{response.json()}")
    exit()

def set_repo_to_private(repo):
  url = f"https://api.github.com/repos/{org_name}/{repo}"
  data = {
    "private": True
  }
  response = requests.patch(url, headers=headers, json=data)
  if response.status_code == 200:
    print(f"Repository '{repo}' set to private successfully.")
  else:
    print(f"Failed to privatize repo '{repo}'. Error: {response.status_code}\n{response.json()}")

''' --- Script Execution --- '''

if __name__ == "__main__":
  # Get all repositories under the organization
  repos = get_repos()
  # repos.sort()
  for repo in repos:
    # if repo in prod_tenants or repo in devstudio_backend_repos:
    if repo in devstudio_backend_repos:
      continue
    set_repo_to_private(repo)
