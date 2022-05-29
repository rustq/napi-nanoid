# napi-nanoid


[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid) [![downloads](https://img.shields.io/npm/dm/napi-nanoid?color=purple)](https://www.npmjs.com/package/napi-nanoid)

`The NAPI nanoid written in Rust (with secure and better performance)`

`一款基于 Rust 实现的 NAPI nanoid (拥有安全性以及更好的性能)`

## Install

```shell
$ yarn add napi-nanoid
```

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
    2 584 156 ops/s, ±0.45%

  napi-nanoid:
    4 517 790 ops/s, ±0.21%

  js-nanoid (non-secure):
    1 852 950 ops/s, ±0.18%

  napi-nanoid (non-secure):
    4 626 292 ops/s, ±0.15%
```

`performance of all`

```rust
shortid                         22,280 ops/sec
cuid                            92,611 ops/sec
secure-random-string           185,539 ops/sec
uuid                           821,349 ops/sec
js-nanoid (non-secure)       1,795,358 ops/sec
js-nanoid (secure)           2,414,409 ops/sec
napi-nanoid (secure)         4,277,026 ops/sec
napi-nanoid (non-secure)     4,381,791 ops/sec
crypto.randomUUID           10,244,998 ops/sec
hyperid                     13,979,761 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz, Node.js 16.15.0

[(runs: 6637597027)](https://github.com/rustq/napi-nanoid/runs/6637597027)

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
