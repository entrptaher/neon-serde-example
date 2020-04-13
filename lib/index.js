const rust = require("./rust");
const javascript = require("./javascript");
const data = require("./data")(2000000);

const prettyPrint = (data) => {
  console.log(JSON.parse(data, true, 2));
};

console.time('init');
const str = JSON.stringify(data);
const buf = Buffer.from(str);
console.timeEnd('init');

console.time('rust');
rust.direct(buf)
console.timeEnd('rust');

console.time('js');
javascript.json(str)
console.timeEnd('js');
