const object = {
  page1: {
    group1: [
      { name: "name1", index: 1 },
    ],
  },
  page500: {
    group500: [
      { name: "name1", index: 50 },
    ],
  },
};

function getSeedData() {
  /**
   * SEED DATA
   */
  const output = {
    page1: {
      group1: [],
    },
  };

  // 1M data
  for (let i = 1; i <= 1000000; i++) {
    output.page1.group1.push({
      name: "example " + i,
      index: i,
    });
  }

  return output;
}

require('fs').writeFileSync('sample.json', JSON.stringify(getSeedData(), true, 2))
// module.exports = getSeedData();