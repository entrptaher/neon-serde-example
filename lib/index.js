const rust = require("./rust");
const javascript = require("./javascript");

const prettyPrint = (data) => console.log(JSON.parse(data, true, 2));

console.time("init");
const data = require("./data")(1000);
const str = JSON.stringify(data);
const buf = Buffer.from(str);
console.timeEnd("init");

// console.time('fib')
// rust.par_array(46, (err, result) => {
//   console.log('async result:');
//   console.log(result);
//   console.timeEnd('fib');
// });

const Benchmark = require("benchmark");
var suite = new Benchmark.Suite();

const addToSuite = (name, fn) => {
  // just to know the initial value without iterating them a hundred/thousand times
  console.time(name);
  fn();
  console.timeEnd(name);
  suite.add(name, fn);
};

console.log('--------------');

// native
addToSuite("js.json", () => javascript.json(data));

// JSON.stringify+js.string+JSON.parse
addToSuite("js.string", () => JSON.parse(javascript.string(str)));

// js.bufNative
addToSuite("js.bufNative", () => javascript.bufNative(buf));

// JSON.stringify+js.bufString+JSON.parse
addToSuite("js.bufString", () => JSON.parse(javascript.bufString(buf)));

// JSON.stringify+rust.buf+JSON.parse
addToSuite("rust.buf", () => JSON.parse(rust.direct(buf)));

console.log('--------------');
suite
  .on("cycle", (event) => console.log(String(event.target)))
  .on("complete", function () {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  .run({ async: true });
