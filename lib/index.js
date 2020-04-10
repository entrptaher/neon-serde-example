const rust = require('./rust');
const javascript = require("./javascript");
const data = require("./data")();

const prettyPrint = (data) => {
  console.log(JSON.stringify(data, true, 2));
};

prettyPrint(rust.hello(data));
javascript.hello(data);
