const js = {
  unfair: (inputObj) => {
    let result = [];
    Object.entries(inputObj).map(([pagekey, page]) => {
      Object.entries(page).map(([groupKey, group]) => {
        group.map((stuff) => {
          result.push({ pagekey, groupKey, stuff });
        });
      });
    });
    return result;
  },
  string: (inputStr) => {
    const inputObj = JSON.parse(inputStr);
    const result = js.unfair(inputObj);
    return JSON.stringify(result);
  },
  bufUnfair: (inputBuf) => {
    const inputObj = JSON.parse(inputBuf.toString("utf8"))
    const result = js.unfair(inputObj);
    return result;
  },
  bufString: (inputBuf) => {
    const inputStr = inputBuf.toString("utf8");
    return js.string(inputStr);
  },
};

module.exports = js;
