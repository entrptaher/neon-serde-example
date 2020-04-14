const fetch = require("node-fetch");
const { performance } = require("perf_hooks");
const rust = require("./rust");
const javascript = require("./javascript");
const { addToSuite, runSuite } = require("./benchmark")();

function piper(buffer) {
  console.log(buffer);
}

const copy = (data) => data; //JSON.parse(JSON.stringify(data));

const http = require("http");
const bl = require("bl");

async function httpGet(url) {
  return new Promise((resolve, reject) => {
    http.get(url, (response) => {
      response.setEncoding("utf8");
      response.pipe(
        bl((err, data) => {
          if (err) {
            reject(err);
          }
          resolve(data);
          // resolve(data.toString());
        })
      );
    });
  });
}

async function run() {
  const t1 = performance.now();
  const buf = await httpGet("http://localhost:3000");
  const text = await buf.toString();
  const t2 = performance.now();
  console.log("fetch", `${parseFloat((t2 - t1) / 1000).toFixed(5)}s`);
  
  console.log('------------------------');
  await addToSuite("js.object", () =>
    javascript.object(JSON.parse(copy(text)))
  );

  await addToSuite("js.string", () =>
    JSON.parse(javascript.string(copy(text)))
  );

  await addToSuite("rust.objecttuple_string", () =>
    JSON.parse(rust.objecttuple_string_serde_string(copy(text)))
  );

  await addToSuite("rust.hashmap_buffer", () =>
    JSON.parse(rust.hashmap_buffer_serde_string(buf))
  );

  await addToSuite("rust.hashmap_string", () =>
    JSON.parse(rust.hashmap_string_serde_string(copy(text)))
  );

  await addToSuite("rust.hashmap_string_no_parse", () =>
    rust.hashmap_string_serde_string(copy(text))
  );
  console.log('------------------------');
  await runSuite();

  process.exit(0);
}

run();
