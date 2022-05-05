import crypto from 'crypto'

import benchmark, { Event } from 'benchmark'
import cuid from 'cuid'
import hyperidFactory from 'hyperid'
import { nanoid as nanoidJs } from 'nanoid'
import srs from 'secure-random-string'
import shortid from 'shortid'
import * as uuid from 'uuid'

import { nanoid as nanoidNapi } from '../index'

const b = new benchmark.Suite()
const hyperid = hyperidFactory()

b.add('shortid', () => {
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
  .add('js-nanoid', () => {
    nanoidJs()
  })

  .add('napi-nanoid', () => {
    nanoidNapi()
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
