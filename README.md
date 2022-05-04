# napi-nanoid


[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid)

`The NAPI nanoid written in Rust`

`一款基于 Rust 实现的 NAPI nanoid `

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

`Benchmark: Linux-x64-gnu | Intel(R) Xeon(R) CPU E5-2673 v4 @ 2.30GHz | 2 Cores | 1 G | node 16`

```rust
Running "napi-nanoid" suite...

  napi-nanoid:
    4 066 259 ops/s, ±0.49%   | fastest

  js-nanoid:
    2 284 720 ops/s, ±0.68%   | 43.81% slower

  uuid:
    662 363 ops/s, ±0.91%     | slowest, 83.71% slower

Finished 3 cases!
  Fastest: napi-nanoid
  Slowest: uuid
Done in 17.41s.
```

[(runs: 6287759337)](https://github.com/rustq/napi-nanoid/runs/6287759337)

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
