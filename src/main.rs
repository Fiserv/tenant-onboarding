#![allow(unused)]

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;

use chrono::{DateTime, Local};

use clap::{App, Arg, Parser};
use clap::arg;

use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
mod team;
mod gitsource { pub mod gitrepo; pub mod gitteam; pub mod githooks; pub mod authtoken; pub mod gitbranches; }
mod dbscripts;
use tokio;
use futures::executor::block_on;

 
 
//mod dbscripts;

/// Search for a pattern in a file and display the lines that contain it.
/*#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("Tenant Onboarding Starting");

    let now: DateTime<Local> = Local::now();
    debug!("timestamp: {}", now.format("%Y-%m-%d_%H_%M_%S").to_string());
    let timestamp = now.format("%Y-%m-%d_%H_%M_%S").to_string();
    let level = log::LevelFilter::Info;
    let log_file_path = format!("{}_{}.log", "tenant-onboarding".to_string(), timestamp);

    //let config_file = "../../tenant-config.yaml";
    let config_file = "../../Tenant-Onboarding-Form.yaml";
    let settings_file = "../../settings.yaml";

    let mut do_team   = false;
    let mut do_repo   = false;
    let mut do_hooks  = false;
    let mut do_branches = false;
    let mut execute   = false;
    let mut dbscripts = false;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build(log_file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?;

    let args = App::new("Tenant Boarding")
        .author("DevStudio Team")
        .about("Use this to setup a tenant")
        .args(&[
            Arg::new("do_team")
                .short('t')
                .long("do_team")
                .takes_value(false)
                .help("create github team name"),
            arg!(do_repo: -r --do_repo "create github repo"),
            arg!(do_hooks: -h --do_hooks "create github repo"),
            arg!(dbscripts: -d --dbscripts "create db scripts"),
            arg!(do_branches: -b --branches "add branch protection"),
            arg!(execute: -e --execute "execute for real.  without it will just be a dry run")
        ]).get_matches();

    //reading YAML file...
    //println!("cwd {:?}", std::env::current_dir());
    //println!("cwd exec {:?}", std::env::current_exe());

    let mut config_file_content = std::fs::File::open(config_file)?;
    let mut config_contents = String::new();

    config_file_content.read_to_string(&mut config_contents).expect("unable to read Tenant Config Yaml string");
    let yaml_config = YamlLoader::load_from_str(&config_contents).unwrap();

    let mut settings_file_content = std::fs::File::open(settings_file)?;
    let mut settings_contents = String::new();

    settings_file_content.read_to_string(&mut settings_contents).expect("unable to read Tenant Config Yaml string");
    let yaml_settings = YamlLoader::load_from_str(&settings_contents).unwrap();
    
    //let d: String = serde_yaml::to_string(f)?;
   // println!("Read YAML string: {}", config_contents);
 
    //set the flags you need
    if args.is_present("execute") {
        execute = true;
        info!("execute flag {}", execute);
    }
    else { 
        info!("Doing DRY run");
    }

    if args.is_present("do_repo") {
        do_repo = gitsource::gitrepo::create_repo(&yaml_config, &yaml_settings, execute).unwrap();
        info!("REPO CREATED-----: {:#?}\n",  do_repo);
    }

    if args.is_present("do_hooks") { 
        do_hooks = gitsource::githooks::add_hooks_repo(&yaml_config, &yaml_settings, execute).unwrap(); 
        info!("WEBHOOKS ADDED-----: {:#?}\n", do_hooks);
     }

    if args.is_present("do_team") {
        //team::do_team(execute, &yaml_config);
        do_team = gitsource::gitteam::process_github_team(&yaml_config, &yaml_settings, execute).unwrap(); 
        info!("TEAM CREATED-----: {:#?}\n",  do_team);
    }

    if args.is_present("do_branches") {
        do_branches = gitsource::gitbranches::process_github_branches(&yaml_config, &yaml_settings, execute).unwrap();
        info!("BRANCH PROTECTION ADDED-----: {:#?}\n", do_branches);
    }

    if args.is_present("dbscripts") {
        dbscripts = true;
        info!("dbscripts flag {}", dbscripts);

        dbscripts::create_dbscripts(execute, &yaml_config , "dev".to_string());
        //dbscripts::insert_dbscripts(execute);
    }

    //now call each function corresponding to the flags
    //remember that passing in EXECUTE will control if that actually runs
    //team::do_team(execute);
    //repo::do_repo(execute);
    //dbscripts::dbscripts(execute);

    /*error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");*/
    info!("<<<<<<<<TENANT ONBOARDING PROCESS COMPLETED>>>>>>>>>");
    if (do_repo && execute){
        let config = &yaml_config[0]; 
        let tenant_repo = config["GitHub_essentials"]["Repository_Name"].as_str().unwrap();
        println!("{}", tenant_repo);
    } 
 
    Ok(())

    /*let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

}
