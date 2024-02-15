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
    log::info!("dbscript Starting for {}", env_flag);

    if (execute) {
        println!("executing create_dbscripts {}"  , env_flag.trim());
    } else {
        println!("create_dbscripts dry");
    }

    let mut path_to_read = Path::new("");
    let mut db_script_file_path = String::new();
    let mut github_main_branch = String::new();
    let mut gts_url = String::new();

    //Create the db-script files
    //To do: Rename the script files based on the sprint name passed as input.

    if ("dev".eq(env_flag.trim())
        || "qa".eq(env_flag.trim())
        || "perf".eq(env_flag.trim())
        || "stage".eq(env_flag.trim())
        || "production".eq(env_flag.trim()))
    {
        db_script_file_path =  "../../dbscripts/".to_string() + env_flag.trim() + &"_db_script.js".to_string();
        path_to_read = Path::new(&db_script_file_path);
        gts_url = "https://tenant-generic.".to_string() + env_flag.trim() + &"-developer-portal.svc".to_string();
        println!("Value of gts_dev_url: {}", gts_url);
    } else {
        // environment value mismatched
        println!("Incorrect environment value");
    }

    match env_flag.trim() { 
      "stage" => github_main_branch = "stage".to_string(), 
      "production" => github_main_branch = "main".to_string(), 
      _ => github_main_branch = "develop".to_string(), 
    };

    // Read the values from tenant-onboarding-form.yaml file
    let y = &yaml[0];

    // Read Product name
    let title = y["Tenant_Title"].as_str().unwrap().to_string();
    let name = y["Tenant_Name"].as_str().unwrap().to_string();
    let github_repo_name = name.to_case(Case::Kebab);
   
    // Read Tenant Type
    //let full_service = y["Tenant Type"]["Full service"].as_bool().unwrap().to_string(); 
    let has_apis = &(y["Tenant_Type"][0]["Full_service"].as_bool().unwrap() && !y["Tenant_Type"][1]["Doc_only"].as_bool().unwrap()); 
    //let link_out = y["Tenant Type"]["Doc only"].as_bool().unwrap().to_string();
    let internal_tag = &y["Studio_essentials"]["Internal"].as_bool().unwrap() ;
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

    let mut region_of_operations: String = String::new();
    let all_regions = regions_vector.len();
    if (all_regions > 0) {
      let space = "','";
      region_of_operations.push_str("'");
      for (i, region) in regions_vector.iter().enumerate() {
          region_of_operations.push_str(region);
          if i < all_regions-1 {
              region_of_operations.push_str(space);
          }
      }
      region_of_operations.push_str("'");
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

    let mut integrations: String = String::new();
    let space = "','";
    let all_integrations = integration_vector.len();
    if (all_integrations > 0) {
      integrations.push_str("'");
      for (i, integration) in integration_vector.iter().enumerate() {
          integrations.push_str(integration);
          if i < all_integrations -1 {
              integrations.push_str(space);
          }
      }
      integrations.push_str("'");
    }
 
    let mut industries: String = String::new();
    // Read Tags: Industry
    if (y["Studio_essentials"]["Tags"]["Industry"].as_str() != None) {
      let mut industry_vector = Vec::new();
      industry_vector.push(y["Studio_essentials"]["Tags"]["Industry"].as_str().unwrap().to_string());
      
      let space = "','";
      let all_industries = industry_vector.len();
      industries.push_str("'");
      for (i, industry) in industry_vector.iter().enumerate() {
        industries.push_str(industry);
        if i < all_industries -1 {
          industries.push_str(space);
        }
      }
      industries.push_str("'");
    }
    
    // Read Runbox essentials
    let mock_server = &(!*has_apis || !y["Runbox_essentials"]["Sandbox"]["Type"]["live"].as_bool().unwrap());
    let live_server_url = y["Runbox_essentials"]["Sandbox"]["Live_Sandbox_details"]["API_gateway_details"]["Server_URL"].as_str().unwrap().to_string();
    let live_auth_type= y["Runbox_essentials"]["Sandbox"]["Live_Sandbox_details"]["API_gateway_details"]["Authentication_Type"].as_str().unwrap().to_string();
    let live_self_signed_cert = y["Runbox_essentials"]["Sandbox"]["Live_Sandbox_details"]["API_gateway_details"]["Self_signed_certificate"].as_bool().unwrap();
    let live_sandbox = "liveSandbox: {
        serverUrl: '".to_string() + &live_server_url + "',
        authenticationScheme: '" + &live_auth_type +"',
        username: '',
        password: '',
        selfSignedCert: "+ &live_self_signed_cert.to_string() +"
      }";

    let db_script_data = String::from("db.tenants.insertOne({
  title: '".to_owned()+ &title + "',
  name: '"+ &name +"',
  tenantHost: '"+ &gts_url+ "',
  tenantPort: '8443',
  providerAPIUrl: '/v1/products/"+&name+"',
  apiAuth: {},
  hasApis: "+ if *has_apis { concat!(true) } else { concat!(false)}+",
  productTags: [
    {
      category: 'Region', 
      value: 'Region',
      tags: ["+ &region_of_operations+  "],
    },   
    {
      category: 'Integration Type', 
      value: 'Integration Type',
      tags: [" + &integrations+  "],
    },  
    {
      category: 'Industry', 
      value: 'Industry',
      tags: [" + &industries+  "],
    },    
  ],   
  active: true,
  betaTag: true,
  internalTag: "+ if *internal_tag { concat!(true) } else { concat!(false)}+",
  github: '" + &github_repo_name + "',
  selfServiceFeatures: [
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
  ], 
  gitHubFeatureBranches: [
    {
      name: 'active',
      value: '" + &github_main_branch + "',
      available: true,
      hasApis: "+ if *has_apis { concat!(true) } else { concat!(false)}+",
      sandboxType: '"+ if *mock_server {"mock"} else {"live"} +"',
      " + if *mock_server {"mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun',"} else {&live_sandbox}+"
    },
    {
      name: 'previous',
      value: 'previous',
      available: false,
      hasApis: false,
      sandboxType: '"+ if *mock_server {"mock"} else {"live"} +"',
      " + if *mock_server {"mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun',"} else {&live_sandbox}+"
    },
    {
      name: 'preview',
      value: 'preview',
      available: false,
      hasApis: false,
      sandboxType: '"+ if *mock_server {"mock"} else {"live"} +"',
      " + if *mock_server {"mockServerUrl: 'http://tenant-generic-mock-service:8443/sandboxrun',"} else {&live_sandbox}+"
    }
  ]
})");

  if (!execute) {
    println!("\n{} env DB Script for {}:\n{}", env_flag.trim(), name, &db_script_data);
    return;
  }
    //Write the contents in the db script files one by one.. this is a test content
    let dbscript_path = format!("../../dbscripts/{}_{}", env_flag.trim(), "db_script.js".to_string());
    fs::write(dbscript_path, db_script_data);
    //fs::write(path_to_read, dev_db_script);

    //ToDo: Once the file is created, download the file in your local machine

    //ToDo: Format the db script file
    // command: sane-fmt --write db-scripts/dev_db_script.js
}

// fn type_of(_: T) -> &'static str {
//     type_name::()
// }