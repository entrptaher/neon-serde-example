// const rust = require('./rust');
// const javascript = require("./javascript");
// const data = require("./data")(100);

// const prettyPrint = (data) => {
//   console.log(JSON.stringify(data, true, 2));
// };

// const payload = JSON.stringify(data);

// console.time('rust');
// rust.hello(payload)
// console.timeEnd('rust');

// console.time('js');
// javascript.hello(payload);
// console.timeEnd('js');

const inputArr = [
  {
    identifier: "page1",
    data: [
      {
        identifier: "group1",
        data: [
          { name: "name1", index: 10 },
          { name: "name2", index: 20 },
          { name: "name3", index: 30 },
          { name: "name4", index: 40 },
        ],
      },
    ],
  },
  {
    identifier: "page2",
    data: [
      {
        identifier: "group2",
        data: [
          { name: "name1", index: 10 },
          { name: "name2", index: 20 },
          { name: "name3", index: 30 },
          { name: "name4", index: 40 },
        ],
      },
    ],
  },
];

const js = (dataStr) => {
  const input = JSON.parse(dataStr);
  let result = [];
  for (let page of input) {
    for (let group of page.data) {
      for (let stuff of group.data) {
        result.push({
          page_key: page.identifier,
          group_key: group.identifier,
          stuff,
        });
      }
    }
  }
  return result;
};

const rust = (data) => JSON.parse(addon.string(data));

const addon = require("../native/index.node");
const payload = JSON.stringify(inputArr);
console.log(js(payload));
console.log(rust(payload));

console.time("js");
js(payload);
console.timeEnd("js");

console.time('rust');
rust(payload);
console.timeEnd("rust");


// const Benchmark = require("benchmark");
//   var suite = new Benchmark.Suite();
//   suite
//     .add("native.module", ()=>rust(payload))
//     .add("javascript", ()=>js(payload))
//     // add listeners
//     .on("cycle", function (event) {
//       console.log(String(event.target));
//     })
//     .on("complete", function () {
//       console.log("Fastest is " + this.filter("fastest").map("name"));
//     })
//     // run async
//     .run({ async: true });