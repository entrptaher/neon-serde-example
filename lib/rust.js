const addon = require("../native/index.node");

const rust = {
  hello: (data) => addon.hello(JSON.stringify(data))
};

module.exports = rust;