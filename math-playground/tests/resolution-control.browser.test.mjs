import test, {after} from 'node:test';
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {readFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {chromium} from 'playwright';
const root=fileURLToPath(new URL('../../',import.meta.url));
let view,writes,operations,interrupt,disabled,unauthorized;
function reset() {
  view={generation:null,revision:0,observed_at:'2026-10-08T18:00:07Z',workspace:{resolutions:[],commitments:[],actions:[],threats:[],current_action:null},buffers:[],imported_history:false};
  writes=[];operations=new Map();interrupt=false;disabled=false;unauthorized=false;
}
reset();
const server=createServer(async(req,res)=>{
  try {
    const path=new URL(req.url,'http://localhost').pathname;
    const send=(status,content,type='application/json')=>{res.writeHead(status,{'Content-Type':type,'Cache-Control':'no-store','Content-Security-Policy':"default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'"});res.end(typeof content==='string'||Buffer.isBuffer(content)?content:JSON.stringify(content));};
    const files={'/command/resolution-control':'static/resolution-control.html','/static/app.css':'static/app.css','/static/resolution-control/style.css':'static/resolution-control/style.css','/static/resolution-control/app.mjs':'static/resolution-control/app.mjs','/static/resolution-control/model.mjs':'static/resolution-control/model.mjs',
  '/static/resolution-control/deadlines.mjs':'static/resolution-control/deadlines.mjs','/static/resolution-control/readiness.mjs':'static/resolution-control/readiness.mjs','/static/resolution-control/private.mjs':'static/resolution-control/private.mjs'};
    if(files[path])return send(200,await readFile(resolve(root,files[path])),path.endsWith('.mjs')?'text/javascript':path.endsWith('.css')?'text/css':'text/html');
    if(path==='/api/resolution-control')return unauthorized?send(401,{error:'authentication_required'}):disabled?send(503,{error:'feature_disabled'}):send(200,{viewer:'github-1',csrf:'test-csrf',view,limits:{events:2048},import_notice:'Imported history is a user-supplied claim.'});
    if(path==='/api/resolution-control/commands' && req.method==='POST') {
      assert.equal(req.headers['x-resolution-csrf'],'test-csrf');
      let body='';for await(const chunk of req)body+=chunk;
      const packet=JSON.parse(body);writes.push(packet);
      const old=operations.get(packet.meta.operation_id);
      if(old){assert.equal(body,old.body);return send(200,{...old.ack,replayed:true});}
      if(packet.meta.generation!==view.generation || packet.meta.expected_revision!==view.revision)return send(409,{error:'refresh_required'});
      const c=packet.command,w=view.workspace;
      if(c.type==='create_resolution')w.resolutions.push({id:c.id,spec:c.spec});
      else if(c.type==='update_resolution')w.resolutions.find(r=>r.id===c.id).spec=c.spec;
      else if(c.type==='create_commitment'){w.commitments.push({id:c.id,resolution:c.resolution,spec:c.spec,schedule:c.schedule,readiness:{scope_revision:0,scope_identified:false,items:[],blocking_threats:0,verified_finish:null}});view.buffers.push({commitment:c.id,unfinished:{state:'target_unknown',deadline_reached:false},finished:null});}
      else if(c.type==='update_commitment')Object.assign(w.commitments.find(r=>r.id===c.id),{spec:c.spec,schedule:c.schedule});
      else if(c.type==='create_action')w.actions.push({id:c.id,commitment:c.commitment,action:{plan:c.plan,status:'new',artifact:null}});
      else if(c.type==='update_action')Object.assign(w.actions.find(a=>a.id===c.id).action,{plan:c.plan,status:'new'});
      else if(c.type==='select_action')w.current_action=c.id;
      else if(['identify_scope','mark_scope_unknown','map_material','deploy_material','record_evidence','confirm_readiness','reopen_readiness','add_threat','resolve_threat'].includes(c.type)) {
        const commitment=c.commitment ?? w.threats.find(t=>t.id===c.id)?.commitment;
        const r=w.commitments.find(item=>item.id===commitment).readiness;
        const matching=key=>r.items.find(item=>item.key.item===key.item&&item.key.dimension===key.dimension);
        if(c.type==='identify_scope') {
          r.scope_identified=true;r.scope_revision++;r.verified_finish=null;
          r.items=c.items.map(key=>matching(key)??{key,mapped:null,deployed:null,stress_test:null,verification:null});
        }
        if(c.type==='mark_scope_unknown'){r.scope_identified=false;r.scope_revision++;r.verified_finish=null;}
        if(c.type==='map_material'){const item=matching(c.key);item.mapped=c.artifact;item.deployed=null;r.scope_revision++;r.verified_finish=null;}
        if(c.type==='deploy_material'){const item=matching(c.key);assert.ok(item.mapped);item.deployed=c.artifact;r.scope_revision++;r.verified_finish=null;}
        if(c.type==='record_evidence') {
          const item=matching(c.input.key);
          assert.ok(item.deployed);
          const evidence={...c.input,scope_revision:r.scope_revision,actor:'github-1',recorded_at:view.observed_at};
          if(c.input.stage==='stress_test'){item.stress_test=evidence;item.verification=null;} else item.verification=evidence;
          r.verified_finish=null;
        }
        if(c.type==='confirm_readiness'){
          assert.ok(r.scope_identified&&r.blocking_threats===0&&r.items.length>0&&r.items.every(x=>x.verification?.outcome==='pass'&&x.verification?.scope_revision===r.scope_revision&&x.stress_test?.outcome==='pass'&&x.stress_test?.scope_revision===r.scope_revision));
          r.verified_finish=view.observed_at;
        }
        if(c.type==='reopen_readiness'){assert.ok(r.verified_finish);r.verified_finish=null;r.scope_revision++;}
        if(c.type==='add_threat'){w.threats.push({id:c.id,commitment:c.commitment,spec:c.spec,resolved:false});}
        if(c.type==='resolve_threat'){w.threats.find(t=>t.id===c.id).resolved=true;}
        if(['add_threat','resolve_threat'].includes(c.type)){
          r.blocking_threats=w.threats.filter(t=>t.commitment===commitment&&!t.resolved&&t.spec.blocking).length;
          if(r.blocking_threats>0){r.scope_revision++;r.verified_finish=null;}
          assert.ok(r.blocking_threats<=128);
        }
      }
      else {
        const action=w.actions.find(a=>a.id===c.id).action;
        const statuses={ready_action:'ready',start_action:'in_progress',block_action:'blocked',cancel_action:'cancelled',complete_action:'completed',reopen_action:'new'};
        assert.ok(statuses[c.type]);action.status=statuses[c.type];
        if(c.type==='complete_action')action.artifact=c.artifact;
        if(['complete_action','cancel_action'].includes(c.type))w.current_action=null;
      }
      view.generation='generation-1';view.revision++;
      const ack={generation:view.generation,revision:view.revision,replayed:false};operations.set(packet.meta.operation_id,{body,ack});
      if(interrupt){interrupt=false;return send(200,'{"revision":');} // Accepted write, deliberately unusable acknowledgement.
      return send(200,ack);
    }
    send(404,{});
  }catch{res.writeHead(500);res.end('{}');}
});
await new Promise(done=>server.listen(0,'127.0.0.1',done));
const base=`http://127.0.0.1:${server.address().port}`;
const browser=await chromium.launch({headless:true});
after(async()=>{await browser.close();await new Promise(done=>server.close(done));});
const pageErrors=[];
async function page() {const p=await browser.newPage({viewport:{width:1280,height:1000}});p.on('pageerror',e=>pageErrors.push(e.message));await p.goto(`${base}/command/resolution-control`);await p.getByText('State refreshed. No changes were made.',{exact:true}).waitFor();return p;}
async function saved(p) {await p.getByText('Saved and refreshed.',{exact:true}).waitFor();}
async function outcome(p,title='Synthetic outcome') {await p.getByLabel('Resolution title',{exact:true}).fill(title);await p.getByRole('button',{name:'Capture resolution',exact:true}).click();await saved(p);}
async function commitment(p) {await p.getByLabel('Commitment title',{exact:true}).fill('Metric preparation');await p.getByLabel('Deadline precision',{exact:true}).selectOption('date_only');await p.getByLabel('Hard deadline',{exact:true}).fill('2026-10-12');await p.getByRole('button',{name:'Capture commitment',exact:true}).click();await saved(p);}
test('Resolution Control keyboard/mobile capture, refine, select, start and externalize',async()=>{
  reset();pageErrors.length=0;const p=await page();
  try {
    await p.getByLabel('Resolution title',{exact:true}).focus();await p.keyboard.press('Tab');assert.equal(await p.getByLabel('Objective (optional)',{exact:true}).evaluate(n=>n===document.activeElement),true);
    await outcome(p,'Metric <img src=x onerror=alert(1)>');await commitment(p);
    assert.equal(view.workspace.commitments[0].schedule.deadline.precision,'date_only');assert.equal(view.workspace.commitments[0].schedule.buffer_minutes,null);
    assert.equal(await p.locator('#ledger img').count(),0);assert.match(await p.locator('#ledger').textContent(),/date only/);
    await p.getByLabel('Action instruction',{exact:true}).fill('Construct one original proof');
    await p.getByRole('button',{name:'Capture action',exact:true}).click();await saved(p);
    assert.equal(view.workspace.current_action,null);
    await p.locator('.obligation summary').click();await p.getByRole('button',{name:'Select action',exact:true}).click();await saved(p);
    assert.equal(await p.getByRole('button',{name:'Mark ready',exact:true}).isDisabled(),true);
    await p.getByRole('button',{name:'Edit plan',exact:true}).click();
    await p.getByLabel('Expected output (needed before ready)',{exact:true}).fill('proof.tex');await p.getByLabel('Verification method (needed before ready)',{exact:true}).fill('Check each implication');
    await p.getByLabel('Start reference (optional; existing work location)',{exact:true}).fill('javascript:alert(1)');
    await p.getByRole('button',{name:'Save action',exact:true}).click();await saved(p);
    assert.equal(await p.locator('#current-content a').count(),0);
    await p.getByRole('button',{name:'Mark ready',exact:true}).click();await saved(p);
    await p.getByRole('button',{name:'Start action',exact:true}).click();await saved(p);
    await p.getByLabel('Completed output reference',{exact:true}).fill('https://example.invalid/proof');
    await p.getByRole('button',{name:'Refresh state',exact:true}).click();await p.getByText('State refreshed. No changes were made.',{exact:true}).waitFor();
    assert.equal(await p.getByLabel('Completed output reference',{exact:true}).inputValue(),'https://example.invalid/proof');
    await p.getByRole('button',{name:'Record completed output',exact:true}).click();await saved(p);
    assert.equal(view.workspace.actions[0].action.status,'completed');assert.equal(view.workspace.current_action,null);
    assert.match(await p.locator('#current-content').textContent(),/No action is selected/);
    assert.equal(writes.length,8);assert.equal(new Set(writes.map(w=>w.meta.operation_id)).size,8);
    assert.ok(writes.every(w=>!('owner' in w) && !('actor' in w.command)));
    assert.equal(await p.locator('body').evaluate(n=>getComputedStyle(n).colorScheme),'dark');
    if(process.env.PLAYGROUND_SCREENSHOT_DIR){await mkdir(process.env.PLAYGROUND_SCREENSHOT_DIR,{recursive:true});await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-desktop.png'),fullPage:true});}
    await p.setViewportSize({width:390,height:844});assert.equal(await p.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);
    await p.getByRole('button',{name:'Refresh state',exact:true}).focus();await p.keyboard.press('Tab');assert.equal(await p.evaluate(()=>getComputedStyle(document.activeElement).outlineStyle),'solid');
    if(process.env.PLAYGROUND_SCREENSHOT_DIR)await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-mobile.png'),fullPage:true});
    assert.deepEqual(pageErrors,[]);
  }finally{await p.close();}
});
test('Resolution Control uncertain saves retry identically and stale drafts are not rebased',async()=>{
  reset();pageErrors.length=0;const p=await page();
  try {
    interrupt=true;await p.getByLabel('Resolution title',{exact:true}).fill('Retained draft');await p.getByRole('button',{name:'Capture resolution',exact:true}).click();
    await p.getByText(/Save outcome is uncertain/).waitFor();assert.equal(writes.length,1);assert.equal(await p.getByLabel('Resolution title',{exact:true}).inputValue(),'Retained draft');
    assert.equal(await p.getByRole('button',{name:'Capture resolution',exact:true}).isDisabled(),true);
    await p.getByRole('button',{name:'Refresh state',exact:true}).click();await p.getByText(/An earlier save is still uncertain/).waitFor();assert.equal(writes.length,1);
    await p.getByRole('button',{name:'Retry the same request',exact:true}).click();await p.getByText(/earlier save was confirmed/).waitFor();
    assert.deepEqual(writes[0],writes[1]);assert.equal(view.revision,1);assert.equal(view.workspace.resolutions.length,1);
    await commitment(p);await p.getByRole('button',{name:'Edit controls',exact:true}).click();await p.getByLabel('Commitment title',{exact:true}).fill('Local draft');
    const original=view.revision;view.workspace.commitments[0].spec.title='Other tab';view.revision++;
    await p.getByRole('button',{name:'Refresh state',exact:true}).click();await p.getByText('State refreshed. No changes were made.',{exact:true}).waitFor();
    await p.getByRole('button',{name:'Save commitment',exact:true}).click();await p.getByText(/Another edit or recovery/).waitFor();
    assert.equal(writes.at(-1).meta.expected_revision,original);assert.equal(view.workspace.commitments[0].spec.title,'Other tab');assert.equal(await p.getByLabel('Commitment title',{exact:true}).inputValue(),'Local draft');
    p.once('dialog',dialog=>dialog.accept());await p.getByRole('button',{name:'Edit controls',exact:true}).click();assert.equal(await p.getByLabel('Commitment title',{exact:true}).inputValue(),'Other tab');
    await p.getByLabel('Commitment title',{exact:true}).fill('Reviewed draft');await p.getByRole('button',{name:'Save commitment',exact:true}).click();await saved(p);assert.equal(view.workspace.commitments[0].spec.title,'Reviewed draft');
    disabled=true;await p.getByRole('button',{name:'Refresh state',exact:true}).click();await p.getByText(/disabled on this deployment/).waitFor();assert.equal(await p.getByRole('button',{name:'Capture action',exact:true}).isDisabled(),true);
    disabled=false;unauthorized=true;await p.getByRole('button',{name:'Refresh state',exact:true}).click();await p.getByText(/Your session has ended/).waitFor();
    assert.equal(await p.locator('#ledger').textContent(),'');
    assert.ok(!(await p.locator('#commitment-resolution').textContent()).includes('Retained draft'));
    assert.ok(!(await p.locator('#action-commitment').textContent()).includes('Reviewed draft'));
    assert.equal(await p.getByLabel('Resolution title',{exact:true}).inputValue(),'');
    assert.equal(await p.getByRole('link',{name:'Sign in again',exact:true}).isVisible(),true);
    assert.deepEqual(pageErrors,[]);
  }finally{await p.close();}
});

test('Resolution Control evidence is revision-bound, threats block finish and reopening invalidates attestations',async()=>{
  reset();pageErrors.length=0;const p=await page();
  try {
    await outcome(p);await commitment(p);
    await p.getByLabel('One pair per line: item_id | dimension').fill('metric_axioms | written');
    p.once('dialog',dialog=>dialog.accept());
    await p.getByRole('button',{name:'Replace identified scope'}).click();await saved(p);
    assert.equal(view.workspace.commitments[0].readiness.items.length,1);
    assert.equal(await p.getByRole('button',{name:/Confirm verified readiness/}).isDisabled(),true);

    await p.getByLabel('Existing material reference').fill('https://example.invalid/source');
    await p.getByRole('button',{name:'Record material'}).click();await saved(p);
    assert.match(await p.locator('#rc-coverage').textContent(),/mapped/);
    await p.getByLabel('Material stage').selectOption('deploy');
    await p.getByLabel('Existing material reference').fill('reviewed-notes.tex');
    await p.getByRole('button',{name:'Record material'}).click();await saved(p);
    await p.getByLabel('Test / verification method').fill('Try an edge case');
    await p.getByLabel('Existing evidence reference').fill('test-case-1');
    await p.getByRole('button',{name:'Record evidence'}).click();await saved(p);
    await p.getByLabel('Evidence stage').selectOption('verification');
    await p.getByLabel('Test / verification method').fill('Inspect each logical implication');
    await p.getByLabel('Existing evidence reference').fill('verification.tex');
    await p.getByRole('button',{name:'Record evidence'}).click();await saved(p);
    assert.equal(await p.getByRole('button',{name:/Confirm verified readiness/}).isDisabled(),false);

    await p.getByLabel('Observed threat / obstacle').fill('<img src=x onerror=alert(1)>');
    await p.getByLabel('Blocks verified readiness').check();
    await p.getByRole('button',{name:'Capture threat'}).click();await saved(p);
    assert.equal(await p.locator('#rc-threats img').count(),0);
    assert.equal(await p.getByRole('button',{name:/Confirm verified readiness/}).isDisabled(),true);
    p.once('dialog',dialog=>dialog.accept());
    await p.getByRole('button',{name:'Resolve threat'}).click();await saved(p);
    assert.equal(view.workspace.commitments[0].readiness.blocking_threats,0);
    assert.match(await p.locator('#rc-coverage').textContent(),/stale/);

    await p.getByLabel('Evidence stage').selectOption('stress_test');
    await p.getByLabel('Test / verification method').fill('New revision stress test');
    await p.getByLabel('Existing evidence reference').fill('stress-after-threat');
    await p.getByRole('button',{name:'Record evidence'}).click();await saved(p);
    await p.getByLabel('Evidence stage').selectOption('verification');
    await p.getByLabel('Test / verification method').fill('Review current revision');
    await p.getByLabel('Existing evidence reference').fill('current-review.tex');
    await p.getByRole('button',{name:'Record evidence'}).click();await saved(p);
    p.once('dialog',dialog=>dialog.accept());
    await p.getByRole('button',{name:/Confirm verified readiness/}).click();await saved(p);
    assert.ok(view.workspace.commitments[0].readiness.verified_finish);
    assert.match(await p.locator('#rc-readiness-status').textContent(),/Human-confirmed/);

    p.once('dialog',dialog=>dialog.accept());
    await p.getByRole('button',{name:'Reopen verified readiness'}).click();await saved(p);
    assert.equal(view.workspace.commitments[0].readiness.verified_finish,null);
    assert.equal(await p.getByRole('button',{name:/Confirm verified readiness/}).isDisabled(),true);
    assert.ok(writes.every(packet=>!('owner' in packet)&&!('actor' in packet.command)));
    if(process.env.PLAYGROUND_SCREENSHOT_DIR)await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-evidence-desktop.png'),fullPage:true});
    await p.setViewportSize({width:390,height:844});
    assert.equal(await p.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);
    if(process.env.PLAYGROUND_SCREENSHOT_DIR)await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-evidence-mobile.png'),fullPage:true});
    assert.deepEqual(pageErrors,[]);
  }finally{await p.close();}
});

test('Resolution Control never rebases a draft from an empty generation across an external reset',async()=>{
  reset();pageErrors.length=0;const p=await page();
  try {
    await p.getByLabel('Resolution title',{exact:true}).fill('Preserve original generation');
    view.generation='replacement-generation';view.revision=0;
    await p.getByRole('button',{name:'Capture resolution',exact:true}).click();
    await p.getByText(/Another edit or recovery/).waitFor();
    assert.equal(writes.length,1);
    assert.equal(writes[0].meta.generation,null);
    assert.equal(writes[0].meta.expected_revision,0);
    assert.equal(view.workspace.resolutions.length,0);
    assert.equal(await p.getByLabel('Resolution title',{exact:true}).inputValue(),'Preserve original generation');
    assert.deepEqual(pageErrors,[]);
  }finally{await p.close();}
});

test('Deadline overview shows every date once, orders SAST times, navigates to details and clears on session loss',async()=>{
  reset();pageErrors.length=0;
  view.observed_at='2026-10-10T19:00:00Z'; // 21:00 South Africa
  view.workspace.resolutions=[{id:'outcome-1',spec:{title:'Schedule check',objective:null,client_reference:null}}];
  const item=(id,title,deadline,area='CS244')=>({
    id,resolution:'outcome-1',spec:{title,area,kind:'assessment',source:null},
    schedule:{deadline,earliest_finish:null,buffer_minutes:null},
    readiness:{scope_revision:0,scope_identified:false,items:[],blocking_threats:0,verified_finish:null}
  });
  const date=value=>({precision:'date_only',value});
  const at=value=>({precision:'instant',value});
  view.workspace.commitments=[
    item('after', 'Later assessment',date('2026-10-22')),
    item('today', 'Date-only no midnight',date('2026-10-10')),
    item('midnight', 'Next calendar day',at('2026-10-10T22:05:00Z')),
    item('earlier', '<img src=x onerror=alert(1)>',at('2026-10-11T12:00:00Z')),
    item('later', 'Following submission',at('2026-10-11T15:00:00Z')),
    item('missing', 'Undated task',null),
    item('expired','Prior deadline',date('2026-10-09')),
    item('past-hour','Expired exact time',at('2026-10-10T17:30:00Z'))
  ];
  const p=await page();
  try{
    const overview=p.locator('#deadline-overview');
    assert.equal(await overview.locator('.rc-deadline-item').count(),8);
    assert.match(await overview.textContent(),/8 recorded obligations/);
    const titles=await overview.locator('.rc-deadline-group h3').allTextContents();
    assert.deepEqual(titles,['Overdue · 2','Today · 1','Next 7 days · 3','Later · 1','Date unknown · 1']);
    assert.match(await overview.textContent(),/Time not specified/);
    assert.match(await overview.textContent(),/00:05 SAST/);
    assert.equal(await overview.locator('img').count(),0);
    const upcoming=await overview.locator('[data-urgency="soon"] .rc-deadline-info strong').allTextContents();
    assert.deepEqual(upcoming,['Next calendar day','<img src=x onerror=alert(1)>','Following submission']);
    await p.getByRole('button',{name:'View obligation: Following submission'}).click();
    assert.equal(await p.evaluate(()=>document.activeElement?.dataset.commitment),'later');
    await p.setViewportSize({width:390,height:844});
    assert.equal(await p.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);
    if(process.env.PLAYGROUND_SCREENSHOT_DIR) {
      await mkdir(process.env.PLAYGROUND_SCREENSHOT_DIR,{recursive:true});
      await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-deadline-overview-mobile.png'),fullPage:true});
    }
    unauthorized=true;
    await p.getByRole('button',{name:'Refresh state',exact:true}).click();
    await p.getByText(/Your session has ended/).waitFor();
    assert.equal(await overview.textContent(),'');
    assert.deepEqual(pageErrors,[]);
  }finally{await p.close();}
});
