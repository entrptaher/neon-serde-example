const { performance } = require("perf_hooks");
const Benchmark = require("benchmark");

module.exports = () => {
  var suite = new Benchmark.Suite();

  const addToSuite = (name, fn) => {
    // just to know the initial value assuming we don't have any optimizations yet
    // without iterating them a hundred/thousand times
    const t1 = performance.now();
    fn();
    const t2 = performance.now();
    console.log(name, `${parseFloat((t2 - t1)/1000).toFixed(5)}s`);
    suite.add(name, fn);
  };

  const runSuite = () => {
    return new Promise(resolve=>{
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
        resolve();
      })
      .run({ async: true });
    })
  };

  return { addToSuite, runSuite };
};
