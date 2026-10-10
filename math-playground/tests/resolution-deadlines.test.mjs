import test from 'node:test';
import assert from 'node:assert/strict';
import {deadlineGroups} from '../../static/resolution-control/deadlines.mjs';

const record = (id, deadline, title = id, area = 'MATH') =>
  ({id, spec:{title,area},schedule:{deadline}});
const date = value => ({precision:'date_only',value});
const at = value => ({precision:'instant',value});
const locate = (result, key) => result.categories.find(group=>group.key===key).entries;

test('deadline overview preserves SA calendar boundaries, date-only uncertainty and a stable chronological order', () => {
  const input = {commitments:[
    record('later',date('2026-10-18')),
    record('soon-seven',date('2026-10-17')),
    record('unknown',null),
    record('today-untimed',date('2026-10-10')),
    record('today-future',at('2026-10-10T19:30:00Z')),
    record('past-minute',at('2026-10-10T18:59:00Z')),
    record('yesterday',date('2026-10-09')),
    record('tomorrow-early',at('2026-10-10T22:05:00Z')),
    record('today-past',at('2026-10-10T08:00:00Z'))
  ]};
  // Observed 19:00 UTC = 21:00 SAST, fixed UTC+02:00.
  const result = deadlineGroups(input,'2026-10-10T19:00:00Z');
  assert.equal(result.count,9);
  assert.equal(result.today,'2026-10-10');
  assert.deepEqual(locate(result,'overdue').map(x=>x.id),['yesterday','today-past','past-minute']);
  assert.deepEqual(locate(result,'today').map(x=>x.id),['today-untimed','today-future']);
  assert.deepEqual(locate(result,'soon').map(x=>x.id),['tomorrow-early','soon-seven']);
  assert.deepEqual(locate(result,'later').map(x=>x.id),['later']);
  assert.deepEqual(locate(result,'unknown').map(x=>x.id),['unknown']);
  assert.equal(locate(result,'today')[0].time,null); // no fabricated midnight
  assert.equal(locate(result,'soon')[0].day,'2026-10-11'); // day rolled in SA
  assert.equal(locate(result,'today')[1].time,'21:30 SAST (UTC+02:00)');
});
test('deadline overview sorts exact same-day times before later instants without using browser timezone', () => {
  const result=deadlineGroups({commitments:[
    record('late',at('2026-10-22T15:00:00Z')),
    record('early',at('2026-10-22T12:00:00Z')),
    record('untimed',date('2026-10-22')),
    record('mid',at('2026-10-22T13:00:00Z')),
  ]},'2026-10-10T12:00:00Z');
  assert.deepEqual(locate(result,'later').map(x=>x.id),['untimed','early','mid','late']);
  assert.equal(locate(result,'later')[0].time,null);
});
test('deadline overview is read-only and handles an empty workspace', () => {
  const input={commitments:[]};
  assert.equal(deadlineGroups(input,'2026-10-10T00:00:00Z').count,0);
  assert.deepEqual(input,{commitments:[]});
  assert.throws(()=>deadlineGroups(input,'not-an-instant'),/authoritative/);
});
