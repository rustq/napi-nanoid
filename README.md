# napi-nanoid


[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid) [![downloads](https://img.shields.io/npm/dm/napi-nanoid?color=purple)](https://www.npmjs.com/package/napi-nanoid)

`The NAPI nanoid written in Rust`

`一款基于 Rust 实现的 NAPI nanoid`

## Install

```shell
$ npm i napi-nanoid
```

## Usage

```js
const { nanoid } = require('napi-nanoid');

nanoid() // => AeogKAGjUMX6mqB4sMzWe
```

## Performance

`nanoid compare`

```rust
  js-nanoid:
    3 392 881 ops/s, ±0.55%

  napi-nanoid:
    5 113 763 ops/s, ±0.12%

  js-nanoid (non-secure):
    1 875 245 ops/s, ±0.14%

  napi-nanoid (non-secure):
    5 237 554 ops/s, ±0.11% 
```

`performance of all`

```rust
shortid                      24,084 ops/sec
cuid                        105,736 ops/sec
secure-random-string        207,409 ops/sec
uuid                        840,460 ops/sec
js-nanoid (non-secure)    1,826,354 ops/sec
js-nanoid (secure)        3,171,036 ops/sec
napi-nanoid (secure)      4,837,387 ops/sec
napi-nanoid (non-secure)  4,977,971 ops/sec
crypto.randomUUID        12,152,367 ops/sec
hyperid                  16,554,640 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz, Node.js 16.15.1

[(runs: 6842925183)](https://github.com/rustq/napi-nanoid/runs/6842925183)

## Support matrix

|                  | node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## Development

```shell
$ git clone https://github.com/rustq/napi-nanoid

$ cd napi-nanoid

$ yarn
```

```shell
$ yarn build

$ yarn test
```

```shell
$ yarn bench
```

## License

[MIT](https://opensource.org/licenses/MIT)
