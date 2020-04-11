module.exports = (...list) => {
  const Benchmark = require("benchmark");
  var suite = new Benchmark.Suite();
  for (let bench of list) {
    suite.add(bench.name, bench);
  }
  suite
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

const rust = require("./rust");
const javascript = require("./javascript");
const benchmark = require("./benchmark");
const data = require("./data")();
const dataStr = JSON.stringify(data);
const dataBuf = Buffer.from(dataStr);

benchmark(
  rustString = () => rust.string(dataStr),
  rustBuffer = () => rust.buffer(dataBuf),
  javascriptJson = () => javascript.json(data)
);
