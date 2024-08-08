import requests, re, string

printable = set(string.printable)

token = "<Github_Access_Token>"

headers = {
    "Authorization": f"token {token}",
    "User-Agent": "Minh Pham",
    "Accept": "application/vnd.github.v3+json"
}

links_list = []
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

def validateFileContent(contents) -> bool:
    for content_file in contents:
        if type(content_file) is not dict:
            return False
        if content_file['type'] == 'file':
            download_url = content_file['download_url']
            file_content_response = requests.get(download_url, headers=headers)
            file_content = file_content_response.text

            links = re.findall(r'https?:\/\/[^\s]+|www\.[^\s]+|[^\s]+\.[a-z]{3}', file_content)
            links = [l if l in printable else ''.join(char if 32 <= ord(char) <= 127 else ' ' for char in l) for l in links]
            if len(links) > 0:
                links_list.append(f"\nLinks in {content_file['name']}: {links}")
            # print(f"Links in {content_file['name']}: {links}\n")
        elif content_file['type'] == 'dir':
            current_path = content_file['path']
            sub_contents_url = f'https://api.github.com/repos/{organization}/{repo}/contents/{current_path}'
            sub_contents_response = requests.get(sub_contents_url, headers=headers).json()
            if not validateFileContent(sub_contents_response):
                return False
    return True

if __name__ == "__main__":
    organization = "Fiserv"

    for repo in get_organization_repositories(organization):
        if repo in devstudio_backend_repos:
            continue
        links_list.append(repo + '\n')
        print(f"Repo: {repo}")
        for folders in ["docs", "reference"]:

            contents_url = f'https://api.github.com/repos/{organization}/{repo}/contents/{folders}'
            contents_response = requests.get(contents_url, headers=headers).json()

            if not validateFileContent(contents_response):
                print(f"Error: {repo} is not a normal product")
                break

        links_list.append('-----------------')
        # print('-----------------')

    with open('external-links.txt', 'w') as file:
        for link in links_list:
            print(link)
            file.write(link + '\n')
