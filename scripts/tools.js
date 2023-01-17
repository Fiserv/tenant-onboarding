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
  console.log(`\x1b[32m \x1b[1m ${message} \x1b[0m`  );
}

module.exports = {
  processArgs,
  errorMsg,
  errorMessage,
  printMessage,
};