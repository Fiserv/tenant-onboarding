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
    let has_apis = &(y["Tenant_Type"][0]["Full_service"].as_bool().unwrap() && !y["Tenant_Type"][1]["Doc_only"].as_bool().unwrap() && y["Runbox_essentials"]["Runbox"].as_bool().unwrap()); 
    //let link_out = y["Tenant Type"]["Doc only"].as_bool().unwrap().to_string();
    let internal_tag = &y["Studio_essentials"]["Internal"].as_bool().unwrap();
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

    // Read Tags: Customer Segments
    let mut segment_vector = Vec::new();
    
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Customer_segments"]["SMB"].as_bool().unwrap()) {
      segment_vector.push("SMB");
    }
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Customer_segments"]["Enterprise"].as_bool().unwrap()) {
      segment_vector.push("Enterprise");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Customer_segments"]["Bank"].as_bool().unwrap()) {
      segment_vector.push("Bank");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Customer_segments"]["Credit_Union"].as_bool().unwrap()) {
      segment_vector.push("Credit Union");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Customer_segments"]["Large_Financial_Institution"].as_bool().unwrap()) {
      segment_vector.push("Large Financial Institution");
    }

    let mut segments: String = String::new();
    let space = "','";
    let all_segments = segment_vector.len();
    if (all_segments > 0) {
      segments.push_str("'");
      for (i, segment) in segment_vector.iter().enumerate() {
          segments.push_str(segment);
          if i < all_segments -1 {
            segments.push_str(space);
          }
      }
      segments.push_str("'");
    }

    // Read Tags: Capabilities
    let mut capabilities_vector = Vec::new();
    
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Capabilities"]["Analytics"].as_bool().unwrap()) {
      capabilities_vector.push("Analytics");
    }
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Capabilities"]["Boarding_and_Management"].as_bool().unwrap()) {
      capabilities_vector.push("Boarding & Management");
    }
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Capabilities"]["Disbursements"].as_bool().unwrap()) {
      capabilities_vector.push("Disbursements");
    }
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Capabilities"]["Omni_Channel"].as_bool().unwrap()) {
      capabilities_vector.push("Omni-Channel");
    }
    if (y["Studio_essentials"]["Product_Areas"][0]["Payments"]["Capabilities"]["Value_Added_Services"].as_bool().unwrap()) {
      capabilities_vector.push("Value Added Services");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Capabilities"]["Banking_as_a_Service"].as_bool().unwrap()) {
      capabilities_vector.push("Banking as a Service");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Capabilities"]["Financial_Data_Management"].as_bool().unwrap()) {
      capabilities_vector.push("Financial Data Management");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Capabilities"]["Issuing"].as_bool().unwrap()) {
      capabilities_vector.push("Issuing");
    }
    if (y["Studio_essentials"]["Product_Areas"][1]["Banking"]["Capabilities"]["Licensing"].as_bool().unwrap()) {
      capabilities_vector.push("Licensing");
    }
    
    let mut capabilities: String = String::new();
    let space = "','";
    let all_capabilities = capabilities_vector.len();
    if (all_capabilities > 0) {
      capabilities.push_str("'");
      for (i, capability) in capabilities_vector.iter().enumerate() {
        capabilities.push_str(capability);
          if i < all_capabilities - 1 {
            capabilities.push_str(space);
          }
      }
      capabilities.push_str("'");
    }

    // Read Runbox essentials

    // Use mock server as default if: No APIs, both `mock` and `live` are checked, or neither is checked
    let mock_server = &(!*has_apis || y["Runbox_essentials"]["Sandbox"]["Type"]["mock"].as_bool().unwrap() || !y["Runbox_essentials"]["Sandbox"]["Type"]["live"].as_bool().unwrap());
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
    let contact_sales = &y["Studio_essentials"]["Contact_Sales"].as_bool().unwrap();
    
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
      category: 'Customer Segment', 
      value: 'Customer_Segment',
      tags: [" + &segments+  "],
    },    
    {
      category: 'Capability', 
      value: 'Capability',
      tags: ["+ &capabilities+  "],
    },
  ],   
  active: true,
  betaTag: false,
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
  ],
  contactSales: "+ if *contact_sales { concat!(true) } else { concat!(false)}+",
})");

  if (!execute) {
    println!("\n{} env DB Script for {}:\n{}", env_flag.trim(), name, &db_script_data);
    return;
  }
    let dbscript_path = format!("../../dbscripts/{}_{}", env_flag.trim(), "db_script.js".to_string());
    fs::write(dbscript_path, db_script_data);
}
