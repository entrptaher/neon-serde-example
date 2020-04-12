const rust = require("./rust");
const javascript = require("./javascript");
const data = require("./data")(1000);

const prettyPrint = (data) => {
  console.log(JSON.parse(data, true, 2));
};
const str = JSON.stringify(data);
const buf = Buffer.from(str);

console.time('rust');
rust.direct(buf)
console.timeEnd('rust');