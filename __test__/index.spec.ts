import test from 'ava'

import { nanoid } from '../index'

test('sync function from native code', (t) => {
  t.is(nanoid(), 'hello')
})
