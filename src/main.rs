#![allow(unused)]

/*
TODO

*/

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
mod gitsource { pub mod gitutils; }
mod dbscripts;
 
 
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
    //println!("timestamp: {}", now.format("%Y-%m-%d_%H_%M_%S").to_string());
    debug!("timestamp: {}", now.format("%Y-%m-%d_%H_%M_%S").to_string());

    let timestamp = now.format("%Y-%m-%d_%H_%M_%S").to_string();
    let filename = "to".to_string();

    let level = log::LevelFilter::Info;
    let file_path = format!("{}-{}.log", filename, timestamp);

    let config_file = "../../tenant-config.yaml";
    let mut do_team = false;
    let mut do_repo = false;
    let mut execute = false;
    let mut dbscripts = false;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build(file_path)
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
            arg!(dbscripts: -d --dbscripts "create db scripts"),
            arg!(execute: -e --execute "execute for real.  without it will just be a dry run")
        ]).get_matches();

    //reading YAML file...
    println!("cwd {:?}", std::env::current_dir());
    println!("cwd exec {:?}", std::env::current_exe());

    let mut file = std::fs::File::open(config_file)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("unable to read string");
    let yaml = YamlLoader::load_from_str(&contents).unwrap();
    
    //let d: String = serde_yaml::to_string(f)?;
    println!("Read YAML string: {}", contents);

    //set the flags you need
    if args.is_present("execute") {
        execute = true;
        println!("execute flag {}", execute);
    }
    else {
        println!("Doing DRY run.");
        info!("Doing DRY run");
    }

    if args.is_present("do_team") {
        do_team = true;

        team::do_team(execute, &yaml);
        gitsource::gitutils::get_github_team(); 
       
        
    }

    if args.is_present("do_repo") {
        do_repo = true;
        println!("do_repo flag {}", do_repo);
    }

    if args.is_present("dbscripts") {
        dbscripts = true;
        println!("dbscripts flag {}", dbscripts);

        dbscripts::create_dbscripts(execute, &yaml);
    }

    //now call each function corresponding to the flags
    //remember that passing in EXECUTE will control if that actually runs
    //team::do_team(execute);
    //repo::do_repo(execute);
    //dbscripts::dbscripts(execute);

    error!("Goes to stderr and file");
    warn!("Goes to stderr and file");
    info!("Goes to stderr and file");
    debug!("Goes to file only");
    trace!("Goes to file only");

    info!("Tenant Onboarding Stop");
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
