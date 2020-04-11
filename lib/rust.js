const addon = require("../native/index.node");

const rust = {
  hello: (data) => addon.hello(data)
};

module.exports = rust;