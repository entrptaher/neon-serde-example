Created for reddit thread: [Deserialize an arbitrary json structure](https://www.reddit.com/r/rust/comments/fyevnx/deserialize_an_arbitrary_json_structure/)

# HashMap + String implementation

- Grabs stringified json data from javascript, 
- converts to serde_json::Value using serde_json::from_str, 
- iterate them using multiple loops, 
- convert the data back using serde_json::to_string to a proper String, 
- finally use JSON.parse.

## Performance

Benchmarked 1k items inside page1.group1, inside javascript context using benchrmark.js

```
➜  node lib/benchmark.js
native.module x 1,078 ops/sec ±0.18% (94 runs sampled)
javascript x 2,426 ops/sec ±1.01% (91 runs sampled)
Fastest is javascript
```

## Development
```
yarn
yarn run build
node lib/index.js
```

## Input

```json
{
  "page1": {
    "group1": [
      {
        "name": "name1",
        "index": 10
      },
      {
        "name": "name3",
        "index": 30
      },
      {
        "name": "name5",
        "index": 50
      }
    ]
  }
}
```

## Output

```json
[
  {
    "page_key": "page1",
    "group_key": "group1",
    "stuff": {
      "index": 10,
      "name": "name1"
    }
  },
  {
    "page_key": "page1",
    "group_key": "group1",
    "stuff": {
      "index": 30,
      "name": "name3"
    }
  },
  {
    "page_key": "page1",
    "group_key": "group1",
    "stuff": {
      "index": 50,
      "name": "name5"
    }
  }
]
```