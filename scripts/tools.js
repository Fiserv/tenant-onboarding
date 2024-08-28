const processArgs = (args = []) => {
  const argsAndValues = {};
  for (let i = 0; i < args.length; i++) {
    const arg = args[i];
    if (/^--.+/.test(arg)) {
      const key = arg.match(/^--(.+)/)[1];
      const next = args[i + 1];
      if (/^--.+/.test(next)) {
        argsAndValues[key] = false;
        continue;
      }
      argsAndValues[key] = next;
      i++;
    }
  }
  return argsAndValues;
};

const errorMsg = (message) => { 
  console.log( message ); 
};

const errorMessage = (type , message) => {
  console.log(`\x1b[31m \x1b[1m-------------------------${type} FAILED -------------------------- \x1b[0m` );
  console.log(`\x1b[33m ${message} \x1b[0m`  );
};
  
const printMessage = (message) => { 
  //console.log(`\x1b[32m\x1b[1m${message}\x1b[0m`);
  console.log(message);
};

 
const convertToKebabCase = str =>
  str &&
  str
    .match(/[A-Z]{2,}(?=[A-Z][a-z]+[0-9]*|\b)|[A-Z]?[a-z]+[0-9]*|[A-Z]|[0-9]+/g)
    .map(x => x.toLowerCase())
    .join('-'); 

const convertToCamelCase = str =>
  str &&
  str
    .match(/[a-zA-Z0-9.]+/g)
    .join(''); 

const tenant_enum = {
  TENANT_NAME: "Name of your product / tenant",
  TENANT_TYPE: "Type of Tenant",
  TENANT_MEMBERS: "Team Members",
  BUG_REPORTING: "Bug Reporting",
  FEEDBACK_REPORTING: "Feedback Reporting",
  ENHANCEMENT_REPORTING:" Enhancement Reporting",
  EXTERNAL_LINK: "External link",
  INTERNAL_TENANT: "Internal tenant",
  MERCHANT_PRODUCT: "Payments category",
  FI_PRODUCT: "Banking category",
  FINTECH_PRODUCT: "Fintech solution area",
  CARAT_PRODUCT: "Product of Carat",
  MERCHANT_AREA: "Merchants solution area",
  CUSTOMER_SEGMENTS_FOR_MERCHANTS: "Customer Segments for Merchants",
  CAPABILITIES_FOR_MERCHANTS: "Capabilities for Payments",
  FINANCIAL_INSTITUTIONS_SOLUTION_AREA: "Financial Institutions solution area",
  CUSTOMER_SEGMENTS_FOR_FINANCIAL_INSTITUTIONS: "Customer Segments for Financial Institutions",
  CAPABILITIES_FOR_FINANCIAL_INSTITUTIONS: "Capabilities for Banking",
  REGION_OF_OPERATION: "Region of Operation",
  CONTACT_SALES: "Sales Contact",
  CONTACT_SALES_PERSON: "Sales Contact Information",
  MOCK_SANDBOX: "Mock Sandbox",
  LIVE_SERVER: "Live Server"
}; 

const tenant_type_enum ={
  FULL_SERVICE: "Full service",
  DOC_ONLY: "Doc only",
  LINK_OUT: "Link out"
}

const regions ={
  NORTH_AMERICA: 'North America',
  EMEA: 'EMEA',
  LATAM: 'LATAM',
  APAC: 'APAC'
}

module.exports = {
  processArgs,
  errorMsg,
  errorMessage,
  printMessage,
  tenant_enum,
  tenant_type_enum,
  regions,
  convertToKebabCase,
  convertToCamelCase
};