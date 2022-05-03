import b from 'benny'
import { nanoid as nanoidJs } from 'nanoid'
import * as uuid from 'uuid'

import { nanoid } from '../index'

async function run() {
  await b.suite(
    'nanoid',

    b.add('uuid', () => {
      uuid.v4()
    }),

    b.add('nanoid-js', () => {
      nanoidJs()
    }),

    b.add('napi-nanoid', () => {
      nanoid()
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
