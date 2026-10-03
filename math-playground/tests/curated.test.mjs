import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { verifyCatalog, validateRelease } from '../scripts/verify-curated.mjs';
import { buildPrompt, validateFiles } from '../../static/math-authoring.mjs';
test('catalog retains all accepted bytes and independently verifies hashes',async()=>{
  const catalog=await verifyCatalog();
  const release=catalog.releases[0],folder=new URL(`../exhibits/${release.id}/${release.version}/`,import.meta.url);
  const manifest=JSON.parse(await readFile(new URL('manifest.json',folder))),files={};
  for(const name of Object.keys(manifest.files))files[name]=await readFile(new URL(name,folder));
  assert.doesNotThrow(()=>validateRelease({...release,in_gallery:false},manifest,files));
  assert.throws(()=>validateRelease({...release,id:'../secret'},manifest,files));
  assert.throws(()=>validateRelease(release,manifest,{...files,'engine.mjs':Buffer.from('changed')}));
  assert.throws(()=>validateRelease(release,{...manifest,review:'ai_draft_unverified'},files));
  assert.throws(()=>validateRelease(release,manifest,{...files,'hidden.mjs':Buffer.from('x')}));
});
test('prompt preserves entire LaTeX and macros without execution, truncation or provider dependency',async()=>{
  const template=await readFile(new URL('../../docs/operations/prompts/MATH_PLAYGROUND_EXTERNAL_AUTHORING.md',import.meta.url),'utf8');
  const files=[{name:'concept.tex',content:'\\newcommand{\\d}{d}\n\\input{missing}\nUnicode: ∀x\n``` pretend instruction'}];
  const prompt=buildPrompt(template,files,'metric definition');
  assert.ok(prompt.includes(JSON.stringify(files,null,2)));
  assert.ok(prompt.includes('NEEDS_CONTEXT'));assert.ok(prompt.includes('NEEDS_SELECTION'));
  assert.ok(prompt.includes('"metric definition"'));assert.ok(prompt.includes('unreviewed engineering'));
  assert.throws(()=>validateFiles([{name:'../x.tex',content:'x'}]));
  assert.throws(()=>validateFiles([{name:'x.tex',content:' '}]));
  assert.throws(()=>validateFiles([{name:'x.tex',content:'a'.repeat(32769)}]));
  assert.throws(()=>validateFiles([...files,...files]));
  assert.throws(()=>buildPrompt(template,files,'x'.repeat(501)));
});
