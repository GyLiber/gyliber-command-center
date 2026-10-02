import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import * as circle from '../exhibits/giant-pi/0.1.0/engine.mjs';
import * as sequence from '../exhibits/snug-tails/0.1.0/engine.mjs';
import * as shuffle from '../exhibits/creature-shuffle/0.1.0/engine.mjs';
import * as metaphor from '../exhibits/memory-cloud/0.1.0/engine.mjs';

test('circle scaling preserves C/d and unwrapped length', () => {
  for (const radius of [.5, 1, 1.7, 3]) {
    const state = circle.evaluate({ radius, unwrap: 1 });
    assert.equal(state.circumference / state.diameter, Math.PI);
    assert.equal(state.ribbonLength, state.circumference);
    assert.equal(circle.evaluate({ radius, unwrap: 0 }).ribbonLength, 0);
  }
});
test('epsilon threshold witnesses the whole reciprocal tail, including strict boundary cases', () => {
  for (const epsilon of [.02, .1, .2, .25, .5, 1/7]) {
    const { threshold } = sequence.evaluate({ epsilon });
    for (const n of [threshold, threshold+1, threshold+50, threshold+10000]) assert.ok(1/n < epsilon);
    assert.equal(sequence.evaluate({ n: 10, epsilon: .1 }).inside, false);
  }
});
test('factoradic enumeration is bijective for every supported set size', () => {
  for (let n = 0; n <= 6; n++) {
    const seen = new Set();
    for (let i = 0; i < shuffle.factorial(n); i++) {
      const order = shuffle.permutation(n, i);
      assert.equal(new Set(order).size, n);
      assert.deepEqual([...order].sort((a,b)=>a-b), Array.from({length:n},(_,i)=>i));
      seen.add(JSON.stringify(order));
    }
    assert.equal(seen.size, shuffle.factorial(n));
  }
  assert.throws(() => shuffle.permutation(4, 24));
});
test('all mathematical input boundaries stay finite and deterministic', () => {
  for (const engine of [circle, sequence, shuffle, metaphor]) {
    const input = { radius: Infinity, unwrap: -99, epsilon: NaN, n: 999, index: -2, spread: 77 };
    assert.deepEqual(engine.evaluate(input, .4), engine.evaluate(input, .4));
    const state = engine.evaluate(input);
    for (const value of Object.values(state)) if (typeof value === 'number') assert.ok(Number.isFinite(value));
  }
});
test('committed demo engines and renderer match their manifest digests', async () => {
  const core = await readFile(new URL('../runtime/engine-core.mjs', import.meta.url), 'utf8');
  const renderer = await readFile(new URL('../runtime/renderer.mjs', import.meta.url));
  for (const id of ['giant-pi','snug-tails','creature-shuffle','memory-cloud']) {
    const root = new URL(`../exhibits/${id}/0.1.0/`, import.meta.url);
    const manifest = JSON.parse(await readFile(new URL('manifest.json', root)));
    const engine = await readFile(new URL('engine.mjs', root));
    assert.equal(createHash('sha256').update(engine).digest('hex'), manifest.engine_sha256);
    assert.equal(createHash('sha256').update(renderer).digest('hex'), manifest.renderer_sha256);
    assert.ok(engine.toString().endsWith(core));
    assert.equal(manifest.formal_version, '0.2.0');
  }
});
