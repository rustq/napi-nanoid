# napi-nanoid


[![license](https://img.shields.io/npm/l/napi-nanoid?color=cyan)](https://revolunet.mit-license.org/) [![npm](https://img.shields.io/npm/v/napi-nanoid?color=orange)](https://www.npmjs.com/package/napi-nanoid) [![downloads](https://img.shields.io/npm/dm/napi-nanoid?color=violet)](https://www.npmjs.com/package/napi-nanoid)

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

```shell
Test bindings on Linux-x64-gnu - node@16

  uuid:
    894 223 ops/s, ±0.16%     | slowest, 81.08% slower

  nanoid-js:
    2 666 811 ops/s, ±0.43%   | 43.57% slower

  napi-nanoid:
    4 725 662 ops/s, ±0.13%   | fastest

Finished 3 cases!
  Fastest: napi-nanoid
  Slowest: uuid
Done in 17.04s.
```

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