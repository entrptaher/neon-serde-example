module.exports = () => {
  const Benchmark = require("benchmark");
  var suite = new Benchmark.Suite();

  const addToSuite = (name, fn) => {
    // just to know the initial value without iterating them a hundred/thousand times
    console.time(name);
    fn();
    console.timeEnd(name);
    suite.add(name, fn);
  };

  const runSuite = () => {
    // const results = [];
    suite
      .on("cycle", (event) => {
        const resultStr = String(event.target);
        console.log(resultStr, `x mean ${parseFloat(event.target.stats.mean * 1000).toFixed(4)}ms`);
        // const [name, , opsPerSec, , marginOfError] = resultStr.split(" ");
        // results.push({
        //   name,
        //   opsPerSec,
        //   marginOfError,
        //   mean: `${parseFloat(event.target.stats.mean * 1000).toFixed(4)}ms`,
        // });
      })
      .on("complete", function () {
        // console.table(results);
        console.log("Fastest is " + this.filter("fastest").map("name"));
      })
      .run({ async: true });
  };

  return { addToSuite, runSuite };
};
