import test from 'node:test';
import assert from 'node:assert/strict';
import {reportQuery,backupShape,metadataShape,eventPage,deletionPhrase} from '../../static/resolution-control/private.mjs';

test('report query requires valid date, deliberate fixed offset and bounded revision',()=>{
  assert.equal(reportQuery('2026-10-09','120',''),'/report?date=2026-10-09&offset_minutes=120');
  assert.equal(reportQuery('2026-10-09','-840','0'),'/report?date=2026-10-09&offset_minutes=-840&cutoff_revision=0');
  for(const [date,offset,cutoff] of [['2026-02-30',120,''],['2026-10-09',841,''],['2026-10-09',120,'-1'],['2026-10-09','2.5',''],['bad',120,'']])
    assert.throws(()=>reportQuery(date,offset,cutoff));
});
test('backup and deletion ledger must have recognizable separate versioned structure',()=>{
  const hash='a'.repeat(64), backup={schema_version:1,source_generation:'root',lineage:['root'],events:[],checksum:hash,exported_at:'2026-10-08T00:00:00Z'};
  const ledger={schema_version:1,deleted_generations:[],checksum:hash};
  assert.equal(backupShape(backup),true);assert.equal(metadataShape(ledger),true);
  assert.equal(backupShape({...backup,schema_version:2}),false);
  assert.equal(backupShape({...backup,checksum:'wrong'}),false);
  assert.equal(metadataShape({...ledger,deleted_generations:Array(4097).fill('x')}),false);
  assert.equal(metadataShape({...ledger,schema_version:2}),false);
  assert.equal(deletionPhrase,'DELETE MY RESOLUTION CONTROL DATA');
});
test('history cursor rejects duplicate, decreasing, excessive and malformed events',()=>{
  const e=revision=>({revision,at:'2026-10-09T06:00:00Z',origin:'live',command:{type:'create_resolution'}});
  assert.equal(eventPage([e(1),e(2)],0),2);
  assert.equal(eventPage([],2),2);
  assert.throws(()=>eventPage([e(2),e(2)],1));
  assert.throws(()=>eventPage([e(1)],1));
  assert.throws(()=>eventPage(Array.from({length:101},(_,i)=>e(i+1)),0));
  assert.throws(()=>eventPage([{revision:1,at:'t',origin:'live',command:{}}],0));
});
