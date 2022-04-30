import test from 'ava'

import { nanoid } from '../index'

test('sync function from native code', (t) => {
  for (let i = 1; i < 100; i++) {
    const id = nanoid()
    // eslint-disable-next-line no-console
    console.log(id)
    t.is(id.length, 21)
  }
})
