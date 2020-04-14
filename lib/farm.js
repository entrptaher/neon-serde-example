module.exports = (input, callback) => {
  const { pagekey, page } = input;
  const result = Object.entries(page).map(([groupKey, group]) =>
    group.map((stuff) => ({ pagekey, groupKey, stuff }))
  );
  callback(null, result);
};
