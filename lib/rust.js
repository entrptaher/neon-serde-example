const addon = require("../native/index.node");

const rust = {
  hello: (data) => JSON.parse(addon.hello(JSON.stringify(data)))
};

module.exports = rust;