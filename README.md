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
    3 312 758 ops/s, ±0.40%

  napi-nanoid:
    5 413 480 ops/s, ±0.21%

  js-nanoid (non-secure):
    1 846 206 ops/s, ±0.26%

  napi-nanoid (non-secure):
    5 648 410 ops/s, ±0.09%
```

`performance of all`

```rust
shortid                      23,659 ops/sec
cuid                        100,672 ops/sec
secure-random-string        194,024 ops/sec
uuid                        837,116 ops/sec
js-nanoid (non-secure)    1,928,304 ops/sec
js-nanoid (secure)        3,286,650 ops/sec
napi-nanoid (secure)      5,125,939 ops/sec
napi-nanoid (non-secure)  5,290,560 ops/sec
crypto.randomUUID        10,601,453 ops/sec
hyperid                  14,736,232 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) CPU E5-2673 v3 @ 2.40GHz, Node.js 16.19.0

[(runs: 4093065024)](https://github.com/rustq/napi-nanoid/actions/runs/4093065024/jobs/7058254207)

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
