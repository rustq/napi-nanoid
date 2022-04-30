import b from 'benny'
import { nanoid as nanoidJs } from 'nanoid'

import * as NapiNanoID from '../index'

async function run() {
  await b.suite(
    'nanoid',

    b.add('nanoidJs', () => {
      nanoidJs()
    }),

    b.add('NapiNanoID', () => {
      NapiNanoID.nanoid()
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
