//mod github;
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
use yaml_rust::{Yaml, YamlLoader, YamlEmitter};

pub fn do_team(execute: bool, yaml: Vec<Yaml>) {
    error!("do_team Goes to stderr and file");
    warn!("do_team Goes to stderr and file");
    info!("do_team Goes to stderr and file");
    debug!("do_team Goes to file only");
    trace!("do_team Goes to file only");

    log::info!("Hello, world!");

    println!("doTeam dry");

    if (execute) {
        println!("executing doTeam");
    }

    let y = &yaml[0];
    //println!("obj {}", tenant_name[0].as_str().unwrap());
    println!("{:?}", y); //hashstring is printed
    println!("the tenant name{:?}", y["tenantName"].as_str().unwrap());

    /*for x in &yaml {
        println!("{}", x);
    }*/
}