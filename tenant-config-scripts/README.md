# Overview

This is a directory for Github scripts to create or modify tenant configs.

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

- For testing, replace `repositories` variable with a list with a single tenant before the for-loop or modify the for-loop itself:
   - ```Python 
     repositories = ["Test-repo"]
     ```
   - ```Python 
     for repo in ["Test-repo"]:
     ```

### Usage

``` Bash
python3 tenant-config-scripts/githubBranches.py
```

A commented out curl command is also placed at the bottom of the script for reference.