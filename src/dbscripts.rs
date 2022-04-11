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
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

/*
    take the parsed config.yaml file as an object
    build the script file & save to disk
*/
pub fn create_dbscripts(execute: bool, yaml: &Vec<Yaml>) {
    log::info!("dbscript Starting");

    if (execute) {
        println!("executing create_dbscripts");
    }else {
        println!("create_dbscripts dry");
    }

    let mut file = File::create("dbscript.js").expect("Unable to create dbscript.js"); //replace filename

    let y = &yaml[0];
    //println!("obj {}", tenant_name[0].as_str().unwrap());
    println!("{:?}", y); //hashstring is printed
    println!("the tenant name{:?}", y["tenantName"].as_str().unwrap());

    //let t = format!("{}", y["tenantName"].as_str());

    //file.write_all(t.as_bytes()).expect("failed to write");
}