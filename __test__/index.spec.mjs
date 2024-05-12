import test from 'ava'

import { checkText } from '../index.js'

test('text containing 1 slurs with 1 as maxLevel', (t) => {
  const isProfanity = checkText('You are a cunt', 1);

  t.is(isProfanity, true)
})

test('text containing 1 slurs with 2 as maxLevel', (t) => {
  const isProfanity = checkText('You are a cunt', 2);

  t.is(isProfanity, false)
})
