module.exports = (native, javascript) => {
  const Benchmark = require("benchmark");
  var suite = new Benchmark.Suite();
  suite
    .add("native.module", native)
    .add("javascript", javascript)
    // add listeners
    .on("cycle", function (event) {
      console.log(String(event.target));
    })
    .on("complete", function () {
      console.log("Fastest is " + this.filter("fastest").map("name"));
    })
    // run async
    .run({ async: true });
};

const rust = require("../native/index.node");
const javascript = require("./javascript");
const benchmark = require("./benchmark");
const data = require("./data")();

benchmark(
  () => rust.hello(data),
  () => javascript.hello(data)
);