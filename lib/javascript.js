module.exports = {
  hello: (input) => {
    let result = [];
    const output = JSON.parse(JSON.stringify(input));
    Object.entries(output).map(([pagekey, page]) => {
      Object.entries(page).map(([groupKey, group]) => {
        group.map((stuff) => {
          result.push({ pagekey, groupKey, stuff });
        });
      });
    });
    return result;
  },
};
