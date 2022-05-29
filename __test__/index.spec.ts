/* eslint-disable no-console */
import test from 'ava'

import { nanoid /*, customSize, customAlphabet */ } from '../index'

test('sync function from native code', (t) => {
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

/* custom method won't be added into 0.0.1 yet until the napi case be resolved */
/*

test('custom size nanoid', (t) => {
  for (let i = 1; i < 100; i++) {
    const size = Math.floor(25 * Math.random()) + 8
    const id = customSize(size)
    t.is(id.length, size)
  }
})

test('custom alphabet nanoid', (t) => {
  const alphabet = 'abcdefghijklmnopqrstuvwxy0123456789'
  for (let i = 1; i < 100; i++) {
    const id = customAlphabet(21, alphabet)
    t.notRegex(id, new RegExp(`[^${alphabet}]`))
  }
})

*/
