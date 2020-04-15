const fetch = require("node-fetch");
const { PerformanceObserver, performance } = require("perf_hooks");
const javascript = require("./javascript");
const { addToSuite, runSuite } = require("./benchmark")();
const data = require("../server/data")({ dynamic: { loop: 1000000} });
const worker = require("worker_threads");

const obs = new PerformanceObserver((items) => {
  console.log(items.getEntries());
  // performance.clearMarks();
});
obs.observe({ entryTypes: ["measure"] });

const {
  Worker,
  isMainThread,
  parentPort,
  workerData,
} = require("worker_threads");

if (isMainThread) {
  function parseJSAsync() {
    const worker = new Worker(__filename, {
      workerData: data,
    });
    worker.on("message", (message) => {
      if (message.data) {
        JSON.parse(message.data);
      }
    });
    worker.on("error", console.error);
    worker.on("exit", (code) => {
      if (code !== 0)
        console.log(new Error(`Worker stopped with exit code ${code}`));
    });
  }
  parseJSAsync();
} else {
  const rust = require("./rust");
  console.time('stuff');
  const data = rust.perform_sync_task(JSON.stringify(workerData));
  console.timeEnd('stuff');
  // rust.perform_async_task(JSON.stringify(workerData), (err, result) => {
  //   parentPort.postMessage({ data: result });
  // })
}

// performance.mark("sync1.1");
// rust.perform_sync_task(JSON.stringify(data));
// performance.mark("sync1.2");
// performance.measure("A to B", "sync1.1", "sync1.2");

// for (let i = 0; i < 4; i++) {
//   let startMark = "async-" + i + "-1";
//   let endMark = "async-" + i + "-2";
//   performance.mark(startMark);
//   rust.perform_async_task(JSON.stringify(data), (err, result) => {
//     // console.log(result);
//     performance.mark(endMark);
//     performance.measure(startMark + "-" + endMark, startMark, endMark);
//   });
// }

// performance.mark("async1.1");
// rust.perform_async_task(JSON.stringify(data), (err, result) => {
//   // console.log(result);
//   performance.mark("async1.2");
//   performance.measure('A to B 2', 'async1.1', 'async1.2');
// });

// performance.mark("async2.1");
// rust.perform_async_task(JSON.stringify(data), (err, result) => {
//   // console.log(result);
//   performance.mark("async2.2");
//   performance.measure('A to B 2', 'async2.1', 'async2.2');
// });

// const measure = performance.getEntriesByName('A to B')[0];
// console.log(measure.duration);
// function piper(buffer) {
//   console.log(buffer);
// }

// const copy = (data) => data; //JSON.parse(JSON.stringify(data));

// const http = require("http");
// const bl = require("bl");

// async function httpGet(url) {
//   return new Promise((resolve, reject) => {
//     http.get(url, (response) => {
//       response.setEncoding("utf8");
//       response.pipe(
//         bl((err, data) => {
//           if (err) {
//             reject(err);
//           }
//           resolve(data);
//           // resolve(data.toString());
//         })
//       );
//     });
//   });
// }

// async function run() {
//   const t1 = performance.now();
//   const buf = await httpGet("http://localhost:3000");
//   const text = await buf.toString();
//   const t2 = performance.now();
//   console.log("fetch", `${parseFloat((t2 - t1) / 1000).toFixed(5)}s`);

//   console.log('------------------------');
//   await addToSuite("js.object", () =>
//     javascript.object(JSON.parse(copy(text)))
//   );

//   await addToSuite("js.string", () =>
//     JSON.parse(javascript.string(copy(text)))
//   );

//   await addToSuite("rust.objecttuple_string", () =>
//     JSON.parse(rust.objecttuple_string_serde_string(copy(text)))
//   );

//   await addToSuite("rust.hashmap_buffer", () =>
//     JSON.parse(rust.hashmap_buffer_serde_string(buf))
//   );

//   await addToSuite("rust.hashmap_string", () =>
//     JSON.parse(rust.hashmap_string_serde_string(copy(text)))
//   );

//   await addToSuite("rust.hashmap_string_no_parse", () =>
//     rust.hashmap_string_serde_string(copy(text))
//   );
//   console.log('------------------------');
//   await runSuite();

//   process.exit(0);
// }

// run();
