/* eslint-disable no-console */
import test from 'ava'

import { nonSecure } from '../index'

test('non-secure', (t) => {
  for (let i = 0; i < 10000; i++) {
    const id = nonSecure()
    if (i % 100 === 99) {
      console.log(i, id)
    }
    t.is(id.length, 21)
  }
})
