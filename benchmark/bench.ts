import crypto from 'crypto'

import benchmark, { Event } from 'benchmark'
import benny from 'benny'
import cuid from 'cuid'
import hyperidFactory from 'hyperid'
import { nanoid as nanoidJs } from 'nanoid'
import { nanoid as nanoidJsNonSecure } from 'nanoid/non-secure'
import srs from 'secure-random-string'
import shortid from 'shortid'
import * as uuid from 'uuid'

import { nanoid as nanoidNapi, nonSecure as nanoidNapiNonSecure } from '../index'

async function run() {
  // nanoid compare
  // compare the performance between the js-nanoid and napi-nanoid only
  // 仅对 nanoid 的 node.js 和 napi 版本自身做对比

  process.stdout.write('\n* nanoid compare\n')

  await benny.suite(
    'nanoid compare',

    benny.add('js-nanoid', () => {
      nanoidJs()
    }),
    benny.add('napi-nanoid', () => {
      nanoidNapi()
    }),

    benny.add('js-nanoid (non-secure)', () => {
      nanoidJsNonSecure()
    }),
    benny.add('napi-nanoid (non-secure)', () => {
      nanoidNapiNonSecure()
    }),

    benny.cycle(),
    benny.complete(),
  )

  // performance of all
  // show the performance of all of the follow libs
  // 展示下方所有库的性能对比

  process.stdout.write('\n* performance of all\n')

  const bench = new benchmark.Suite()
  const hyperid = hyperidFactory()

  bench
    .add('shortid', () => {
      shortid()
    })
    .add('cuid', () => {
      cuid()
    })
    .add('secure-random-string', () => {
      srs()
    })
    .add('uuid', () => {
      uuid.v4()
    })
    .add('js-nanoid (non-secure)', () => {
      nanoidJsNonSecure()
    })
    .add('js-nanoid (secure)', () => {
      nanoidJs()
    })

    .add('napi-nanoid (secure)', () => {
      nanoidNapi()
    })
    .add('napi-nanoid (non-secure)', () => {
      nanoidNapiNonSecure()
    })

    .add('crypto.randomUUID', () => {
      crypto.randomUUID()
    })
    .add('hyperid', () => {
      hyperid()
    })
    .on('cycle', (event: Event) => {
      process.stdout.write(
        `${event.target.name!} ${Number(event.target.hz!.toFixed(0)).toLocaleString('en-US').padStart(20)} ops/sec\n`,
      )
    })
    .run()
}

run().catch((e) => {
  console.error(e)
})
