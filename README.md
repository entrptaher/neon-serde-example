Created for reddit thread: [Deserialize an arbitrary json structure](https://www.reddit.com/r/rust/comments/fyevnx/deserialize_an_arbitrary_json_structure/)

## Performance

Benchmarked 1k items inside page1.group1, inside javascript context using benchrmark.js

```
➜  node lib/index.js 
init: 1.703ms
{ static: false, dynamic: { loop: 1000, singleArray: true } }
```

The following assumes I am running all functions for first time, for just once.
```
js.unfair 0.31148ms
js.buf 0.43268ms
js.string 1.36488ms
js.bufString 1.69201ms
rust.hashmap_buffer_serde_string_no_parse 0.46569ms
rust.hashmap_buffer_serde_string 0.82932ms
rust.hashmap_buffer_neon_value 1.76512ms
```

Then the results from benchmark.js appears,
```
js.unfair x 98,727 ops/sec ±5.94% (87 runs sampled) x mean 0.0101ms
js.buf x 4,036 ops/sec ±1.04% (92 runs sampled) x mean 0.2478ms
js.string x 913 ops/sec ±0.64% (96 runs sampled) x mean 1.0949ms
js.bufString x 917 ops/sec ±0.46% (95 runs sampled) x mean 1.0910ms
rust.hashmap_buffer_serde_string_no_parse x 2,371 ops/sec ±0.34% (98 runs sampled) x mean 0.4217ms
rust.hashmap_buffer_serde_string x 1,311 ops/sec ±0.66% (95 runs sampled) x mean 0.7630ms
rust.hashmap_buffer_neon_value x 626 ops/sec ±0.36% (95 runs sampled) x mean 1.5967ms
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
