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

```rust
shortid                  23,544 ops/sec
cuid                    103,653 ops/sec
secure-random-string    201,080 ops/sec
uuid                    896,956 ops/sec
js-nanoid             2,582,868 ops/sec
napi-nanoid           4,649,540 ops/sec
crypto.randomUUID    11,813,899 ops/sec
hyperid              14,359,518 ops/sec
```

Benchmark configuration: Linux x64 gnu, Intel(R) Xeon(R) CPU E5-2673 v4 @ 2.30GHz, Node.js 16.15.0

[(runs: 6301184360)](https://github.com/rustq/napi-nanoid/runs/6301184360)

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
