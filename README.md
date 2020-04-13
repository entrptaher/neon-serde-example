Created for reddit thread: [Deserialize an arbitrary json structure](https://www.reddit.com/r/rust/comments/fyevnx/deserialize_an_arbitrary_json_structure/)

## Performance

Benchmarked 1k items inside page1.group1, inside javascript context using benchrmark.js

```
➜  node lib/index.js 
init: 1.828ms
{ static: false, dynamic: { loop: 1000, singleArray: true } }
--------------
js.unfair 0.34795ms
js.buf 0.46733ms
js.string 2.02784ms
js.bufString 1.53930ms
rust.hashmap_buffer_serde_string_no_parse 0.53442ms
rust.hashmap_buffer_serde_string 0.85842ms
rust.hashmap_buffer_neon_value 1.74317ms
--------------
js.unfair x 91,452 ops/sec ±6.97% (85 runs sampled) x mean 0.0109ms
js.buf x 3,986 ops/sec ±1.52% (87 runs sampled) x mean 0.2509ms
js.string x 915 ops/sec ±0.96% (93 runs sampled) x mean 1.0929ms
js.bufString x 915 ops/sec ±0.67% (94 runs sampled) x mean 1.0924ms
rust.hashmap_buffer_serde_string_no_parse x 2,310 ops/sec ±0.86% (92 runs sampled) x mean 0.4328ms
rust.hashmap_buffer_serde_string x 1,304 ops/sec ±0.60% (96 runs sampled) x mean 0.7671ms
rust.hashmap_buffer_neon_value x 643 ops/sec ±0.47% (95 runs sampled) x mean 1.5558ms
Fastest is js.unfair
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
    "page": "page1",
    "group": "group1",
    "name": "name1",
    "index": 10
  },
  // ... other data
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
