#!/usr/bin/env node

const fs = require("fs");
const yaml = require("js-yaml");
const args = process.argv.slice(2);
const {
  errorMessage,
  printMessage,
  tenant_enum,
  tenant_type_enum,
  regions,
  convertToKebabCase,
  convertToCamelCase
} = require("./tools");
const md = require("markdown-it")();
const html2json = require("html2json").html2json;
const axios = require("axios");

const fsPromises = fs.promises;

const tenant_onboarding_file = "../Tenant-Onboarding-Form.yaml";
const tenant_json_file = "../tenant.json";
const settings_yaml = "../settings.yaml";

const tenantConfigurator = async (issueNo) => {
  let check = true;
  try {
    const tenant_yaml = fs.readFileSync(
      tenant_onboarding_file,
      "utf8"
    );
    const yamlData = await yaml.load(tenant_yaml); 
    const url = `https://api.github.com/repos/Fiserv/Support/issues/${issueNo}`;
    
    const config = {
      headers: {
        "User-Agent": "tenant-onbaording",
        Accept: "application/vnd.github+json" 
      },
    };

    await axios({
      method: "GET",
      url,
      config,
    })
      .then(function (response) {
        const tenantConfig = response?.data?.body;
        const md_result = md.render(tenantConfig);
        const result_data = html2json(md_result); 

        let last_title;
        for (const obj of result_data.child) {
          if (obj?.node === "element") {
            // Parsing title
            if (obj?.tag === "h3") {
              last_title = obj?.child[0].text?.trim();
            }
            try {
              if (obj?.tag === "p") {
                
                const tagValue = obj?.child[0].text?.trim();
 
                switch (last_title) {
                  case tenant_enum.TENANT_NAME:
                    { 
                        yamlData["Tenant_Title"] = tagValue;
                        yamlData["Tenant_Name"] = convertToCamelCase(tagValue);
                      if ( yamlData["GitHub_essentials"].Repository_Name != undefined && tagValue != undefined) {
                        yamlData["GitHub_essentials"].Repository_Name = convertToKebabCase(tagValue);
                      }
                    }
                    break;

                  case tenant_enum.TENANT_MEMBERS:
                    {
                      if ( yamlData["GitHub_essentials"].Team_Members != undefined && tagValue != undefined ) {
 
                        const arr = tagValue.split(", ");
                        const qarr = arr.map(item => item);
                        yamlData["GitHub_essentials"].Team_Members = tagValue;
                      }
                    }
                    break;

                  case tenant_enum.BUG_REPORTING:
                    {
                      if (  yamlData.GitHub_essentials.Support[0].Bug_Reporting.Assignees != undefined && tagValue != undefined ) {
                        yamlData.GitHub_essentials.Support[0].Bug_Reporting.Assignees = tagValue;
                      }
                    }
                    break;

                  case tenant_enum.FEEDBACK_REPORTING:
                    {
                      if ( yamlData.GitHub_essentials.Support[1].Feedback_Reporting .Assignees != undefined && tagValue != undefined) {
                        yamlData.GitHub_essentials.Support[1].Feedback_Reporting.Assignees =
                          tagValue;
                      }
                    }
                    break;

                  case tenant_enum.ENHANCEMENT_REPORTING:
                    {
                      if ( yamlData.GitHub_essentials.Support[2].Enhancement_Reporting.Assignees != undefined && tagValue != undefined) {
                        yamlData.GitHub_essentials.Support[2].Enhancement_Reporting.Assignees = tagValue;
                      }
                    }
                    break;

                  case tenant_enum.EXTERNAL_LINK:
                    {
                      if ( yamlData.Studio_essentials.External_link != undefined  && tagValue != undefined) {
                        yamlData.Studio_essentials.External_link = tagValue;
                      }
                    }
                    break;

                  case tenant_enum.INTERNAL_TENANT:
                    {
                      if (yamlData.Studio_essentials.Internal != undefined && tagValue != undefined) {
                        if (tagValue === "No") {
                          yamlData.Studio_essentials.Internal = false;
                        } else {
                          yamlData.Studio_essentials.Internal = true;
                        }
                      }
                    }
                    break;

                  case tenant_enum.TENANT_TYPE:
                    {
                      if (yamlData.Studio_essentials.Internal != undefined) {
                        switch (tagValue) {
                          case tenant_type_enum.FULL_SERVICE:
                            if (yamlData["Tenant_Type"][0].Full_service != undefined) {
                              yamlData["Tenant_Type"][0].Full_service = true;
                            }
                            break;
                          case tenant_type_enum.DOC_ONLY:
                            if (yamlData["Tenant_Type"][1].Doc_only != undefined) {
                              yamlData["Tenant_Type"][1].Doc_only = true;
                            }
                            break;
                          case tenant_type_enum.LINK_OUT:
                            if (yamlData["Tenant_Type"][2].Link_out != undefined) {
                              yamlData["Tenant_Type"][2].Link_out = true;
                            }
                            break;
                        }
                      }
                    }
                    break;

                  case tenant_enum.CARAT_PRODUCT:
                    {
                      if (yamlData.Studio_essentials.Carat != undefined && tagValue != undefined) {
                        if (tagValue === "No") {
                          yamlData.Studio_essentials.Carat = false;
                        } else {
                          yamlData.Studio_essentials.Carat = true;
                        }
                      }
                    }
                    break;

                  case tenant_enum.RESTFUL:
                    {
                      if (
                        yamlData.Studio_essentials.Tags.Integration.restful != undefined  && tagValue != undefined) {
                        if (tagValue === "Yes") {
                          yamlData.Studio_essentials.Tags.Integration.restful = true;
                        } else {
                          yamlData.Studio_essentials.Tags.Integration.restful = false;
                        }
                      }
                    }
                    break;

                  case tenant_enum.SDK:
                    {
                      if (
                        yamlData.Studio_essentials.Tags.Integration.sdk != undefined  && tagValue != undefined) {
                        if (tagValue === "Yes") {
                          yamlData.Studio_essentials.Tags.Integration.sdk = true;
                        } else {
                          yamlData.Studio_essentials.Tags.Integration.sdk = false;
                        }
                      }
                    }
                    break;

                  case tenant_enum.INDUSTRY:
                    {
                      if ( yamlData.Studio_essentials.Tags.Industry != undefined && tagValue != undefined) {
                          yamlData.Studio_essentials.Tags.Industry = tagValue;
                      } 
                    }
                    break;

                  case tenant_enum.RUNBOX:
                    {
                      if (yamlData.Runbox_essentials.Runbox != undefined && tagValue != undefined) {
                        if (tagValue === "Yes") {
                          yamlData.Runbox_essentials.Runbox = true;
                        } else {
                          yamlData.Runbox_essentials.Runbox = false;
                        }
                      }
                    }
                    break;

                  case tenant_enum.MOCK_SANDBOX:
                    {
                      if (
                        yamlData.Runbox_essentials.Sandbox.Type.mock != undefined && tagValue != undefined) {
                        if (tagValue === "Yes") {
                          yamlData.Runbox_essentials.Sandbox.Type.mock = true;
                        } else {
                          yamlData.Runbox_essentials.Sandbox.Type.mock = false;
                        }
                      }
                    }
                    break;

                  case tenant_enum.LIVE_SERVER:
                    {
                      if (
                        yamlData.Runbox_essentials.Sandbox.Type.live !=  undefined  && tagValue != undefined) {
                        if (tagValue === "Yes") {
                          yamlData.Runbox_essentials.Sandbox.Type.live = true;
                        } else {
                          yamlData.Runbox_essentials.Sandbox.Type.live = false;
                        }
                      }
                    }
                    break;
                }
              } else if (obj?.tag === "ul") {
                for (const chd of obj?.child) {
                  if (chd?.node === "element") {
                    const val = JSON.stringify(chd?.child[0]?.text?.trim());
                    if (val.includes("[X]")) {
                      const len = val.length;
                      const tagValue = val.slice(4, len - 1).trim();

                      switch (last_title) {
                        case tenant_enum.CUSTOMER_SEGMENTS_FOR_MERCHANTS:
                          {
                            if (tagValue === "SMB") {
                              if ( yamlData.Studio_essentials.Product_Areas[0] .Merchants.Customer_segments.SMB != undefined ){
                                  yamlData.Studio_essentials.Product_Areas[0].Merchants.Customer_segments.SMB = true;
                              }else{
                                yamlData.Studio_essentials.Product_Areas[0].Merchants.Customer_segments.SMB = false;
                              }
                            }

                            if (tagValue === "Enterprise") {
                              if ( yamlData.Studio_essentials.Product_Areas[0].Merchants.Customer_segments.Enterprise != undefined ) {
                                yamlData.Studio_essentials.Product_Areas[0].Merchants.Customer_segments.Enterprise = true;
                              }
                            }
                          }
                          break;

                        case tenant_enum.CUSTOMER_SEGMENTS_FOR_FINANCIAL_INSTITUTIONS:
                          {
                            if (tagValue === "Bank") {
                              if (  yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Banks != undefined) {
                                yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Banks = true;
                              }
                            }

                            if (tagValue === "Credit Union") {
                              if ( yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Credit_Unions != undefined) {
                                yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Credit_Unions = true;
                              }
                            }

                            if (tagValue === "Large Financial Institution") {
                              if ( yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Large_financial_instutitions != undefined ) {
                                yamlData.Studio_essentials.Product_Areas[1].Financial_Institutions.Customer_segments.Large_financial_instutitions = true;
                              }
                            }
                          }
                          break;
                        case tenant_enum.REGION_OF_OPERATION:
                          {
                            switch (tagValue) {
                              case regions.NORTH_AMERICA:
                                {
                                  if (  yamlData.Studio_essentials.Tags.Region_of_Operation.North_America != undefined) {
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.North_America = true;
                                  }
                                }
                                break;

                              case regions.EMEA:
                                {
                                  if (
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.EMEA != undefined) {
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.EMEA = true;
                                  }
                                }
                                break;

                              case regions.LATAM:
                                {
                                  if (
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.LATAM != undefined) {
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.LATAM = true;
                                  }
                                }
                                break;

                              case regions.APAC:
                                {
                                  if (
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.APAC != undefined) {
                                    yamlData.Studio_essentials.Tags.Region_of_Operation.APAC = true;
                                  }
                                }
                                break;
                            }
                          }
                          break;
                      }
                    }
                  }
                }
              }

              fs.writeFileSync(
                tenant_onboarding_file,
                yaml.dump(yamlData)
              );
            } catch (err) {
              console.error(err);
              check = false;
            }
          }
        }
      })
      .catch(function (error) {
        errorMessage(error);
        check = false;
      });
  } catch (e) {
    errorMessage("Unable to call", e.message);
    check = false;
  }

  return check;
};


  
async function updateTenantJSONFile() {
 
  const tenant_yaml = fs.readFileSync(
    tenant_onboarding_file,
    "utf8"
  );
  const yamlData = await yaml.load(tenant_yaml);

  //printMessage(JSON.stringify(yamlData, null, 2));

  const tenant_json = fs.readFileSync(tenant_json_file, "utf8");
  let tenant_Data = JSON.parse(tenant_json);

  if (yamlData.Tenant_Name != undefined) {
    tenant_Data.title = yamlData.Tenant_Title;
    tenant_Data.name = yamlData.Tenant_Name;
    tenant_Data.product.apiSpecification = `/v1/apis/${yamlData.Tenant_Name}`;
    tenant_Data.product.layout = `/v1/layouts/${yamlData.Tenant_Name}`;
    tenant_Data.product.documentation = `/v1/docs/${yamlData.Tenant_Name}`;
    tenant_Data.product.documenttree = `/v1/docs/${yamlData.Tenant_Name}`;
    tenant_Data.product.documenttreeV2 = `/v2/docs/${yamlData.Tenant_Name}`;
    tenant_Data.product.docsCount = `/v2/docs/count/${yamlData.Tenant_Name}`;
    tenant_Data.product.sandbox = `/v2/sandboxrun/${yamlData.Tenant_Name}`;
    tenant_Data.product.accessConfig = `/v1/fileAccess/${yamlData.Tenant_Name}`;
    tenant_Data.product.assets = `/v1/assets/${yamlData.Tenant_Name}`;
  }

  if (  yamlData.GitHub_essentials.Support[0].Bug_Reporting.Assignees != undefined ) {
    tenant_Data.supportConfig[0].bug.assignees = yamlData.GitHub_essentials.Support[0].Bug_Reporting.Assignees;
  }

  if ( yamlData.GitHub_essentials.Support[1].Feedback_Reporting.Assignees != undefined ) {
    tenant_Data.supportConfig[0].feedback.assignees = yamlData.GitHub_essentials.Support[1].Feedback_Reporting.Assignees;
  }
  if (  yamlData.GitHub_essentials.Support[2].Enhancement_Reporting.Assignees != undefined  ) {
    tenant_Data.supportConfig[0].enhancement.assignees = yamlData.GitHub_essentials.Support[2].Enhancement_Reporting.Assignees;
  }
  return new Promise((resolve, rejects) => {

   // printMessage(JSON.stringify(tenant_Data, null, 2));

    fsPromises .writeFile(tenant_json_file, JSON.stringify(tenant_Data, null, 2))
      .then(() => {
        resolve(convertToKebabCase(tenant_Data.title));
      })
      .catch((err) => {
        rejects(false);
      });
  }); 
}




async function service() {
  let check = false;
  let tname = '';
  try {
    //printMessage(`Gihub Issue No. ---->>> ${args}`);
    if (args?.length > 0) {
      check = await tenantConfigurator(args);
      
      if (check) {
         tname = await updateTenantJSONFile();
        
      }
      
    }
  } catch (e) {
    errorMessage("FAILED", e.message);
    check = false;
  }
  if(check && tname){
    printMessage(tname);
  }
  
  return check;
}

if (require.main === module) {
  service();
}
