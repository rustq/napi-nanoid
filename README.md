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
const { nanoid } = require('napi-nanoid')

nanoid() // => AeogKAGjUMX6mqB4sMzWe
```

## Performance

`nanoid compare`

```rust
  js-nanoid:
    3 318 612 ops/s, ±1.10%

  napi-nanoid:
    5 141 154 ops/s, ±0.84%

  js-nanoid (non-secure):
    2 005 501 ops/s, ±0.15%

  napi-nanoid (non-secure):
    5 359 638 ops/s, ±0.13%
```

`performance of all`

```rust
shortid                      23,900 ops/sec
cuid                        102,473 ops/sec
secure-random-string        194,803 ops/sec
uuid                        839,287 ops/sec
js-nanoid (non-secure)    1,944,889 ops/sec
js-nanoid (secure)        3,323,263 ops/sec
napi-nanoid (secure)      4,862,067 ops/sec
napi-nanoid (non-secure)  5,007,932 ops/sec
crypto.randomUUID        10,658,212 ops/sec
hyperid                  14,534,342 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) Platinum 8370C CPU @ 2.80GHz, Node.js 16.20.0

[(runs: 4905855849)](https://github.com/rustq/napi-nanoid/actions/runs/4905855849/jobs/8759930864)

## Support matrix

|                  | node14 | node16 | node18 |
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
