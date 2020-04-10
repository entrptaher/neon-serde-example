const rust = require("../native/index.node");
const javascript = require("./javascript");
const data = require("./data")();

const prettyPrint = (data) => {
  console.log(JSON.stringify(data, true, 2));
};

rust.hello(data);
javascript.hello(data)
