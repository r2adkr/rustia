import test from 'ava'

import { Rustia } from '../index'

test('Is Rustia Instant configured correctly?', (t) => {
  const config = {
    nodes: [{ name: "test", host: "localhost:2333", secure: true }, { name: "test", host: "localhost:3000", secure: false }]
  };

  const instance = new Rustia(config);
  t.truthy(instance);
  t.deepEqual(config.nodes, instance.getNodes())
})
