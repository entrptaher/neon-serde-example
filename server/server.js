const express = require("express");
const app = express();
const port = 3000;

const generateData = require("./data");

app.get("/", (req, res) => {
  res.send(
    generateData({
      static: false,
      dynamic:{
        loop: 1000000,
        singleArray: false
      }
    })
  );
});

app.listen(port, () =>
  console.log(`Example app listening at http://localhost:${port}`)
);
