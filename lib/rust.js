const data = require('./data');

var addon = require("../native");

addon.hello(JSON.stringify(data))

// const Benchmark = require("benchmark");
// var suite = new Benchmark.Suite();
// suite
//   .add("native.module", () => {
//     addon.hello(JSON.stringify(data))
//   })
  
//   // add listeners
//   .on("cycle", function (event) {
//     console.log(String(event.target));
//   })
//   .on("complete", function () {
//     console.log("Fastest is " + this.filter("fastest").map("name"));
//   })
//   // run async
//   .run({ async: true });
