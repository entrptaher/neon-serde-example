function getSeedData({
  static = false,
  dynamic = {
    loop: 1000,
    singleArray: false,
  },
} = {}) {
  if (static) {
    const input = {
      page1: {
        group1: [
          { name: "name1", index: 10 },
          { name: "name3", index: 30 },
          { name: "name5", index: 50 },
        ],
      },
      page2: {
        group2: [
          { name: "name2", index: 20 },
          { name: "name4", index: 40 },
          { name: "name6", index: 60 },
        ],
      },
    };
    return input;
  }

  const dynamicInput = {};

  for (let i = 1; i <= dynamic.loop; i++) {
    let pageIndex = dynamic.singleArray ? 1 : i;
    let groupIndex = dynamic.singleArray ? 1 : i;
    if (!dynamicInput[`page${pageIndex}`])
      dynamicInput[`page${pageIndex}`] = {};
    if (!dynamicInput[`page${pageIndex}`][`group${groupIndex}`])
      dynamicInput[`page${pageIndex}`][`group${groupIndex}`] = [];

    dynamicInput[`page${pageIndex}`][`group${groupIndex}`].push({
      name: "name" + i,
      index: parseInt(i),
    });
  }

  return dynamicInput;
}

exports = writeToFile = () =>
  require("fs").writeFileSync("sample.json", JSON.stringify(getSeedData()));

module.exports = getSeedData;
