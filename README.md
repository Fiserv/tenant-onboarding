# Overview

This program is used to onboard new tenants. It automates the process of creating a new repo, adding webhooks, branch protection, etc. It is invoked by the Fiserv [Support workflow](https://github.com/Fiserv/Support/blob/main/.github/workflows/run-onboarding-service.yaml)

The program provides the following functionality:

1. Create a github team (-t) -- Abhishek
2. Create a github repo from the sample repo using the supplied name (-r) -- Abhishek
3. Setup the github webooks (-h) -- Abhishek
4. Create Ruleset/Branch protection (-b) -- Minh
5. Add the devstudio and tenant github teams to the repo -- Abhishek
6. Update the basic files to have the proper names - Abhishek 
   - Fill out tenant.json and product-layout.yaml file. 
7. Generate the necessary DB snippets needed (-d)  -- Tania
8. (delete coming later...) -- Tania
9. (integrate with GitLab to commit DB changes coming later) -- Tania

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

config.yaml will hold specific info that will be used in the repo & files.

TO will look for it at the same level.

## Logs

Logs will be written out to logs/to-TIMESTAMP.log

## DB snippets

To make life easier TO will generate the DB snippets you neee to use.

They will be stored in the subdirectory: dbscripts

Don't worry, we'll make the folder if you don't have one.

For now it will overwrite contents if you run it more than once.

## Example commands

### Get help (I'm not doing "Get Help")

to -h

### Full shabang

to -trdv  OR to -a

### Only DB files

to -d

## Owners

1. skeleton = alvin
   1. logging = alvin
2. github calls = abhishek
3. create github team = abhishek
   1. assign devstudio members
   2. assign tenant members (fiserv emails only)
4. create github repo from template repo = abhishek
   1. set team
   2. set webhooks
   3. add github validators
5. dbscripts = tania
6. prep tenant repo = tania
   1. fill in tenant.json
   2. fill in product.yaml


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
