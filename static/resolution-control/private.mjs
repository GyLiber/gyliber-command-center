// Private history, reporting and recovery; do not persist any uploaded/downloaded payload in storage.
const $ = id => document.getElementById(id);
const el = (tag,value,cls) => {const n=document.createElement(tag);if(value!==undefined&&value!==null)n.textContent=String(value);if(cls)n.className=cls;return n;};
const maxUpload = 4*1024*1024;
export const deletionPhrase = 'DELETE MY RESOLUTION CONTROL DATA';
export function reportQuery(date,offset,cutoff) {
  if(!/^\d{4}-\d{2}-\d{2}$/.test(date)||!Number.isFinite(Date.parse(date+'T00:00:00Z'))||
     new Date(date+'T00:00:00Z').toISOString().slice(0,10)!==date)throw new Error('Choose a valid report date.');
  if(!/^-?\d+$/.test(String(offset))||Number(offset)<-840||Number(offset)>840)throw new Error('Explicit UTC offset must be a whole number of minutes between -840 and +840.');
  if(cutoff!==''&&!/^(0|[1-9]\d*)$/.test(String(cutoff)))throw new Error('Report cutoff must be a nonnegative revision or blank.');
  const query=new URLSearchParams({date,offset_minutes:String(offset)});
  if(cutoff!=='')query.set('cutoff_revision',String(cutoff));
  return '/report?'+query.toString();
}
export function backupShape(value) {
  return value && !Array.isArray(value) && value.schema_version===1 &&
    typeof value.source_generation==='string' && Array.isArray(value.lineage) &&
    Array.isArray(value.events) && value.events.length<=2048 && typeof value.checksum==='string' &&
    /^[a-f0-9]{64}$/.test(value.checksum) && typeof value.exported_at==='string';
}
export function metadataShape(value) {
  return value && !Array.isArray(value) && value.schema_version===1 &&
    Array.isArray(value.deleted_generations) && value.deleted_generations.length<=4096 &&
    typeof value.checksum==='string' && /^[a-f0-9]{64}$/.test(value.checksum);
}
export function eventPage(events,after) {
  if(!Array.isArray(events)||events.length>100)throw new Error('History response exceeded the page limit.');
  let last=after;
  for(const entry of events) {
    if(!Number.isSafeInteger(entry.revision)||entry.revision<=last||typeof entry.at!=='string'||typeof entry.origin!=='string'||!entry.command?.type)
      throw new Error('History response is not a strictly increasing event page.');
    last=entry.revision;
  }
  return last;
}
async function fileJson(file,shape) {
  if(!file||file.size>maxUpload||file.size===0)throw new Error('Choose a nonempty JSON file under 4 MiB.');
  let result;try{result=JSON.parse(await file.text());}catch{throw new Error('Invalid JSON file.');}
  if(!shape(result))throw new Error('File is not the expected versioned recovery schema.');
  return result;
}
function downloadJson(value,name) {
  const data=new Blob([JSON.stringify(value,null,2)+'\n'],{type:'application/json'});
  const url=URL.createObjectURL(data);const link=el('a');
  link.href=url;link.download=name;document.body.append(link);link.click();link.remove();
  setTimeout(()=>URL.revokeObjectURL(url),30000);
}
function clearNode(id){$(id).replaceChildren();}
export function createPrivateUi({getSnapshot,getViewer,getCsrf,getAvailable,notice,refresh,sendRecovery,read}) {
  let entries=[],after=0,hasMore=false,historyRevision=null,historyGeneration=null;
  let report=null,reportGeneration=null,receipt=null,receiptOwner=null,receiptGeneration=null;
  let busy=false,selectedBackup=null,selectedMetadata=null;
  function blocked(){return !getAvailable()||busy;}
  function controls() {
    for(const id of ['rc-history-refresh','rc-report-form','rc-export','rc-ledger-download','rc-restore','rc-purge']) {
      const n=$(id); if(n.tagName==='FORM')n.querySelector('fieldset').disabled=blocked();else n.disabled=blocked();
    }
    $('rc-history-more').disabled=blocked()||!hasMore;
    $('rc-report-download').disabled=blocked()||!report||reportGeneration!==getSnapshot()?.generation;
    $('rc-receipt-download').disabled=blocked()||!receipt||receiptOwner!==getViewer()||receiptGeneration!==getSnapshot()?.generation;
  }
  function resetPrivate(){
    entries=[];after=0;hasMore=false;historyRevision=null;historyGeneration=null;
    report=null;reportGeneration=null;receipt=null;receiptOwner=null;receiptGeneration=null;selectedBackup=null;selectedMetadata=null;busy=false;
    for(const id of ['rc-history-list','rc-report-view','rc-recovery-status'])clearNode(id);
    for(const id of ['rc-backup-file','rc-metadata-file'])$(id).value='';
    $('rc-purge-phrase').value='';$('rc-restore-confirm').checked=false;
    $('rc-report-cutoff').value='';$('rc-history-note').textContent='History not loaded.';
    controls();
  }
  function onSnapshot(previous,next,ownerChanged=false) {
    if(ownerChanged || (previous && previous.generation!==next.generation)) {
      // Keep the newly acknowledged purge receipt for a second download.
      const newest=receiptOwner===getViewer()&&receiptGeneration===next.generation?receipt:null;
      resetPrivate();
      if(newest){receipt=newest;receiptOwner=getViewer();receiptGeneration=next.generation;}
    } else if(previous && previous.revision!==next.revision) {
      // Never offer a report or paged event list as if it reflected newer writes.
      report=null;reportGeneration=null;entries=[];after=0;hasMore=false;
      historyRevision=null;historyGeneration=null;
      clearNode('rc-report-view');clearNode('rc-history-list');
      $('rc-history-note').textContent='Workspace changed. Load history again.';
    }
    $('rc-workspace-revision').textContent=`Workspace revision ${next.revision}; report times use an explicit fixed UTC offset, not automatic DST.`;
    if(!$('rc-report-date').value) {
      const stamp=Date.parse(next.observed_at);
      if(Number.isFinite(stamp))$('rc-report-date').value=new Date(stamp+120*60000).toISOString().slice(0,10);
    }
    controls();
  }
  function validRead(){if(blocked())throw new Error('Refresh the authenticated workspace before reading private records.');}
  async function withRead(action,label) {
    if(busy||!getAvailable()){notice('Refresh the private workspace before requesting '+label+'.');return;}
    busy=true;controls();notice('Reading private '+label+'…');
    try {await action();notice(label+' ready. No changes were made.');}
    catch(e) {if(e.code==='authentication_required'||e.code==='reauthentication_required')resetPrivate();notice(e.message||'Private read failed; refresh before continuing.');}
    finally{busy=false;controls();}
  }
  function renderHistory() {
    const list=$('rc-history-list');list.replaceChildren();
    if(!entries.length)list.append(el('p','No recorded changes in this workspace.','muted'));
    for(const e of entries){
      const row=el('article',null,'rc-scope-card');
      row.append(el('strong',`Revision ${e.revision} · ${e.command.type.replaceAll('_',' ')}`),
        el('p',`${e.at} · ${e.origin==='imported'?'Imported claim':'Recorded event'}`,'muted'));
      list.append(row);
    }
    $('rc-history-note').textContent=`${entries.length} events shown. Pages contain at most 100. ${hasMore?'More may be available.':'No further recorded events at the observed revision.'}`;
  }
  async function history(first=false) {
    const snapshot=getSnapshot();if(!snapshot)throw new Error('Workspace not loaded.');
    const isNew=first||historyGeneration!==snapshot.generation||historyRevision!==snapshot.revision;
    const cursor=isNew?0:after;
    const data=await read('/history?after_revision='+cursor);
    const last=eventPage(data.events,cursor);
    if(getSnapshot()?.generation!==snapshot.generation||getSnapshot()?.revision!==snapshot.revision)throw new Error('Workspace changed while reading history. Refresh and retry.');
    if(isNew){entries=[];after=0;}
    entries.push(...data.events);after=last;
    historyGeneration=snapshot.generation;historyRevision=snapshot.revision;
    hasMore=data.events.length===100&&after<snapshot.revision;
    renderHistory();
  }
  async function reportLoad() {
    const form=$('rc-report-form');
    const route=reportQuery(form.elements.date.value,form.elements.offset.value,form.elements.cutoff.value);
    const observed=getSnapshot();
    const data=await read(route);
    if(!Number.isSafeInteger(data.cutoff_revision)||!Array.isArray(data.changes)||!data.state?.commitments||!data.state?.actions||typeof data.empty_day!=='boolean')
      throw new Error('Unexpected report response.');
    if(getSnapshot()?.generation!==observed.generation||getSnapshot()?.revision!==observed.revision)throw new Error('Workspace changed during report generation.');
    report=data;reportGeneration=observed.generation;
    const panel=$('rc-report-view');panel.replaceChildren();
    panel.append(el('p',`Date ${data.date} · explicit UTC offset ${data.offset_minutes} minutes · revision cutoff ${data.cutoff_revision} · snapshot ${data.state_at}`));
    if(data.imported_history)panel.append(el('p','Imported history is a user-supplied claim, not independent evidence.','warning'));
    if(data.empty_day)panel.append(el('p','No recorded changes on the selected local day.'));
    else {
      const section=el('div',null,'rc-scope-list');
      for(const c of data.changes) {
        const card=el('article',null,'rc-scope-card');
        card.append(el('strong',`Revision ${c.revision}: ${c.code}`),
          el('p',`${c.at} · ${c.origin==='imported'?'Imported claim':'Recorded event'}`,'muted'));
        section.append(card);
      }
      panel.append(section);
    }
    panel.append(el('h4','Snapshot: outputs and blockers'));
    for(const a of data.state.actions.filter(x=>x.action?.status==='completed'))
      panel.append(el('p',`Recorded completion: ${a.action.plan?.instruction||'Action'}; output: ${a.action.artifact?.value||'Reference unavailable'}`,'instruction'));
    for(const t of data.state.threats.filter(x=>!x.resolved))
      panel.append(el('p',`${t.spec?.blocking?'Blocking':'Unresolved'} threat: ${t.spec?.description||'Unknown'}`,'warning'));
    const incomplete=data.state.commitments.filter(c=>!c.readiness?.verified_finish).length;
    panel.append(el('p',`${incomplete} obligations without current verified finish in this snapshot. This is not proof certification.`));
  }
  async function downloadPair() {
    const observed=getSnapshot();
    // These are two independent, private documents. A second tab may mutate
    // between reads; users must retain the newest ledger after any deletion.
    const backup=await read('/export');
    if(!backupShape(backup))throw new Error('Unexpected backup format.');
    const metadata=await read('/recovery-metadata');
    if(!metadataShape(metadata))throw new Error('Unexpected deletion metadata.');
    if(getSnapshot()?.generation!==observed.generation||getSnapshot()?.revision!==observed.revision)throw new Error('Workspace changed; refresh and repeat both downloads.');
    downloadJson(backup,'resolution-control-backup.json');
    downloadJson(metadata,'resolution-control-recovery-metadata.json');
    $('rc-recovery-status').textContent='Backup and separate deletion metadata requested as downloads. Store privately; after every purge retain the newest ledger. A downloaded file alone is not a tested restore.';
  }
  async function restoreFromFiles() {
    const observed=getSnapshot();
    if(!observed||observed.revision!==0||observed.workspace.resolutions.length||observed.workspace.commitments.length||observed.workspace.actions.length||observed.workspace.threats.length)
      throw new Error('Restore requires an empty workspace. Existing records must never be overwritten.');
    if(!$('rc-restore-confirm').checked)throw new Error('Confirm that the deletion metadata is the newest independent copy before restore.');
    const backup=await fileJson($('rc-backup-file').files[0],backupShape);
    const recovery_metadata=await fileJson($('rc-metadata-file').files[0],metadataShape);
    if(!confirm('Restore this user-supplied history into this empty workspace? It cannot prove its author or the source data and cannot bypass deletion barriers.'))return;
    sendRecovery('/restore',{backup,recovery_metadata,confirm_recovery_metadata:true},ack=>{
      selectedBackup=null;selectedMetadata=null;
      $('rc-recovery-status').textContent=`Restore acknowledged at revision ${ack.revision}. All imported events remain user-supplied claims.`;
      $('rc-backup-file').value='';$('rc-metadata-file').value='';$('rc-restore-confirm').checked=false;
    },{generation:observed.generation,revision:observed.revision});
  }
  function purgeWorkspace() {
    const observed=getSnapshot();
    if($('rc-purge-phrase').value!==deletionPhrase)throw new Error('Type the exact deletion phrase.');
    if(!confirm('Permanently purge this workspace’s private event history? Copies already downloaded or kept by the host are not deleted.'))return;
    sendRecovery('/purge',{confirmation:deletionPhrase},ack=>{
      if(!metadataShape(ack.recovery_metadata)) {
        notice('Purge accepted but deletion receipt missing. Fetch recovery metadata now; do not rely on older backups.');
      } else {
        // The receipt is retained only in this tab and is also downloaded,
        // even if the subsequent state refresh fails.
        receipt=ack.recovery_metadata;receiptOwner=getViewer();receiptGeneration=ack.generation;
        downloadJson(receipt,'resolution-control-purge-receipt.json');
        $('rc-recovery-status').textContent='Purge acknowledged. Download of the newest deletion receipt requested. Retain it privately; do not restore an older deleted backup.';
      }
      $('rc-purge-phrase').value='';
    },{generation:observed.generation,revision:observed.revision});
  }
  function setup() {
    $('rc-history-refresh').onclick=()=>withRead(()=>history(true),'history');
    $('rc-history-more').onclick=()=>withRead(()=>history(false),'history page');
    $('rc-report-form').onsubmit=e=>{e.preventDefault();withRead(reportLoad,'daily report');};
    $('rc-report-download').onclick=()=>{if(report&&reportGeneration===getSnapshot()?.generation)downloadJson(report,'resolution-control-daily-report.json');};
    $('rc-export').onclick=()=>withRead(downloadPair,'backup and independent deletion ledger');
    $('rc-ledger-download').onclick=()=>withRead(async()=>{
      const metadata=await read('/recovery-metadata');if(!metadataShape(metadata))throw new Error('Unexpected deletion metadata.');
      downloadJson(metadata,'resolution-control-recovery-metadata.json');
      $('rc-recovery-status').textContent='Latest deletion ledger download requested; keep private and update it after each purge.';
    },'latest deletion metadata');
    $('rc-receipt-download').onclick=()=>{if(receipt&&receiptOwner===getViewer()&&receiptGeneration===getSnapshot()?.generation)downloadJson(receipt,'resolution-control-purge-receipt.json');};
    $('rc-restore').onclick=async()=>{try{if(blocked())throw new Error('Refresh private state before recovery.');await restoreFromFiles();}catch(e){notice(e.message);}};
    $('rc-purge').onclick=()=>{try{if(blocked())throw new Error('Refresh private state before deletion.');purgeWorkspace();}catch(e){notice(e.message);}};
    controls();
  }
  setup();
  return {clear:resetPrivate,onSnapshot,controls};
}
