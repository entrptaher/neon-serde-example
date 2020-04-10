const object = {
  page1: {
    group1: [{ name: "name1", index: 1 }],
  },
  page505: {
    group505: [{ name: "name1", index: 500 }],
  },
};

function getSeedData() {
  const output = {
    page1: {
      group1: [{ name: "name1", index: 1 }],
    },
    page505: {
      group505: [{ name: "name1", index: 500 }],
    },
  };

  for (let i = 1; i <= 1000000; i++) {
    output.page1.group1.push({ name: "name1", index: parseInt(i) });
  }

  return output;
}

require("fs").writeFileSync(
  "sample.json",
  JSON.stringify(getSeedData(), true, 2)
);
// console.log(JSON.stringify(getSeedData()));
// module.exports = getSeedData();
