import test,{after} from 'node:test';
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {readFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {chromium} from 'playwright';

const root=fileURLToPath(new URL('../../',import.meta.url)), hash='a'.repeat(64);
let state,ops,requests,auth,broken,downloads,purgeCount,deleted;
const empty=()=>({resolutions:[],commitments:[],actions:[],threats:[],current_action:null});
function reset() {
  state={generation:null,revision:0,observed_at:'2026-10-09T06:00:00Z',workspace:empty(),buffers:[],imported_history:false};
  ops=new Map();requests=[];auth=true;broken=false;downloads=[];purgeCount=0;deleted=[];
}
reset();
function ledger(){return {schema_version:1,deleted_generations:[...deleted],checksum:hash};}
function backup(){return {schema_version:1,source_generation:state.generation||'root',
  lineage:['root'],exported_at:state.observed_at,events:state.workspace.resolutions.map((r,i)=>({revision:i+1,at:state.observed_at,origin:'live',command:{type:'create_resolution',id:r.id,spec:r.spec}})),checksum:hash};}
const staticFiles={
  '/command/resolution-control':'static/resolution-control.html',
  '/static/app.css':'static/app.css',
  '/static/resolution-control/style.css':'static/resolution-control/style.css',
  '/static/resolution-control/app.mjs':'static/resolution-control/app.mjs',
  '/static/resolution-control/model.mjs':'static/resolution-control/model.mjs',
  '/static/resolution-control/deadlines.mjs':'static/resolution-control/deadlines.mjs',
  '/static/resolution-control/readiness.mjs':'static/resolution-control/readiness.mjs',
  '/static/resolution-control/private.mjs':'static/resolution-control/private.mjs'
};
const server=createServer(async(req,res)=>{
  const url=new URL(req.url,'http://localhost'),path=url.pathname;
  const send=(status,data,type='application/json')=>{
    res.writeHead(status,{'Content-Type':type,'Cache-Control':'no-store','Content-Security-Policy':"default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'"});
    res.end(typeof data==='string'||Buffer.isBuffer(data)?data:JSON.stringify(data));
  };
  try{
    if(staticFiles[path])return send(200,await readFile(resolve(root,staticFiles[path])),path.endsWith('.mjs')?'text/javascript':path.endsWith('.css')?'text/css':'text/html');
    if(!path.startsWith('/api/resolution-control'))return send(404,{});
    if(!auth)return send(401,{error:'authentication_required'});
    if(path==='/api/resolution-control')return send(200,{viewer:'github-1',csrf:'test-csrf',view:state,limits:{events:2048},import_notice:'Imported history is a user-supplied claim.'});
    if(path.endsWith('/history')){
      const after=Number(url.searchParams.get('after_revision'))||0;
      const events=state.workspace.resolutions.map((r,i)=>({revision:i+1,at:state.observed_at,origin:state.imported_history?'imported':'live',command:{type:'create_resolution',id:r.id,spec:r.spec}})).filter(e=>e.revision>after).slice(0,100);
      return send(200,{events});
    }
    if(path.endsWith('/report')){
      const date=url.searchParams.get('date'),offset=Number(url.searchParams.get('offset_minutes')),cutoff=url.searchParams.has('cutoff_revision')?Number(url.searchParams.get('cutoff_revision')):state.revision;
      if(!/^\d{4}-\d\d-\d\d$/.test(date)||Math.abs(offset)>840||cutoff>state.revision)return send(400,{error:'invalid_request'});
      const live=state.workspace.resolutions.slice(0,cutoff);
      const changes=date==='2026-10-09'?live.map((r,i)=>({revision:i+1,at:state.observed_at,origin:state.imported_history?'imported':'live',code:'resolution.created',detail:{type:'create_resolution',id:r.id,spec:r.spec}})):[];
      const stateAt=date==='2026-10-09'?state.observed_at:'2026-10-08T21:59:59Z';
      return send(200,{date,offset_minutes:offset,cutoff_revision:cutoff,state_at:stateAt,changes,state:{...empty(),resolutions:changes.length?live:[]},empty_day:changes.length===0,imported_history:state.imported_history&&changes.length>0});
    }
    if(path.endsWith('/export')){downloads.push('backup');return state.revision?send(200,backup()):send(404,{error:'not_found'});}
    if(path.endsWith('/recovery-metadata')){downloads.push('ledger');return send(200,ledger());}
    if(req.method==='POST'&&['/commands','/purge','/restore'].some(suffix=>path.endsWith(suffix))){
      assert.equal(req.headers['x-resolution-csrf'],'test-csrf');
      let body='';for await(const chunk of req)body+=chunk;
      const packet=JSON.parse(body);requests.push({path,body,packet});
      const prior=ops.get(packet.meta.operation_id);
      if(prior){assert.equal(body,prior.body);return send(200,{...prior.ack,replayed:true});}
      if(packet.meta.generation!==state.generation||packet.meta.expected_revision!==state.revision)return send(409,{error:'refresh_required'});
      if(path.endsWith('/commands')){
        if(packet.command.type!=='create_resolution')return send(400,{error:'invalid_request'});
        state.workspace.resolutions.push({id:packet.command.id,spec:packet.command.spec});
        state.generation='root';state.revision++;
      }else if(path.endsWith('/purge')){
        if(packet.confirmation!=='DELETE MY RESOLUTION CONTROL DATA')return send(400,{error:'confirmation_required'});
        purgeCount++;
        deleted.push(state.generation??'root');
        state.generation='generation-after-purge';state.workspace=empty();state.revision=0;state.imported_history=false;
      }else {
        if(state.revision||state.workspace.resolutions.length)return send(409,{error:'refresh_required'});
        if(!packet.confirm_recovery_metadata||packet.recovery_metadata?.schema_version!==1||packet.backup?.schema_version!==1)return send(400,{error:'invalid_backup'});
        if(deleted.includes(packet.backup.source_generation)||packet.recovery_metadata.deleted_generations.includes(packet.backup.source_generation))return send(410,{error:'deleted_generation'});
        state.workspace.resolutions=packet.backup.events.map(e=>({id:e.command.id,spec:e.command.spec}));
        state.revision=state.workspace.resolutions.length;state.generation='generation-after-restore';state.imported_history=true;
      }
      const ack={generation:state.generation,revision:state.revision,operation:path.endsWith('/purge')?'purge':path.endsWith('/restore')?'restore':'mutation',replayed:false,
        recovery_metadata:path.endsWith('/purge')?ledger():null};
      ops.set(packet.meta.operation_id,{body,ack});
      if(broken){broken=false;return send(200,'{"incomplete":');}
      return send(200,ack);
    }
    return send(404,{});
  }catch(error){send(500,{error:'fixture_failure',message:error.message});}
});
await new Promise(done=>server.listen(0,'127.0.0.1',done));
const base=`http://127.0.0.1:${server.address().port}`;
const browser=await chromium.launch({headless:true});after(async()=>{await browser.close();await new Promise(done=>server.close(done));});
const pageErrors=[];
async function page(){
 const p=await browser.newPage({viewport:{width:1280,height:900},acceptDownloads:true});
 p.on('pageerror',e=>pageErrors.push(e.message));
 await p.goto(base+'/command/resolution-control');
 await p.getByText('State refreshed. No changes were made.',{exact:true}).waitFor();
 return p;
}
async function capture(p,title='Synthetic sample'){
 await p.getByLabel('Resolution title',{exact:true}).fill(title);
 await p.getByRole('button',{name:'Capture resolution',exact:true}).click();
 await p.getByText('Saved and refreshed.',{exact:true}).waitFor();
}
async function downloadClicks(p,label,callback){
 const downloads=[];
 const handler=d=>downloads.push(d.suggestedFilename());
 p.on('download',handler);
 try{await callback();await p.waitForTimeout(200);return downloads;}finally{p.off('download',handler);}
}
test('private history, fixed-offset report, explicit cutoff, empty day and source-only rendering',async()=>{
 reset();pageErrors.length=0;const p=await page();
 try{
   await capture(p,'<img src=x onerror=alert(1)>');
   await p.getByRole('button',{name:'Load private history'}).click();
   await p.getByText('1 events shown.',{exact:false}).waitFor();
   assert.match(await p.locator('#rc-history-list').textContent(),/Revision 1/);
   await p.getByRole('button',{name:'Preview private daily report'}).click();
   await p.getByText('Date 2026-10-09',{exact:false}).waitFor();
   assert.match(await p.locator('#rc-report-view').textContent(),/revision cutoff 1/);
   assert.equal(await p.locator('#rc-report-view img').count(),0);
   const downloaded=await downloadClicks(p,'daily report',async()=>p.getByRole('button',{name:'Download displayed report JSON'}).click());
   assert.ok(downloaded.includes('resolution-control-daily-report.json'));
   await p.getByLabel('History cutoff revision',{exact:false}).fill('0');
   await p.getByRole('button',{name:'Preview private daily report'}).click();
   await p.getByText('No recorded changes on the selected local day.').waitFor();
   await p.getByLabel('Local report date').fill('2026-10-08');
   await p.getByLabel('History cutoff revision',{exact:false}).fill('');
   await p.getByRole('button',{name:'Preview private daily report'}).click();
   await p.getByText('No recorded changes on the selected local day.').waitFor();
   await p.setViewportSize({width:390,height:844});
   assert.equal(await p.evaluate(()=>document.documentElement.scrollWidth<=innerWidth),true);
   if(process.env.PLAYGROUND_SCREENSHOT_DIR){await mkdir(process.env.PLAYGROUND_SCREENSHOT_DIR,{recursive:true});await p.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'resolution-private-mobile.png'),fullPage:true});}
   assert.deepEqual(pageErrors,[]);
 }finally{await p.close();}
});
test('backup and newest ledger download, deliberate purge receipt, identical uncertain retry and deletion barrier',async()=>{
 reset();pageErrors.length=0;const p=await page();
 try{
   await capture(p);
   const downloads=await downloadClicks(p,'pair',async()=>{await p.getByRole('button',{name:'Download backup and deletion ledger'}).click();await p.getByText('backup and independent deletion ledger ready.',{exact:false}).waitFor();});
   assert.ok(downloads.includes('resolution-control-backup.json'));
   assert.ok(downloads.includes('resolution-control-recovery-metadata.json'));
   assert.deepEqual(downloads.filter(x=>x.endsWith('.json')).length,2);
   assert.deepEqual(downloads,downloads.slice().sort((a,b)=>a.includes('backup')?-1:1));
   assert.ok(downloads.length===2);
   await p.getByText('5. Deliberate full-workspace purge',{exact:true}).click();
   await p.getByLabel('Type DELETE MY RESOLUTION CONTROL DATA exactly').fill('DELETE MY RESOLUTION CONTROL DATA');
   broken=true;p.once('dialog',d=>d.accept());
   await p.getByRole('button',{name:'Purge private workspace'}).click();
   await p.getByText(/Save outcome is uncertain/).waitFor();
   assert.equal(purgeCount,1);
   assert.equal(await p.getByRole('button',{name:'Purge private workspace'}).isDisabled(),true);
   await p.getByRole('button',{name:'Retry the same request'}).click();
   await p.getByText(/earlier save was confirmed/).waitFor();
   assert.equal(purgeCount,1);
   const purgeRequests=requests.filter(x=>x.path.endsWith('/purge'));
   assert.equal(purgeRequests.length,2);
   assert.equal(purgeRequests[0].body,purgeRequests[1].body);
   assert.equal(await p.getByRole('button',{name:'Download retained purge receipt again'}).isDisabled(),false);
   assert.equal(state.revision,0);
   const staleBackup=backup();staleBackup.source_generation='root';
   await p.getByLabel('Versioned workflow backup JSON').setInputFiles({name:'old-backup.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify(staleBackup))});
   await p.getByLabel('Newest independent deletion ledger JSON').setInputFiles({name:'ledger.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify(ledger()))});
   await p.getByLabel(/I have checked this is the newest independently retained ledger/).check();
   p.once('dialog',d=>d.accept());
   await p.getByRole('button',{name:'Restore reviewed backup'}).click();
   await p.getByText(/request belongs to a replaced or deleted workspace|request was rejected|deleted/i).first().waitFor();
   assert.equal(state.revision,0);
   assert.deepEqual(pageErrors,[]);
 }finally{await p.close();}
});
test('restore into empty destination and session-loss clears uploaded and displayed private state',async()=>{
 reset();pageErrors.length=0;const p=await page();
 try{
   const r={id:'synthetic-one',spec:{title:'Recovered synthetic claim',objective:null,client_reference:null}};
   const restoreBackup={schema_version:1,source_generation:'old-source',lineage:['old-source'],exported_at:'2026-10-09T05:00:00Z',
     events:[{revision:1,at:'2026-10-09T05:00:00Z',origin:'live',command:{type:'create_resolution',id:r.id,spec:r.spec}}],checksum:hash};
   await p.getByLabel('Versioned workflow backup JSON').setInputFiles({name:'backup.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify(restoreBackup))});
   await p.getByLabel('Newest independent deletion ledger JSON').setInputFiles({name:'ledger.json',mimeType:'application/json',buffer:Buffer.from(JSON.stringify(ledger()))});
   await p.getByLabel(/I have checked this is the newest independently retained ledger/).check();
   p.once('dialog',d=>d.accept());
   await p.getByRole('button',{name:'Restore reviewed backup'}).click();
   await p.getByText('Saved and refreshed.',{exact:true}).waitFor();
   assert.equal(state.imported_history,true);
   assert.equal(state.workspace.resolutions[0].spec.title,'Recovered synthetic claim');
   assert.equal(await p.getByRole('button',{name:'Restore reviewed backup'}).isDisabled(),false);
   await p.getByLabel('Local report date').fill('2026-10-09');
   await p.getByRole('button',{name:'Preview private daily report'}).click();
   await p.getByText('Imported history is a user-supplied claim, not independent evidence.').waitFor();
   auth=false;
   await p.getByRole('button',{name:'Refresh state',exact:true}).click();
   await p.getByText(/Your session has ended/).waitFor();
   assert.equal(await p.locator('#rc-report-view').textContent(),'');
   assert.equal(await p.locator('#rc-history-list').textContent(),'');
   assert.equal(await p.getByRole('button',{name:'Download displayed report JSON'}).isDisabled(),true);
   assert.equal(await p.getByRole('link',{name:'Sign in again'}).isVisible(),true);
   assert.deepEqual(pageErrors,[]);
 }finally{await p.close();}
});
