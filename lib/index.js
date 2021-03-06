const generateData = require("./data");
const { performance } = require("perf_hooks");
const { addToSuite, runSuite } = require("./benchmark")();

const prettyPrint = (data, parseString = true) => {
  const t1 = performance.now();
  const result = parseString ? JSON.parse(data, true, 2) : data;
  const t2 = performance.now();
  console.log({
    parseString,
    time: `${parseFloat(t2 - t1).toFixed(5)}ms`,
    // result,
  });
};

const getData = (...opts) => {
  const rawData = generateData(...opts);
  const data = JSON.parse(JSON.stringify(rawData));
  const str = JSON.stringify(data);
  const buf = Buffer.from(str);
  return { rawData, data, str, buf };
};

const rust = require("./rust");
const javascript = require("./javascript");

console.time("init");
const opts = {
  static: false,
  dynamic: {
    loop: 1000,
    singleArray: true,
  },
};
const { data, str, buf } = getData(opts);
console.timeEnd("init");
console.log(opts);

// prettyPrint(rust.buffer_neon_value(buf), false);
// prettyPrint(rust.buffer_serde_string(buf), true);

console.log("--------------");

// unfair
// the data is already parsed and in the memory
addToSuite("js.unfair", () => javascript.unfair(data));

// JSON.Stringify
// make buffer from string
// js.buf
// return raw data (it's already parsed)
addToSuite("js.buf", () => javascript.buf(buf));

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
// make buffer from string
// rust.hashmap
addToSuite("rust.hashmap_buffer_serde_string_no_parse", () =>
  rust.hashmap_buffer_serde_string(buf)
);

// JSON.stringify
// make buffer from string
// rust.hashmap
// serde converts the stuff into string
// JSON.parse
addToSuite("rust.hashmap_buffer_serde_string", () =>
  JSON.parse(rust.hashmap_buffer_serde_string(buf))
);

// JSON.stringify
// make buffer from string
// rust.hashmap
// neon converts it to js value
addToSuite("rust.hashmap_buffer_neon_value", () =>
  rust.hashmap_buffer_neon_value(buf)
);

// JSON.stringify
// rust.objecttuple
// neon converts it to js value
// addToSuite("rust.objecttuple_string_neon_value", () =>
//   rust.objecttuple_string_neon_value(str)
// );

// JSON.stringify
// rust.objecttuple
// serde converts the stuff into string
// JSON.parse
// addToSuite("rust.objecttuple_string_serde_string", () =>
//   JSON.parse(rust.objecttuple_string_serde_string(str))
// );

console.log("--------------");

runSuite();
