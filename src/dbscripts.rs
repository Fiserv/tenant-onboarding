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
use same_file::Handle;
use std::fs;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

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

    // let mut file = File::create("dbscript.js").expect("Unable to create dbscript.js"); //replace filename

    // let y = &yaml[0];
    // //println!("obj {}", tenant_name[0].as_str().unwrap());
    // //println!("{:?}", y); //hashstring is printed
    // println!("the tenant name{:?}", y["tenantName"].as_str().unwrap());

    // //let t = format!("{}", y["tenantName"].as_str());

    // //file.write_all(t.as_bytes()).expect("failed to write");

    let mut file = std::fs::File::open("../../tenant-config.yaml").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Debug support
    //println!("{:?}", doc);
    
    // printing the contents to test.. will be removed later
    println!("{:?}", doc["tenantName"].as_str().unwrap());
    println!("{:?}", doc["prettyTenantName"].as_str().unwrap());
    println!("{:?}", doc["github"]["repoName"].as_str().unwrap());
    println!("{:?}", doc["github"]["teamName"].as_str().unwrap());
    println!("{:?}", doc["studio"]["active"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["beta"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["internal"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["solutions"]["merchants"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["solutions"]["financialInstitutions"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["runbox"]["sandbox"].as_bool().unwrap());
    println!("{:?}", doc["studio"]["runbox"]["sandboxType"].as_str().unwrap());
    
    // To Do: Create the Insert DB script from the above contents in a string


    //Create the db-script files 
    //To do: Rename the sccript files based on the sprint name passed as input.
    let mut dev_db_script_file = File::create("../../db-scripts/dev_db_script.js");
    let mut qa_db_script_file = File::create("../../db-scripts/qa_db_script.js");
    let mut stage_db_script_file = File::create("../../db-scripts/stage_db_script.js");
    let mut perf_db_script_file = File::create("../../db-scripts/perf_db_script.js");
    let mut prod_db_script_file = File::create("../../db-scripts/prod_db_script.js");

    // Read the script file names here
    let path_to_read = Path::new("../../db-scripts/dev_db_script.js");

    // Write the contents in the db script files one by one
    fs::write("../../db-scripts/dev_db_script.js", doc["tenantName"].as_str().unwrap()); // pass the Insert script here

    // To Do: Once the file is created, download the file in your local machine


    
}
