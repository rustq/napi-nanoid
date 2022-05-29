/* eslint-disable no-console */
import test from 'ava'

import { nanoid } from '../index'

test('nanoid', (t) => {
  const set = new Set()
  for (let i = 0; i < 10000; i++) {
    const id = nanoid()
    t.is(set.has(id), false)
    set.add(id)
    if (i % 100 === 99) {
      console.log(i, id)
    }
    t.is(id.length, 21)
  }
  t.is(set.size, 10000)
})
