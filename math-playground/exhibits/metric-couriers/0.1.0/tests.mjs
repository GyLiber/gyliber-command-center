import test from 'node:test';
import assert from 'node:assert/strict';
import { points, distance, evaluate } from './engine.mjs';
test('all nine distances match the independent finite table and metric axioms', () => {
  const table = [[0,2,3],[2,0,1],[3,1,0]];
  for (let i=0;i<3;i++) for (let j=0;j<3;j++) {
    const d=distance(points[i],points[j]);
    assert.equal(d,table[i][j]); assert.ok(d>=0);
    assert.equal(d===0,i===j); assert.equal(d,distance(points[j],points[i]));
  }
});
test('all 27 routes satisfy triangle inequality; equality and strict detours occur', () => {
  let equal=0, strict=0;
  for (const x of points) for (const y of points) for (const z of points) {
    const state=evaluate({x,y,z});
    assert.ok(state.direct<=state.detour); assert.equal(state.slack,state.detour-state.direct);
    if (state.equality) equal++; else strict++;
    assert.deepEqual(state,evaluate({x,y,z}));
  }
  assert.ok(equal>0 && strict>0);
  assert.deepEqual(evaluate(),{x:1,y:3,z:4,direct:3,first:2,second:1,detour:3,slack:0,equality:true});
  assert.equal(evaluate({x:1,y:4,z:3}).slack,2);
});
test('invalid points fail explicitly; repeated points and zero-length routes work', () => {
  for (const bad of [0,2,5,-1,NaN,Infinity,'1',null,{},1.5]) {
    assert.throws(()=>distance(bad,1),RangeError);
    for (const key of ['x','y','z']) assert.throws(()=>evaluate({[key]:bad}),RangeError);
  }
  assert.equal(evaluate({x:3,y:3,z:3}).detour,0);
  assert.ok(Object.isFrozen(points)); assert.ok(Object.isFrozen(evaluate()));
});
