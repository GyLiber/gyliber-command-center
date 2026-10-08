import test from 'node:test';
import assert from 'node:assert/strict';
import {parseScope,scopeText,coverage,canConfirm,keyLabel} from '../../static/resolution-control/readiness.mjs';

const key={item:'metric_axioms',dimension:'written'};
const item=()=>({key,mapped:null,deployed:null,stress_test:null,verification:null});
test('scope is explicit, bounded, ordered and never guesses missing dimensions',()=>{
  assert.deepEqual(parseScope('metric_axioms | written\n metric_axioms | oral'),[
    key,{item:'metric_axioms',dimension:'oral'}]);
  assert.equal(scopeText({items:[{key}]}),'metric_axioms | written');
  assert.match(keyLabel(key),/written/);
  for(const value of ['', 'bad value | written','metric_axioms | guessed','metric_axioms | written\nmetric_axioms | written','metric_axioms | written | oral'])
    assert.throws(()=>parseScope(value));
  assert.throws(()=>parseScope(Array(129).fill('metric_axioms | written').join('\n')),/between 1 and 128/);
});
test('verified finish requires every current dimension, no blocker, explicit confirmation',()=>{
  const x=item(),r={scope_identified:true,scope_revision:4,items:[x],blocking_threats:0,verified_finish:null};
  assert.equal(coverage(x,4),'identified');assert.equal(canConfirm(r),false);
  x.mapped={kind:'text',value:'notes.tex'};assert.equal(coverage(x,4),'mapped');
  x.deployed={kind:'text',value:'notes.tex'};assert.equal(coverage(x,4),'deployed');
  x.stress_test={scope_revision:4,outcome:'pass'};assert.equal(coverage(x,4),'stress_tested');
  x.verification={scope_revision:4,outcome:'pass'};assert.equal(coverage(x,4),'verified');assert.equal(canConfirm(r),true);
  r.blocking_threats=1;assert.equal(canConfirm(r),false);r.blocking_threats=0;
  r.verified_finish='2026-10-08T19:00:00Z';assert.equal(canConfirm(r),false);r.verified_finish=null;
  r.items.push(item());assert.equal(canConfirm(r),false);r.items.pop();
  r.scope_identified=false;assert.equal(canConfirm(r),false);r.scope_identified=true;
  r.scope_revision++;assert.equal(coverage(x,5),'stale');assert.equal(canConfirm(r),false);
  x.stress_test={scope_revision:5,outcome:'fail'};assert.equal(coverage(x,5),'stress_test_failed');
  x.stress_test={scope_revision:5,outcome:'pass'};x.verification={scope_revision:5,outcome:'fail'};
  assert.equal(coverage(x,5),'verification_failed');assert.equal(canConfirm(r),false);
});
