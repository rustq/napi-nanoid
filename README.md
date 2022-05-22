# napi-nanoid


[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid) [![downloads](https://img.shields.io/npm/dm/napi-nanoid?color=purple)](https://www.npmjs.com/package/napi-nanoid)

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

`nanoid compare`

```rust
  js-nanoid (non-secure):
    1 908 066 ops/s, ±0.54%

  js-nanoid:
    2 646 331 ops/s, ±0.21%

  napi-nanoid (non-secure):
    4 670 219 ops/s, ±0.13%
```

`performance of all`

```rust
shortid                        22,556 ops/sec
cuid                           94,301 ops/sec
secure-random-string          186,282 ops/sec
uuid                          870,076 ops/sec
js-nanoid (non-secure)      1,889,717 ops/sec
js-nanoid                   2,514,450 ops/sec
napi-nanoid (non-secure)    4,387,239 ops/sec
crypto.randomUUID          10,256,820 ops/sec
hyperid                    12,161,546 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) CPU E5-2673 v4 @ 2.30GHz, Node.js 16.15.0

[(runs: 6542770267)](https://github.com/rustq/napi-nanoid/runs/6542770267)

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
