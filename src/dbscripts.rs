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
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

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

    let y = &yaml[0];

    println!("{:?}", y["studio"]["runbox"]["sandboxType"].as_str().unwrap());
    
    // To Do: Create the Insert DB script from the above contents in a string


    //Create the db-script files 
    //To do: Rename the sccript files based on the sprint name passed as input.
    let mut dev_db_script_file = File::create("../../db-scripts/dev_db_script.js").expect("Unable to create dev_dbscript.js");;
    let mut qa_db_script_file = File::create("../../db-scripts/qa_db_script.js").expect("Unable to create qa_dbscript.js");;
    let mut stage_db_script_file = File::create("../../db-scripts/stage_db_script.js").expect("Unable to create stage_dbscript.js");;
    let mut perf_db_script_file = File::create("../../db-scripts/perf_db_script.js").expect("Unable to create perf_dbscript.js");;
    let mut prod_db_script_file = File::create("../../db-scripts/prod_db_script.js").expect("Unable to create prod_dbscript.js");;

    // Read the script file names here
    let path_to_read = Path::new("../../db-scripts/dev_db_script.js");

    // Write the contents in the db script files one by one.. this is a test content
    fs::write("../../db-scripts/dev_db_script.js", y["studio"]["runbox"]["sandboxType"].as_str().unwrap()); // pass the Insert script here

    // To Do: Once the file is created, download the file in your local machine


    
}

/*
- setup connection to db.  use settings.yaml for data
- set the proper db
- run the dev script job
- close connection & clean up
*/
#[tokio::main]
pub async fn insert_dbscripts(execute: bool) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Insert Starting");

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("").await?;

    // Manually set an option.
    client_options.app_name = Some("Tenant Rust Program".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("test");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    if (!execute) {
        println!("End Dry Run.  Nothing was inserted");
        return Ok(());
    }

    //do insert

    log::info!("insert_dbscripts end");
    Ok(())
}