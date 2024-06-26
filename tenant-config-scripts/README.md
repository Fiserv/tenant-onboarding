# Overview

This is a directory for Github scripts to create or modify tenant configs.

## branchLock.py

Script to parse through an organization's repositories and mass lock branches (set to read-only).

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify configs

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `org_name`: Name for Github organization/project owner you'd like to make changes for
- `branches`: List of branch names to apply lock to
   - For curl you must manually input one branch per API call
- `branch_lock`: Configs for the branch protection being applied, check [Branch Protection Reference](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28#update-branch-protection) for more details.
   - The current script is configured to only set `read_only` and `enforce_admin` since other protections are already being covered under the RuleSets related script.
- `headers`: Default headers as provided by Github API

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/branchLock.py
```

A commented out curl command is also placed at the bottom of the script for reference.

## branchUnlock.py

Script to parse through an organization's repositories and mass unlock branches (remove all branch protection).

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify configs

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `org_name`: Name for Github organization/project owner you'd like to make changes for
- `branches`: List of branch names to remove protection from
   - For curl you must manually input one branch per API call
- `headers`: Default headers as provided by Github API

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/branchUnlock.py
```

A commented out curl command is also placed at the bottom of the script for reference.

## directFixTenantFiles.py

Script to parse through an organization's repositories and directly create/delete various files for certain branches (after removing their rulesets/protections). Primarily used to update a repo to use new workflow files.

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify repo (and workflow if changing those yaml files)

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `organization`: Name for Github organization/project owner you'd like to make changes for
- `repo_exceptions`: Repositories to skip during processing
- `branches`: List of branch names you'd like to change.
- `create_file_path`/`update_file_path`: File path for a file being added/edited.
- `create_file_content`/`update_file_content`: File content to create/edit at the `file_path`. Does not append, simply overwrite.

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/directFixTenantFiles.py
```
## fixTenantFiles.py

Script to parse through an organization's repositories and create/delete various files for a certain branch. Primarily used to update a certain branch (such as `preview`) to use new workflow files.

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify repo (and workflow if changing those yaml files)

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `organization`: Name for Github organization/project owner you'd like to make changes for
- `base_branch`: Branch name you'd like to change. Could be change to a list as needed and add a for-loop.
- `feature_branch`: Name of your feature branch.
   - If multiple branches are being changed, similarly make a list or generate this variable in each for-loop iteration of the `base_branches` list.
   - Make sure that it's not a common name that the tenant might already use (such as `preview-validator-fix`)
- `file_path`: File path for a file being added/edited.
- `file_content`: File content to overwrite at the `file_path`. Does not append, simply overwrite.
- `commit_message`: Commit message for creating/editing file.
- `delete_files`: List of files to be deleted (if they exist)
- `pull_request_title`
- `pull_request_body`: Description

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/fixTenantFiles.py
```

## githubBranches.py

Script to parse through an organization's repositories and mass configure branch protection Ruleset to enforce Pull Request and Status Checks across a list of predetermined branches (by name/regex).

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify configs

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `org_name`: Name for Github organization/project owner you'd like to make changes for
- `branches`: List of branch names to apply rules to; Starts with "refs/heads/" and can utilize regex
   - For curl you must copy paste this entire list into the command itself, or save as variable and use string format
- `ruleset`: Configs for the Ruleset being applied, check [Ruleset Reference](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#create-a-repository-ruleset) for more details
- `headers`: Default headers as provided by Github API

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/githubBranches.py
```

A commented out curl command is also placed at the bottom of the script for reference.

## privatizeRepos.py

Script to parse through an organization's repositories and change their visibility status.

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify configs

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `headers`: Default headers as provided by Github API
- `org_name`: Name for Github organization/project owner you'd like to make changes for
- `prod_tenants`: List of current prod tenants (to be used for excluding from the script if not intended to roll out across all tenants)
- `devstudio_backend_repos`: List of backend tenants (to be used for excluding from the script as they should never be privatized)
- `data.private`: Sets the `private` visibility proper to either `True` or `False`

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/githubBranches.py
```

A commented out curl command is also placed at the bottom of the script for reference.

## rulesetUpdate.py

Script to parse through an organization's repositories and mass update branch protection Ruleset to change various rules as needed. This script updates the `status_checks` rules.

- Utilizes `python3` so make sure you have it installed
- Will require a personal access token with proper organization/project read-write permissions to modify configs

### Variables

- `token`: Github access token - variable `<Github_Access_Token>` will need to be replaced with proper data
   - Should be saved in Github secret if we want to utilize pipeline
   - If running locally, you will need to generate your own access token with proper permission
- `org_name`: Name for Github organization/project owner you'd like to make changes for
- `ruleset`: Configs for the Ruleset being updated, check [Ruleset Reference](https://docs.github.com/en/rest/repos/rules?apiVersion=2022-11-28#create-a-repository-ruleset) for more details
- `headers`: Default headers as provided by Github API

For testing, replace `repositories` variable with a list containing a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     for repo in repositories:
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/rulesetUpdate.py
```