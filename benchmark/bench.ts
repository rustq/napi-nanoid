import b from 'benny'
import hyperidFactory from 'hyperid'
import { nanoid as nanoidJs } from 'nanoid'
import * as uuid from 'uuid'

import { nanoid as nanoidNapi } from '../index'

const hyperid = hyperidFactory()

async function run() {
  await b.suite(
    'napi-nanoid',

    b.add('napi-nanoid', () => {
      nanoidNapi()
    }),

    b.add('js-nanoid', () => {
      nanoidJs()
    }),

    b.add('uuid', () => {
      uuid.v4()
    }),

    b.add('hyperid', () => {
      hyperid()
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
