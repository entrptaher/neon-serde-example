Created for reddit thread: [Deserialize an arbitrary json structure](https://www.reddit.com/r/rust/comments/fyevnx/deserialize_an_arbitrary_json_structure/)

# Original implementation

Grabs data from javascript, converts to serde_json::Value using neon_serde::from_value, iterate them using multiple loops, and convert the data back using neon_serde::to_value to a proper JsValue.

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

## Performance

Benchmarked 1k items inside page1.group1, inside javascript context using benchrmark.js

```
➜  node lib/benchmark.js
native.module x 171 ops/sec ±0.37% (86 runs sampled)
javascript x 2,337 ops/sec ±1.10% (94 runs sampled)
Fastest is javascript
```