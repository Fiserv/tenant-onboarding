#!/usr/bin/env node

const fs = require('fs'); 
const args = process.argv.slice(2); 
const {errorMessage  , printMessage} = require('./tools')
const md = require('markdown-it')();
const html2json = require('html2json').html2json;
const axios = require('axios');
const { rejects } = require('assert');

const tenantConfigurator =  async (issueNo ) => {

try{

const url = `https://api.github.com/repos/Fiserv/Support/issues/${issueNo}`;
printMessage("GITHUB URL : " + url);
const config = {
    headers:{
        "User-Agent": "tenant-onbaording",
        "Accept": "application/vnd.github+json",
        "Authorization" : `Bearer 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`
        }
        };                               

    axios({
        method:'GET',
        url,
        config
    })
    .then(function (response) {

      const tenantConfig =  response?.data?.body;
      const md_result = md.render(tenantConfig);
      const result_data = html2json(md_result);  

      for (const obj of result_data.child){
            
            if (obj?.node === 'element' ){
            // Parsing title
            if (obj?.tag === 'h3'){ 
              console.log( JSON.stringify(obj?.child[0].text));
            }
            else if (obj?.tag === 'p'){
              console.log(JSON.stringify(obj?.child[0].text));
            }
            else if (obj?.tag === 'ul'){ 
              for (const chd of obj?.child){ 
                if (chd?.node === 'element'){
                  const val = JSON.stringify(chd?.child[0]?.text.trim());
                  if ( val.includes('[X]')){ 
                    const len = val.length;
                    console.log("Value" , val.slice(4 ,len-1 ));
                  }                  
                }
              } 
            } 
          }
        }
    })
    .catch(function (error) {
        errorMessage(error);
    });
}catch( e){
    errorMessage('Unable to call' ,e.message);
}
};

const configGenerator = async () => {
  const fsPromises = fs.promises; 
          return new Promise((resolve , rejects) => {
            fsPromises.readFile('tenant.json', 'utf8') 
            .then(data => { 
                    let json = JSON.parse(data); 
                    console.log(JSON.stringify(json , null , 2));
    
                    fsPromises.writeFile('tenant.json', JSON.stringify(json , null , 2))
                            .then(  () => { resolve('Updated Success'); })
                            .catch(err => { rejects("Update Failed: " + err);});
                })
            .catch(err => { console.log("Read Error: " +err);});
 
          });
};

 

try {
  console.log(`Gihub Issue No. ---->>> ${args}`);   
 if ( args?.length > 0){ 
   // tenantConfigurator(args);
     const check  = configGenerator();
      
 }else{  
  errorMessage('MD VALIDATOR' ,'No Path for docs dir. defined');
 }
} catch (e) {
  errorMessage('MD VALIDATOR' ,e.message);
}
