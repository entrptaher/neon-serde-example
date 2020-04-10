Created for reddit thread: [Deserialize an arbitrary json structure](https://www.reddit.com/r/rust/comments/fyevnx/deserialize_an_arbitrary_json_structure/)

# kflansburg: ObjectTuple implementation

- Grabs stringified json data from javascript,
- converts to serde_json::Value using serde_json::from_str
- iterate them using multiple loops,
- convert the data back using neon_serde::to_vazlue to a proper JsValue,

## Performance

Benchmarked 1k items inside page1.group1, inside javascript context using benchrmark.js

```
➜  node lib/benchmark.js 
native.module x 548 ops/sec ±0.80% (90 runs sampled)
javascript x 2,414 ops/sec ±0.89% (89 runs sampled)
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

The output of this version is a bit different,

```json
[
  {
    "page": 500,
    "group": 500,
    "stuff": {
      "name": 6,
      "index": 60
    }
  }
]
```

The original target is the following,
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
