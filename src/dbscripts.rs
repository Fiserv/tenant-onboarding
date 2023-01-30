//use ifmt::iprintln;
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
use mongodb::bson::doc;
use same_file::Handle;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::Path;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};
use serde::Deserialize;
//use boolvec::BoolVec;
use convert_case::{Case, Casing};

//use std::any::type_name;


/*
    take the parsed config.yaml file as an object
    build the script file & save to disk
*/
pub fn create_dbscripts(execute: bool, yaml: &Vec<Yaml>, env_flag: String) {
    log::info!("dbscript Starting");

    if (execute) {
        println!("executing create_dbscripts {}"  , env_flag.trim());
    } else {
        println!("create_dbscripts dry");
    }

    let mut path_to_read = Path::new("");
    let mut db_script_file_path = String::new();
    let mut gts_url = String::new();

    //Create the db-script files
    //To do: Rename the script files based on the sprint name passed as input.

    if ("dev".eq(env_flag.trim())
        || "qa".eq(env_flag.trim())
        || "perf".eq(env_flag.trim())
        || "stage".eq(env_flag.trim())
        || "prod".eq(env_flag.trim()))
    {
        db_script_file_path =  "../../db-scripts/".to_string() + env_flag.trim() + &"_db_script.js".to_string();
        path_to_read = Path::new(&db_script_file_path);
        gts_url = "https://tenant-generic.".to_string() + env_flag.trim() + &"-developer-portal.svc".to_string();
        println!("Value of gts_dev_url: {}", gts_url);
    } else {
        // environment value mismatched
        println!("Incorrect enviornment value");
    }

    // Read the values from tenant-onboarding-form.yaml file
    let y = &yaml[0];

    // Read Product name
    let title = y["Tenant_Name"].as_str().unwrap().to_string();
    let name = title.trim();
    let github_repo_name = name.to_case(Case::Kebab);
   
    // Read Tenant Type
    //let full_service = y["Tenant Type"]["Full service"].as_bool().unwrap().to_string(); 
     let has_apis = &y["Tenant_Type"][0]["Full_service"].as_bool().unwrap(); 
    //let link_out = y["Tenant Type"]["Doc only"].as_bool().unwrap().to_string();
    println!("has_apis {:?}",has_apis);
    // Read Tags: Region Of Operation
    let mut regions_vector = Vec::new();
    
    if true.eq(&y["Studio_essentials"]["Tags"]["Region_of_Operation"]["North_America"].as_bool().unwrap()) {
        regions_vector.push("North America");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Region_of_Operation"]["EMEA"].as_bool().unwrap()) {
        regions_vector.push("EMEA");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Region_of_Operation"]["LATAM"].as_bool().unwrap()) {
        regions_vector.push("LATAM");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Region_of_Operation"]["APAC"].as_bool().unwrap()) {
        regions_vector.push("APAC");
    }
    println!("{:?}",regions_vector);

    let mut region_of_operations: String = String::new();
    let space = "','";
    let all_regions = regions_vector.len();
    for (i, region) in regions_vector.iter().enumerate() {
        region_of_operations.push_str(region);
        if i < all_regions-1 {
            region_of_operations.push_str(space);
        }
    }

    // Read Tags: Integration
    let mut integration_vector = Vec::new();
    
    if true.eq(&y["Studio_essentials"]["Tags"]["Integration"]["restful"].as_bool().unwrap()) {
        integration_vector.push("restful");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Integration"]["sdk"].as_bool().unwrap()) {
        integration_vector.push("sdk");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Integration"]["soap"].as_bool().unwrap()) {
        integration_vector.push("soap");
    } 
    if true.eq(&y["Studio_essentials"]["Tags"]["Integration"]["xml"].as_bool().unwrap()) {
        integration_vector.push("xml");
    }
    println!("{:?}",integration_vector);

    let mut integrations: String = String::new();
    let space = "','";
    let all_integrations = integration_vector.len();
    for (i, integration) in integration_vector.iter().enumerate() {
        integrations.push_str(integration);
        if i < all_integrations -1 {
            integrations.push_str(space);
        }
    }
 
    // Read Tags: Industry
    let mut industry_vector = Vec::new();
    industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"].as_str().unwrap().to_string());
    //industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"]["Industry2"].as_str().unwrap().to_string());
    //industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"]["Industry3"].as_str().unwrap().to_string());
    //industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"]["Industry4"].as_str().unwrap().to_string());
    //industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"]["Industry5"].as_str().unwrap().to_string());
    println!("{:?}",industry_vector);
    
    let mut industries: String = String::new();
    let space = "','";
    let all_industries = industry_vector.len();
    for (i, industry) in industry_vector.iter().enumerate() {
        industries.push_str(industry);
        if i < all_industries -1 {
            industries.push_str(space);
        }
    }
    
    // Read Runbox essentials
    let mock_server = &y["Runbox_essentials"]["Sandbox"]["Type"]["mock"].as_bool().unwrap();


    let dev_db_script = String::from(
        "db.tenants.insertOne({
                title: ",
    ) + "'"
        + &title
        + "',"
        + "tenantHost: "
        + "'"
        + &gts_url
        + "',"
        + "tenantPort: "
        + "'8443',"
        + "providerAPIUrl: '/v1/products/SampleTenant',apiAuth: {},"
        +  "productTags: ["
        +  "{category: 'Region', 
        value: 'Region',
        tags: ['" 
        + &region_of_operations
        +  "'],"
        + "},"  // end of 'Region' tag
        +  "{category: 'Integration Type', 
        value: 'Integration Type',
        tags: ['" 
        + &integrations
        +  "'],"
        + "},"  // end of 'Region' tag
        + "{category: 'Industry', 
        value: 'Industry',
        tags: ['" 
        + &industries
        +  "'],"
        + "},"  // end of 'industry' tag
        + "]," // end of 'ProductTags'
        + "active: true,
        betaTag: true,
        internalTag: true,
        name: '"
        + &name //CloudAccelerationCenter
        + "',"
        + "github: '"
        + &github_repo_name //cloud-acceleration-center
        + "',"
        + "selfServiceFeatures: [
            {
              featureName: 'Explore documentation',
              featureUrl: 'Support/docs/?path=docs/explore-documentation.md',
              active: true,
            },
            {
              featureName: 'API experimentation with Runbox',
              featureUrl: 'Support/docs/?path=docs/try-out-the-api-sandbox.md',
              active: false,
            },
            {
              featureName: 'Generate Credentials',
              featureUrl: '',
              active: false,
            },
            {
              featureName: 'Testing & Certification',
              featureUrl: '',
              active: false,
            },
          ],"
        + "gitHubFeatureBranches: [
            {
                name: 'active',
                value: 'develop',
                available: true,
                hasApis: " + if *has_apis { concat!(true) } else { concat!(false)}
                +", sandboxType: " + if *mock_server { concat!(true)} else { concat!(false)} +",
                mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun'
            },
            {
                name: 'previous',
                value: 'previous',
                available: false,
                hasApis: false,
                sandboxType: 'GMS',
                mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun'
            },
            {
                name: 'preview',
                value: 'preview',
                available: false,
                hasApis: false,
                sandboxType: 'GMS',
                mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun'
            }
            ],"
        + "});";

    //Write the contents in the db script files one by one.. this is a test content
    let tenant_db_script = format!("{}{}_{}","../../db-scripts/".to_string()  ,name, "dev_db_script.js".to_string()); 
    fs::write(tenant_db_script, dev_db_script);
    //fs::write(path_to_read, dev_db_script);

    //ToDo: Once the file is created, download the file in your local machine

    //ToDo: Format the db script file
    // command: sane-fmt --write db-scripts/dev_db_script.js
}

// fn type_of(_: T) -> &'static str {
//     type_name::()
// }