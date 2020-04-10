const data = require('./data');
// const data = require('../native/src/sample.json')

const iterJs = (inputRef) => {
  let result = [];
  const input = JSON.parse(JSON.stringify(inputRef));
  Object.entries(input).map(([page, pageObj]) => {
    Object.entries(pageObj).map(([group, groupObj]) => {
      groupObj.map((stuff) => {
        result.push({ page, group, stuff });
      });
    });
  });
  return result;
};

console.log(iterJs(data));