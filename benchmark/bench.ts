import b from 'benny'
import { nanoid as nanoidJs } from 'nanoid'
import * as uuid from 'uuid'

import { nanoid as nanoidNapi } from '../index'

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

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
