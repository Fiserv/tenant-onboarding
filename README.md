# Overview

This program is used to onboard new tenants. It automates the process of creating a new repo, adding webhooks, branch protection, etc. It is invoked by the Fiserv [Support workflow](https://github.com/Fiserv/Support/blob/main/.github/workflows/run-onboarding-service.yaml)

The program provides the following functionality:

1. Create a github repo (-r)
2. Add the devstudio and tenant github teams to the repo (-t)
3. Setup the github webooks (-h)
4. Create Ruleset/Branch protection (-b)
5. Generate the necessary DB snippets needed (-d)
6. Fill out tenant.json and product-layout.yaml file.
7. (delete coming later...)
8. (integrate with GitLab to commit DB changes coming later)

## How to build

1. Clone/checkout the repo
2. From the cloned directory run: `cargo build`

## How to use

### PREP
It makes use of MongoDB tools. mongodbimport.  You'll need to have this installed first.
If you're on mac then you can just homebrew install it.

brew tap mongodb/brew
brew install mongodb-database-tools

Either update a config file or pass in data through the CLI

Create an environment variable called GITHUB_AUTH_TOKEN whose value is a valid GitHub auth-token, for example, a Personal Access Token

## Dry run

Default mode is a dry run that will print out info about what happened to a log.  This way you can confirm you did the right thing before you really create a repo.

To actually execute for REAL, you'll need the extra flag.
Example :  ./startup.sh -f
to -r --execute

### Using Startup Script

From root directory run startup script

Example :  ./startup.sh -f '-te'

## Using the config file

Tenant-Onboarding-Form.yaml holds information that will be used when creating the repo, when updating tenant.json, and the db scripts.

### tenant.json 
The tenant.json file, that is updated to include values corresponding to the tenant being onboarded, is in the `tenant-onboarding` repo.
The updated tenant.json is pushed to the tenant's repo created by this program. Therefore, the tenant.json in the `tenant-onboarding` 
repo must be kept in sync with the tenant.json (`develop` branch) in the `sample-tenant-repo` (the template repo for the tenant's repo 
created by this program).

## Logs

Logs will be written out to logs/to-TIMESTAMP.log

## DB snippets

DB snippets (scripts) will be stored in the subdirectory: dbscripts

The directory will be created if it doesn't exist

For now it will overwrite contents if you run it more than once.

## Example commands

### Get help (I'm not doing "Get Help")

to -h

### Full shabang

to -trdv  OR to -a

### Only DB files

to -d

## How to get started with Rust

Search for information online.

### Get System up

### Setup debugger

In VSCode set up launch.json in the same directory.

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "(Windows) Launch",
            "type": "cppvsdbg",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/tenant-boarding.exe",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceRoot}",
            "environment": [],
            "externalConsole": true
        },
        {
            "name": "(OSX) Launch",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceRoot}/target/debug/tenant-onboarding",
            "args": ["-te"],
            "cwd": "${workspaceRoot}",
        }
    ]
}
```

### Basic build commands

### Useful links
