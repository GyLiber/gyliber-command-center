import test from 'node:test';
import assert from 'node:assert/strict';
import {text,reference,timeFact,schedule,plan,displayTime,exactLocal} from '../../static/resolution-control/model.mjs';
const data=(values)=>new Map(Object.entries({deadline_precision:'unknown',earliest_precision:'unknown',buffer:'',...values}));
test('Resolution Control input model preserves unknowns, exact offsets and schedule boundaries',()=>{
  assert.deepEqual(schedule(data({})),{deadline:null,earliest_finish:null,buffer_minutes:null});
  assert.equal(schedule(data({buffer:'0'})).buffer_minutes,0);
  const controls={deadline_precision:'instant',deadline:'2026-10-09T12:00',earliest_precision:'instant',earliest:'2026-10-09T11:00:00',buffer:'60'};
  assert.equal(schedule(data(controls)).deadline.value,'2026-10-09T12:00:00+02:00');
  assert.throws(()=>schedule(data({...controls,earliest:'2026-10-09T11:00:01'})),/before deadline/);
  assert.deepEqual(timeFact('date_only','2026-10-09'),{precision:'date_only',value:'2026-10-09'});
  assert.throws(()=>timeFact('instant','2026-10-09'),/complete date/);
  for(const buffer of ['-1','0.1','1e2','525601'])assert.throws(()=>schedule(data({buffer})),/whole minutes/);
  assert.equal(exactLocal('2026-10-09T10:00:07Z'),'2026-10-09T12:00:07');
  assert.match(displayTime({precision:'instant',value:'2026-10-09T10:00:07Z'}),/12:00:07 · UTC\+02:00/);
  assert.match(displayTime({precision:'date_only',value:'2026-10-09'}),/date only/);
  assert.match(displayTime({precision:'instant',value:'9999-12-31T23:00:00Z'}),/outside/);
});
test('Resolution Control references and text are bounded data, never executable input',()=>{
  assert.equal(reference('javascript:alert(1)').kind,'text');
  assert.deepEqual(reference('proof.tex'),{kind:'text',value:'proof.tex'});
  assert.deepEqual(reference('https://example.invalid/proof'),{kind:'web',value:'https://example.invalid/proof'});
  for(const value of ['https://u:p@example.invalid','https://example.invalid/?secret=x','https://example.invalid/#key','https://example.invalid/a b'])assert.throws(()=>reference(value),/Remove/);
  assert.throws(()=>text('é'.repeat(81),160),/UTF-8/);
  assert.throws(()=>text('bad\0',160),/control/);
  assert.deepEqual(plan(data({instruction:'One proof'})),{instruction:'One proof',expected_artifact:null,verification_method:null,start_reference:null});
});
