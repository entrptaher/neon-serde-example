var addon = require("../native");
const data = require('./data');

const iterJs = (inputRef) => {
  let result = [];
  const input = JSON.parse(JSON.stringify(inputRef));
  Object.entries(input).map(([page_key, page]) => {
    Object.entries(page).map(([group_key, group]) => {
      group.map((stuff) => {
        result.push({ page_key, group_key, stuff });
      });
    });
  });
  return result;
};

const iterRust = (input) => {
  return addon.hello(JSON.stringify(input)) //JSON.parse(addon.hello(JSON.stringify(input)));
};

console.time('init');
console.timeEnd('init');

console.time("iterJS");
iterJs(data);
console.timeEnd("iterJS");

console.time("iterRust");
iterRust(data);
console.timeEnd("iterRust");

const Benchmark = require("benchmark");
var suite = new Benchmark.Suite();
suite
  .add("native.module", () => {
    iterRust(data);
  })
  .add("javascript", () => {
    iterJs(data);
  })
  // add listeners
  .on("cycle", function (event) {
    console.log(String(event.target));
  })
  .on("complete", function () {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  // run async
  .run({ async: true });
