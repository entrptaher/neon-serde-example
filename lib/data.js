function getSeedData(loop = 1000, staticData = false) {
  const inputArr = [
    // page
    [
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
    ],
  ];
}

exports = writeToFile = () =>
  require("fs").writeFileSync("sample.json", JSON.stringify(getSeedData()));

module.exports = getSeedData;
