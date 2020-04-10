function getSeedData(loop = 1000, staticData = false) {
  const input = {
    page1: {
      group1: [
        { name: "name1", index: 10 },
        { name: "name3", index: 30 },
        { name: "name5", index: 50 },
      ],
    },
    page500: {
      group500: [
        { name: "name2", index: 20 },
        { name: "name4", index: 40 },
        { name: "name6", index: 60 },
      ],
    },
  };

  if (staticData) return input;

  for (let i = 1; i <= loop; i++) {
    input.page1.group1.push({
      name: "name" + i,
      index: parseInt(i),
    });
  }

  return input;
}

exports = writeToFile = () =>
  require("fs").writeFileSync("sample.json", JSON.stringify(getSeedData()));

module.exports = getSeedData;
