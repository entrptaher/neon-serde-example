const fetch = require("node-fetch");
const rust = require("./rust");
const javascript = require("./javascript");
const { addToSuite, runSuite } = require("./benchmark")();

function piper(buffer) {
  console.log(buffer);
}

async function run() {
  console.time("fetch");
  const res = await fetch("http://localhost:3000");
  console.timeEnd("fetch");

  console.time("res.text");
  const text = await res.text();
  console.timeEnd("res.text");

  await addToSuite("js.object", () => javascript.unfair(JSON.parse(text)));

  await addToSuite("js.string", () => JSON.parse(javascript.string(text)));

  await addToSuite("rust.objecttuple_string", () =>
    JSON.parse(rust.objecttuple_string_serde_string(text))
  );
  await addToSuite("rust.hashmap_buffer", () =>
    JSON.parse(rust.hashmap_buffer_serde_string(Buffer.from(text)))
  );

  await addToSuite("rust.hashmap_string", () =>
    JSON.parse(rust.hashmap_string_serde_string(text))
  );

  await addToSuite("rust.hashmap_string_no_parse", () =>
    rust.hashmap_string_serde_string(text)
  );

  await runSuite();

  process.exit(0);
}

run();
