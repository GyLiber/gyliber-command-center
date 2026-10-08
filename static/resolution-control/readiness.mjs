// Resolution Control member evidence and readiness. No private payload is HTML.
import {text, reference, displayTime} from './model.mjs';

export const dimensions = ['written','oral','board','justification','research','practical','test_case'];
const idPattern = /^[A-Za-z0-9_-]{1,64}$/u;
export const keyLabel = key => `${key.item} · ${key.dimension.replaceAll('_',' ')}`;
const sameKey = (a,b) => a?.item===b?.item && a?.dimension===b?.dimension;

// A line is an explicit (item, applicable dimension) pair, not an inferred syllabus.
export function parseScope(source) {
  const lines = String(source).split(/\r?\n/u).map(s=>s.trim()).filter(Boolean);
  if(lines.length===0 || lines.length>128)throw new Error('Identify between 1 and 128 scope/dimension pairs, one per line.');
  const keys = [], seen = new Set();
  for(const line of lines) {
    const parts=line.split('|').map(s=>s.trim());
    if(parts.length!==2 || !idPattern.test(parts[0]) || !dimensions.includes(parts[1]))throw new Error('Use item_id | dimension on each line. Dimensions: '+dimensions.join(', ')+'.');
    const signature=parts.join('|');
    if(seen.has(signature))throw new Error('Each item/dimension pair must be unique.');
    seen.add(signature);keys.push({item:parts[0],dimension:parts[1]});
  }
  return keys;
}
export const scopeText = readiness => (readiness?.items||[]).map(x=>`${x.key.item} | ${x.key.dimension}`).join('\n');

export function coverage(item,revision) {
  if(item.stress_test?.scope_revision===revision && item.stress_test.outcome==='fail')return 'stress_test_failed';
  if(item.verification?.scope_revision===revision && item.verification.outcome==='fail')return 'verification_failed';
  if(item.stress_test?.scope_revision===revision && item.stress_test.outcome==='pass') {
    return item.verification?.scope_revision===revision && item.verification.outcome==='pass' ? 'verified' : 'stress_tested';
  }
  if(item.stress_test || item.verification)return 'stale';
  if(item.deployed)return 'deployed';
  if(item.mapped)return 'mapped';
  return 'identified';
}
export function canConfirm(readiness) {
  return !!readiness?.scope_identified && readiness.items?.length>0 && readiness.blocking_threats===0
    && !readiness.verified_finish && readiness.items.every(item=>coverage(item,readiness.scope_revision)==='verified');
}

const $ = id => document.getElementById(id);
const elt = (tag,value,cls) => {const n=document.createElement(tag);if(value!=null)n.textContent=value;if(cls)n.className=cls;return n;};
const btn = (label,handler,eligible=true) => {
  const b=elt('button',label);b.type='button';b.dataset.write='true';b.dataset.eligible=String(eligible);
  b.disabled=!eligible;b.addEventListener('click',handler);return b;
};
const chosen = (id,options,empty) => {
  const select=$(id),previous=select.value;
  select.replaceChildren(new Option(empty,''));
  for(const [value,label] of options)select.add(new Option(label,value));
  if(options.some(([value])=>value===previous))select.value=previous;
  else if(options.length===1)select.value=options[0][0];
  return select.value;
};
const recordFor = (ws,id) => ws.commitments.find(c=>c.id===id);
function baseNow(snapshot) {return {generation:snapshot.generation,revision:snapshot.revision};}

// Explicit save semantics: do not rebase a dirty form on Refresh. The main
// controller owns idempotency, CSRF, pending/uncertain writes and acknowledgements.
export function createReadinessUi({getSnapshot,save,notice,referenceNode}) {
  const forms=['scope','material','evidence','threat'];
  let selected=null;
  const markDraft=form=>{if(!form.dataset.base)form.dataset.base=JSON.stringify(baseNow(getSnapshot()));};
  for(const name of forms) {
    const form=$(`rc-${name}-form`);
    form.addEventListener('input',()=>markDraft(form));
    form.addEventListener('change',()=>markDraft(form));
  }
  function resetForm(name,scope=false) {
    const form=$(`rc-${name}-form`);delete form.dataset.base;
    if(name==='scope') {if(scope)form.elements.scope.value=scopeText(recordFor(getSnapshot()?.workspace||{commitments:[]},selected)?.readiness);}
    else if(name==='material')form.elements.artifact.value='';
    else if(name==='evidence'){form.elements.method.value='';form.elements.artifact.value='';}
    else if(name==='threat'){form.elements.description.value='';form.elements.resolution_path.value='';form.elements.blocking.checked=false;}
  }
  function clear() {
    selected=null;
    for(const name of forms){const form=$(`rc-${name}-form`);form.reset();delete form.dataset.base;}
    $('rc-commitment').replaceChildren(new Option('No private state',''));
    for(const id of ['rc-coverage','rc-threats','rc-readiness-status'])$(id).replaceChildren();
  }
  function submit(command,name) {
    const form=$(`rc-${name}-form`);
    const base=form.dataset.base?JSON.parse(form.dataset.base):baseNow(getSnapshot());
    save(command,()=>resetForm(name,true),base);
  }
  function draw(ws) {
    const id=chosen('rc-commitment',ws.commitments.map(c=>[c.id,c.spec.title]),'Choose an obligation');
    if(id!==selected) {
      selected=id;
      for(const name of forms)resetForm(name,true);
    }
    const commitment=recordFor(ws,id),r=commitment?.readiness,area=$('rc-coverage'),threats=$('rc-threats'),status=$('rc-readiness-status');
    area.replaceChildren();threats.replaceChildren();status.replaceChildren();
    if(!r){status.append(elt('p','Choose an obligation to review its applicable scope and evidence.'));return;}
    if(!$('rc-scope-form').dataset.base)$('rc-scope-form').elements.scope.value=scopeText(r);
    status.append(elt('p',r.verified_finish
      ? 'Human-confirmed verified finish is recorded. Later changes require renewed verification.'
      : r.scope_identified ? `Scope revision ${r.scope_revision} · ${r.items.length} pairs · ${r.blocking_threats} blocking threats. No verified finish yet.`
      : 'Scope unknown. Do not infer preparation or readiness from completed actions.'));
    if(r.verified_finish) {
      status.append(btn('Reopen verified readiness',()=>{if(confirm('Reopen readiness? Recorded evidence remains in history, but fresh tests will be required.'))save({type:'reopen_readiness',commitment:id});}));
    } else {
      status.append(btn('Confirm verified readiness (human attestation)',()=>{if(confirm('Confirm all listed dimensions have current passing recorded evidence? This is not proof certification.'))save({type:'confirm_readiness',commitment:id});},canConfirm(r)));
    }
    const items=r.items||[];
    if(!r.scope_identified)area.append(elt('p','Scope not identified; previous material cannot establish current readiness.','warning'));
    const list=elt('div',null,'rc-scope-list');
    for(const item of items) {
      const card=elt('article',null,'rc-scope-card');
      const stage=coverage(item,r.scope_revision);
      card.append(elt('strong',keyLabel(item.key)),elt('p',`Coverage: ${stage.replaceAll('_',' ')}`));
      const dl=elt('dl');
      for(const [label,ref] of [['Mapped material',item.mapped],['Deployed material',item.deployed]]) {
        dl.append(elt('dt',label));const dd=elt('dd');dd.append(ref?referenceNode(ref):elt('span','Unknown'));dl.append(dd);
      }
      for(const [label,evidence] of [['Stress test',item.stress_test],['Verification',item.verification]]) {
        dl.append(elt('dt',label));const dd=elt('dd');
        dd.textContent=evidence?`${evidence.outcome} · scope revision ${evidence.scope_revision} · ${evidence.method} · recorded by ${evidence.actor} at ${displayTime({precision:'instant',value:evidence.recorded_at})} · human-attested, not machine-certified`:'Not recorded';
        if(evidence){dd.append(elt('br'));dd.append(referenceNode(evidence.artifact));}
        dl.append(dd);
      }
      card.append(dl);list.append(card);
    }
    area.append(list);
    const opts=items.map(x=>[x.key.item+'|'+x.key.dimension,keyLabel(x.key)]);
    chosen('rc-material-key',opts,'Choose scope pair');
    chosen('rc-evidence-key',opts,'Choose scope pair');
    const existing=ws.threats.filter(t=>t.commitment===id);
    if(!existing.length)threats.append(elt('p','No recorded threats. This does not establish that risks have been assessed.'));
    for(const t of existing) {
      const card=elt('article',null,'rc-scope-card');
      card.append(elt('strong',t.resolved?'Resolved threat':t.spec.blocking?'Blocking threat':'Nonblocking threat'),
        elt('p',t.spec.description,'instruction'),elt('p',`Resolution path: ${t.spec.resolution_path??'Unknown'}`));
      if(!t.resolved)card.append(btn('Resolve threat',()=>{if(confirm('Mark this threat resolved?'))save({type:'resolve_threat',id:t.id});}));
      threats.append(card);
    }
    for(const form of forms)$(`rc-${form}-form`).querySelector('fieldset').disabled=false;
  }

  $('rc-commitment').onchange=()=>{
    const changed=$('rc-commitment').value!==selected;
    if(changed && forms.some(name=>$('rc-'+name+'-form').dataset.base)) {
      if(!confirm('Switch obligation and discard the unsaved evidence drafts?')){$('rc-commitment').value=selected??'';return;}
    }
    draw(getSnapshot().workspace);
  };
  $('rc-reload-scope').onclick=()=>{
    if($('rc-scope-form').dataset.base && !confirm('Discard the current scope draft and reload the observed server scope?'))return;
    resetForm('scope',true);
    notice('Scope input reset to the last observed server version. No changes were saved.');
  };
  $('rc-scope-form').onsubmit=e=>{
    e.preventDefault();try{if(!selected)throw new Error('Choose an obligation first.');
      const items=parseScope(e.target.elements.scope.value);
      if(!confirm('Replace the identified scope? All previous verification will become stale.'))return;
      submit({type:'identify_scope',commitment:selected,items},'scope');
    }catch(err){notice(err.message);}
  };
  $('rc-scope-unknown').onclick=()=>{
    if(!selected)return;
    if(confirm('Mark scope unknown? Existing preparation can no longer establish readiness.'))save({type:'mark_scope_unknown',commitment:selected});
  };
  const picked = (name,ws) => {
    const [item,dimension] = $(`rc-${name}-key`).value.split('|');
    const key={item,dimension},r=recordFor(ws,selected)?.readiness;
    if(!r?.scope_identified || !r.items.some(x=>sameKey(x.key,key)))throw new Error('Choose an identified scope pair.');
    return {key,item:r.items.find(x=>sameKey(x.key,key)),r};
  };
  $('rc-material-form').onsubmit=e=>{
    e.preventDefault();try{
      const {key,item}=picked('material',getSnapshot().workspace),data=new FormData(e.target),
        artifact=reference(data.get('artifact')),phase=data.get('phase');
      if(!artifact)throw new Error('Provide an existing material reference.');
      if(phase==='deploy' && !item.mapped)throw new Error('Map material before deployment.');
      submit({type:phase==='deploy'?'deploy_material':'map_material',commitment:selected,key,artifact},'material');
    }catch(err){notice(err.message);}
  };
  $('rc-evidence-form').onsubmit=e=>{
    e.preventDefault();try{
      const {key,item,r}=picked('evidence',getSnapshot().workspace),data=new FormData(e.target),
        stage=data.get('stage'),outcome=data.get('outcome'),method=text(data.get('method'),2048,true),artifact=reference(data.get('artifact'));
      if(!artifact)throw new Error('Provide an evidence artifact reference.');
      if(!item.deployed)throw new Error('Deploy material before recording evidence.');
      if(stage==='verification' && outcome==='pass' && !['stress_tested','verified'].includes(coverage(item,r.scope_revision)))throw new Error('Pass the current stress test before recording passing verification.');
      submit({type:'record_evidence',commitment:selected,input:{key,stage,outcome,method,artifact}},'evidence');
    }catch(err){notice(err.message);}
  };
  $('rc-threat-form').onsubmit=e=>{
    e.preventDefault();try{if(!selected)throw new Error('Choose an obligation first.');
      const data=new FormData(e.target),spec={description:text(data.get('description'),2048,true),blocking:data.get('blocking')==='on',resolution_path:text(data.get('resolution_path'),2048)};
      submit({type:'add_threat',id:crypto.randomUUID(),commitment:selected,spec},'threat');
    }catch(err){notice(err.message);}
  };
  return {render:draw,clear};
}
