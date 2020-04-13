const generateData = require("./data");
const { addToSuite, runSuite } = require("./benchmark")();
const prettyPrint = (data) => console.log(JSON.parse(data, true, 2));
const getData = () => {
  const data = JSON.parse(JSON.stringify(generateData(1000)));
  const str = JSON.stringify(data);
  const buf = Buffer.from(str);
  return { data, str, buf };
};

const rust = require("./rust");
const javascript = require("./javascript");

console.time("init");
const { data, str, buf } = getData();
console.timeEnd("init");

console.log("--------------");

// unfair
// the data is already parsed and in the memory
addToSuite("js.unfair", () => javascript.unfair(data));

// JSON.Stringify
// make buffer from string
// js.bufUnfair
// return raw data (which means it's already parsed)
addToSuite("js.bufUnfair", () => javascript.bufUnfair(buf));

// JSON.stringify
// js.string
// JSON.parse
addToSuite("js.string", () => JSON.parse(javascript.string(str)));

// JSON.stringify
// make buffer from string
// js.bufString
// JSON.parse
addToSuite("js.bufString", () => JSON.parse(javascript.bufString(buf)));

// JSON.stringify
// rust.buf
// JSON.parse
addToSuite("rust.buf", () => JSON.parse(rust.direct(buf)));

console.log("--------------");

runSuite();
