import {text, reference, schedule, plan, displayTime, exactLocal, errorMessage} from './model.mjs';
import {createReadinessUi} from './readiness.mjs';
const $ = (id) => document.getElementById(id);
let snapshot = null, csrf = null, viewer = null, ready = false, busy = false, pending = null;
const forms = ['resolution','commitment','action'];
const completionDrafts=new Map();
let readinessUi=null;
function node(tag, value, className) { const n = document.createElement(tag); if(value != null)n.textContent=value; if(className)n.className=className; return n; }
function notice(value) { $('notice').textContent=value; }
function buttons() {
  for(const kind of forms)$(`${kind}-form`).querySelector('fieldset').disabled=!ready || busy || !!pending;
  for(const fieldset of document.querySelectorAll('.rc-readiness fieldset'))fieldset.disabled=!ready || busy || !!pending;
  for(const b of document.querySelectorAll('[data-write]'))b.disabled=!ready || busy || !!pending || b.dataset.eligible==='false';
  document.querySelector('.rc').setAttribute('aria-busy',String(busy));
  $('refresh').disabled=busy; $('retry').hidden=!pending; $('retry').disabled=busy || !csrf;
}
function button(label, command, eligible=true) {
  const b=node('button',label);b.type='button';b.dataset.write='true';b.dataset.eligible=String(eligible);
  b.addEventListener('click',()=>save(command));return b;
}
function referenceNode(ref) {
  if(ref?.kind==='web') {
    // Only an allowlisted, credential/query/fragment-free URL becomes a link.
    try { const checked=reference(ref.value);if(checked?.kind==='web') {const a=node('a',ref.value,'reference');a.href=checked.value;a.rel='noreferrer noopener';a.target='_blank';return a;} } catch { /* Render invalid references as plain text. */ }
  }
  return node('span',ref?.value ?? 'Not provided','reference');
}
function clearPrivate() {
  snapshot=null;csrf=null;pending=null;ready=false;completionDrafts.clear();readinessUi?.clear();
  $('current-content').replaceChildren(node('p','Private state is not available.'));
  $('ledger').replaceChildren();$('capacity').textContent='';$('observed').textContent='Not loaded';$('import-notice').hidden=true;
  for(const kind of forms)resetForm(kind);
  $('commitment-resolution').replaceChildren(new Option('Choose resolution',''));
  $('action-commitment').replaceChildren(new Option('Choose commitment',''));
}
async function fetchJson(path, options={}) {
  const controller=new AbortController(), timer=setTimeout(()=>controller.abort(),10000);
  try {
    const response=await fetch(`/api/resolution-control${path}`,{credentials:'same-origin',cache:'no-store',signal:controller.signal,...options});
    let data;try {data=await response.json();} catch {throw new Error('response');}
    return {response,data};
  } finally {clearTimeout(timer);}
}
async function load() {
  ready=false;buttons();
  const {response,data}=await fetchJson('');
  if(!response.ok) {
    if(response.status===401){clearPrivate();$('signin').hidden=false;}
    throw new Error(errorMessage(data.error));
  }
  if(!/^github-\d+$/.test(data.viewer) || !data.view?.workspace || !Array.isArray(data.view.workspace.actions) || !Number.isSafeInteger(data.view.revision) || typeof data.csrf!=='string')throw new Error('response');
  if(viewer && viewer!==data.viewer) {clearPrivate();notice('The signed-in account changed. Earlier drafts and displayed state were cleared.');}
  viewer=data.viewer;csrf=data.csrf;snapshot=data.view;ready=true;$('signin').hidden=true;
  $('observed').textContent=`Observed ${displayTime({precision:'instant',value:snapshot.observed_at})}`;
  $('import-notice').hidden=!snapshot.imported_history;
  $('import-notice').textContent=data.import_notice;
  $('capacity').textContent=`${snapshot.workspace.resolutions.length}/16 outcomes · ${snapshot.workspace.commitments.length}/64 obligations · ${snapshot.workspace.actions.length}/128 actions · ${snapshot.revision}/${data.limits.events} changes`;
  render();buttons();
}
function safeMessage(error) {return ['response','Failed to fetch','Load failed','fetch failed'].includes(error.message) || error.name==='AbortError' ? 'State could not be refreshed. The last displayed state may be stale; refresh before continuing.' : error.message;}
async function refresh() {
  if(busy)return;busy=true;buttons();notice('Refreshing private state…');
  try {await load();notice(pending ? 'State refreshed. An earlier save is still uncertain. Retry the same request; keep this tab open.' : 'State refreshed. No changes were made.');}
  catch(error){notice(safeMessage(error));}
  finally{busy=false;buttons();}
}
async function sendPending() {
  if(busy || !pending)return;busy=true;buttons();notice('Saving…');
  const attempt=pending;
  try {
    const {response,data}=await fetchJson('/commands',{method:'POST',headers:{'Content-Type':'application/json','x-resolution-csrf':csrf},body:attempt.body});
    if(!response.ok) {
      if(response.status>=500 && data.error!=='feature_disabled')throw new Error('uncertain');
      pending=null;
      if(response.status===401){clearPrivate();$('signin').hidden=false;}
      if(response.status===403 || data.error==='feature_disabled')ready=false;
      if(response.status===409 || response.status===410) {try{await load();}catch{ready=false;}}
      notice(errorMessage(data.error));return;
    }
    // Only a received acknowledgement establishes acceptance. Never update
    // displayed state optimistically or retry automatically.
    pending=null;const accepted=JSON.parse(attempt.body).command;if(["cancel_action","complete_action","reopen_action"].includes(accepted.type))completionDrafts.delete(accepted.id);attempt.saved?.();
    try {await load();if(!accepted.type.startsWith('create_'))$('current').focus();notice(data.replayed ? 'The earlier save was confirmed. No duplicate change was recorded.' : 'Saved and refreshed.');}
    catch {ready=false;notice('Save accepted, but current state could not be refreshed. Refresh before continuing.');}
  } catch {
    ready=false;notice('Save outcome is uncertain. Your draft is retained. Retry the same request; do not capture it again. Keep this tab open.');
  } finally {busy=false;buttons();}
}
function save(command, saved, base) {
  if(!ready || busy || pending)return;
  pending={body:JSON.stringify({meta:{operation_id:crypto.randomUUID(),generation:base ? base.generation : snapshot.generation,expected_revision:base ? base.revision : snapshot.revision},command}),saved};
  sendPending();
}
function refillSelect(select, records, label, empty) {
  const previous=select.value;select.replaceChildren(new Option(empty,''));
  for(const record of records)select.add(new Option(label(record),record.id));
  if(records.some(record=>record.id===previous))select.value=previous;
  else if(records.length===1)select.value=records[0].id;
}
function addFact(list, label, value) {list.append(node('dt',label), typeof value==='string' ? node('dd',value) : (()=>{const dd=node('dd');dd.append(value);return dd;})());}
function render() {
  const workspace=snapshot.workspace;
  refillSelect($('commitment-resolution'),workspace.resolutions,r=>r.spec.title,'Choose resolution');
  refillSelect($('action-commitment'),workspace.commitments,c=>c.spec.title,'Choose commitment');
  const current=workspace.actions.find(a=>a.id===workspace.current_action);
  const running=workspace.actions.find(a=>a.action.status==='in_progress');
  const focus=$('current-content');focus.replaceChildren();
  if(current) {
    const {action}=current;focus.append(node('h3',action.plan.instruction,'instruction'),node('p',`State: ${action.status.replaceAll('_',' ')}`));
    const info=node('dl');addFact(info,'Expected output',action.plan.expected_artifact ?? 'Unknown');addFact(info,'Verification method',action.plan.verification_method ?? 'Unknown');addFact(info,'Start reference',referenceNode(action.plan.start_reference));focus.append(info);
    const row=node('div',null,'button-row');
    if(['new','blocked'].includes(action.status)) {
      const executable=!!action.plan.expected_artifact && !!action.plan.verification_method;
      row.append(button('Mark ready',{type:'ready_action',id:current.id},executable));
      if(!executable)focus.append(node('p','Add the expected output and verification method using Edit plan before marking ready.','warning'));
    }
    if(action.status==='ready')row.append(button('Start action',{type:'start_action',id:current.id},!running));
    if(['new','ready','in_progress'].includes(action.status))row.append(button('Block action',{type:'block_action',id:current.id}));
    row.append(button('Cancel action',{type:'cancel_action',id:current.id}));
    if(['new','ready','blocked'].includes(action.status)){const edit=node('button','Edit plan');edit.type='button';edit.dataset.write='true';edit.onclick=()=>editForm('action',current);row.append(edit);}
    focus.append(row);
    if(action.status==='in_progress') {
      const form=node('form'), fieldset=node('fieldset'), input=node('input');input.id='completion-output';input.name='artifact';input.required=true;input.maxLength=2048;input.value=completionDrafts.get(current.id) ?? '';input.oninput=()=>completionDrafts.set(current.id,input.value);
      const label=node('label','Completed output reference');label.htmlFor=input.id;const submit=node('button','Record completed output');submit.type='submit';submit.dataset.write='true';
      fieldset.append(label,input,submit);form.append(fieldset);form.onsubmit=event=>{event.preventDefault();try{const artifact=reference(input.value);if(!artifact)throw new Error('Provide the completed output reference.');save({type:'complete_action',id:current.id,artifact});}catch(error){notice(error.message);}};focus.append(form);
    }
  } else focus.append(node('p','No action is selected. Choose an action from the ledger; nothing is selected automatically.'));
  if(running && running.id!==current?.id){focus.append(node('p',`An action is still running: ${running.action.plan.instruction}`,'warning'),button('Return to running action',{type:'select_action',id:running.id}));}
  const ledger=$('ledger');ledger.replaceChildren();
  if(!workspace.resolutions.length)ledger.append(node('p','Capture an intended outcome, then one obligation and one action.','muted'));
  for(const resolution of workspace.resolutions) {
    const group=node('section');group.append(node('h3',resolution.spec.title));
    if(resolution.spec.objective)group.append(node('p',resolution.spec.objective,'instruction'));
    const edit=node('button','Edit outcome');edit.type='button';edit.dataset.write='true';edit.onclick=()=>editForm('resolution',resolution);group.append(edit);
    const commitments=workspace.commitments.filter(c=>c.resolution===resolution.id);
    if(!commitments.length)group.append(node('p','No commitments captured yet.','muted'));
    for(const commitment of commitments) {
      const card=node('article',null,'obligation');card.dataset.commitment=commitment.id;card.append(node('h3',commitment.spec.title));
      const info=node('dl');addFact(info,'Area / kind',`${commitment.spec.area ?? 'Unknown'} · ${commitment.spec.kind}`);
      addFact(info,'Deadline',displayTime(commitment.schedule.deadline));addFact(info,'Earliest finish',displayTime(commitment.schedule.earliest_finish));
      addFact(info,'Chosen buffer',commitment.schedule.buffer_minutes===null ? 'Unknown' : `${commitment.schedule.buffer_minutes} minutes`);
      const buffer=snapshot.buffers.find(b=>b.commitment===commitment.id);
      addFact(info,'Buffer observation',buffer?.unfinished ? `${buffer.unfinished.state.replaceAll('_',' ')}${buffer.unfinished.deadline_reached ? ' · known deadline reached' : ''}` : 'Finish confirmation recorded (human attestation)');
      if(buffer?.finished){addFact(info,'Confirmed finish',displayTime({precision:'instant',value:buffer.finished.verified_at}));addFact(info,'Frozen actual buffer',buffer.finished.actual_buffer_seconds===null?'Unknown':`${buffer.finished.actual_buffer_seconds} seconds`);}
      addFact(info,'Source',referenceNode(commitment.spec.source));addFact(info,'Scope',commitment.readiness?.scope_identified?'Identified (coverage review still required)':'Unknown');addFact(info,'Blocking threats',String(commitment.readiness?.blocking_threats ?? 'Unknown'));addFact(info,'Readiness','Action outputs alone do not establish readiness.');card.append(info);
      const edit=node('button','Edit controls');edit.type='button';edit.dataset.write='true';edit.onclick=()=>editForm('commitment',commitment);card.append(edit);
      const actions=workspace.actions.filter(a=>a.commitment===commitment.id), details=node('details');details.append(node('summary',`${actions.length} action${actions.length===1?'':'s'}`));
      for(const record of actions) {
        const row=node('article',null,'action-record');row.dataset.action=record.id;row.append(node('p',record.action.plan.instruction,'instruction'),node('p',`State: ${record.action.status.replaceAll('_',' ')}`,'muted'));
        if(['completed','cancelled'].includes(record.action.status))row.append(button('Reopen action',{type:'reopen_action',id:record.id}));
        else row.append(button('Select action',{type:'select_action',id:record.id}));
        if(record.action.artifact)row.append(referenceNode(record.action.artifact));details.append(row);
      }
      card.append(details);group.append(card);
    }
    ledger.append(group);
  }
  readinessUi?.render(workspace);
}
function setPrecision(name, fact) {
  const select=$(`${name}-precision`),input=$(`${name}-value`);select.value=fact?.precision ?? 'unknown';input.type=select.value==='instant'?'datetime-local':'date';input.step='1';input.disabled=select.value==='unknown';input.value=!fact?'':fact.precision==='instant'?exactLocal(fact.value):fact.value;
}
function resetForm(kind) {
  const form=$(`${kind}-form`);form.reset();delete form.dataset.editId;delete form.dataset.base;delete form.dataset.dirty;
  form.querySelector('button[type=submit]').textContent=`Capture ${kind}`;form.querySelector('.cancel-edit').hidden=true;
  if(kind==='commitment'){setPrecision('deadline',null);setPrecision('earliest',null);$('commitment-resolution').disabled=false;}
  if(kind==='action')$('action-commitment').disabled=false;
}
function editForm(kind, record) {
  if(!ready || busy || pending)return;
  const form=$(`${kind}-form`);
  if((form.dataset.editId || form.dataset.dirty) && !confirm('Replace this editing draft with the latest displayed item for review?'))return;
  resetForm(kind);form.dataset.editId=record.id;form.dataset.base=JSON.stringify({generation:snapshot.generation,revision:snapshot.revision});
  if(kind==='resolution') {form.elements.title.value=record.spec.title;form.elements.objective.value=record.spec.objective ?? '';}
  if(kind==='commitment') {
    for(const key of ['title','area','kind'])form.elements[key].value=record.spec[key] ?? '';
    form.elements.resolution.value=record.resolution;form.elements.resolution.disabled=true;form.elements.source.value=record.spec.source?.value ?? '';
    form.elements.buffer.value=record.schedule.buffer_minutes ?? '';setPrecision('deadline',record.schedule.deadline);setPrecision('earliest',record.schedule.earliest_finish);
  }
  if(kind==='action') {form.elements.commitment.value=record.commitment;form.elements.commitment.disabled=true;for(const key of ['instruction','expected_artifact','verification_method'])form.elements[key].value=record.action.plan[key] ?? '';form.elements.start_reference.value=record.action.plan.start_reference?.value ?? '';}
  form.querySelector('button[type=submit]').textContent=`Save ${kind}`;form.querySelector('.cancel-edit').hidden=false;$(`${kind}-details`).open=true;
  form.querySelector('input:not(:disabled),textarea').focus();notice('Editing the displayed item. Refresh does not rebase this draft; reload the item after a conflict.');
}
function setup() {
  readinessUi=createReadinessUi({getSnapshot:()=>snapshot,save,notice,referenceNode});
  for(const kind of forms) {
    const form=$(`${kind}-form`);form.querySelector('.cancel-edit').onclick=()=>resetForm(kind);
    form.addEventListener('input',()=>{form.dataset.dirty='true';});
    form.addEventListener('change',()=>{form.dataset.dirty='true';});
    form.onsubmit=event=>{
      event.preventDefault();if(!ready || busy || pending)return;
      try {
        const data=new FormData(form), editing=!!form.dataset.editId,id=form.dataset.editId ?? crypto.randomUUID(),workspace=snapshot.workspace;
        let command;
        if(kind==='resolution')command={type:editing?'update_resolution':'create_resolution',id,spec:{title:text(data.get('title'),160,true),objective:text(data.get('objective'),2048),client_reference:editing?workspace.resolutions.find(r=>r.id===id)?.spec.client_reference ?? null:null}};
        if(kind==='commitment')command={type:editing?'update_commitment':'create_commitment',id,spec:{title:text(data.get('title'),160,true),area:text(data.get('area'),160),kind:data.get('kind'),source:reference(data.get('source'))},schedule:schedule(data),...(!editing?{resolution:data.get('resolution')}:{})};
        if(kind==='action')command={type:editing?'update_action':'create_action',id,plan:plan(data),...(!editing?{commitment:data.get('commitment')}:{})};
        if(!editing && ((kind==='commitment' && !command.resolution)||(kind==='action' && !command.commitment)))throw new Error('Choose the parent item first.');
        save(command,()=>resetForm(kind),editing?JSON.parse(form.dataset.base):null);
      }catch(error){notice(error.message);}
    };
  }
  for(const name of ['deadline','earliest'])$(`${name}-precision`).onchange=()=>{const precision=$(`${name}-precision`).value,input=$(`${name}-value`);input.type=precision==='instant'?'datetime-local':'date';input.step='1';input.disabled=precision==='unknown';input.value='';};
  $('refresh').onclick=refresh;$('retry').onclick=sendPending;refresh();
}
setup();
