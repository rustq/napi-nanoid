import test from 'ava'

import { nanoid, Factory, customSize, customAlphabet } from '../index'

test('sync function from native code', (t) => {
  for (let i = 1; i < 100; i++) {
    const id = nanoid()
    // eslint-disable-next-line no-console
    console.log(id)
    t.is(id.length, 21)
  }
})

test('Factory 31', (t) => {
  const instance = new Factory(31)
  t.is(instance.nanoid().length, 31)
})

test('Factory undefined', (t) => {
  const instance = new Factory(undefined)
  t.is(instance.nanoid().length, 21)
})

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
