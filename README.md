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
    2 648 596 ops/s, ±1.07%

  napi-nanoid:
    4 514 874 ops/s, ±0.30%

  js-nanoid (non-secure):
    1 928 811 ops/s, ±0.19%

  napi-nanoid (non-secure):
    4 650 390 ops/s, ±0.15%
```

`performance of all`

```rust
shortid                                23,347 ops/sec
cuid                                   99,297 ops/sec
secure-random-string                  194,417 ops/sec
uuid                                  889,546 ops/sec
js-nanoid (non-secure)              1,880,626 ops/sec
js-nanoid (secure)                  2,533,078 ops/sec
napi-nanoid (secure)                4,260,840 ops/sec
napi-nanoid (non-secure)            4,313,727 ops/sec
crypto.randomUUID                  10,290,907 ops/sec
hyperid                            14,344,743 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz, Node.js 16.15.0

[(runs: 6641636519)](https://github.com/rustq/napi-nanoid/runs/6641636519)

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
