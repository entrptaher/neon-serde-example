// const data = require('./data');
const data = require('../native/src/sample.json')

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

iterJs(data);