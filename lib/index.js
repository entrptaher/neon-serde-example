const rust = require("./rust");
const javascript = require("./javascript");
const data = require("./data")(1000);

const prettyPrint = (data) => {
  console.log(JSON.stringify(data, true, 2));
};

console.time("init");
const str = JSON.stringify(data);
const buf = Buffer.from(str);
console.timeEnd("init");

console.time("direct");
rust.direct(buf)
console.timeEnd("direct");

console.time("js");
javascript.json(str);
console.timeEnd("js");

console.time("string");
rust.string(str);
console.timeEnd("string");

console.time("buffer");
rust.buffer(buf);
console.timeEnd("buffer");



// const Benchmark = require("benchmark");
// var suite = new Benchmark.Suite();

// suite
//   .add("js", () => javascript.json(str))
//   .add("direct", () => JSON.parse(rust.direct(buf)))
//   // add listeners
//   .on("cycle", function (event) {
//     console.log(String(event.target));
//   })
//   .on("complete", function () {
//     console.log("Fastest is " + this.filter("fastest").map("name"));
//   })
//   // run async
//   .run({ async: true });
