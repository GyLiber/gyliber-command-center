import { readFile, readdir, lstat } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { resolve } from 'node:path';
const root=fileURLToPath(new URL('../',import.meta.url));
export function validateRelease(release,manifest,files) {
  if(!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(release.id)||!/^\d+\.\d+\.\d+$/.test(release.version)||typeof release.in_gallery!=='boolean'||release.visibility!=='member')throw new Error('Invalid catalog identity or visibility');
  if(typeof release.title!=='string'||!release.title.trim()||release.title.length>160)throw new Error('Invalid catalog title');
  if(manifest.format!==2||manifest.id!==release.id||manifest.version!==release.version||manifest.review!==release.review||!['sol_reviewed_public_synthetic','developer_reviewed_public'].includes(release.review)||!manifest.publication_rights)throw new Error('Release not reviewed or identity mismatch');
  if(!['mathematical_model','representative_example','mnemonic_metaphor'].includes(manifest.classification)||typeof manifest.runtime_contract!=='string')throw new Error('Missing model contract');
  for(const required of ['engine.mjs','renderer.mjs','viewer.html','viewer.mjs','viewer.css','tests.mjs','README.md'])if(!manifest.files?.[required])throw new Error('Incomplete replay package');
  if(Object.keys(files).sort().join('|')!==Object.keys(manifest.files).sort().join('|'))throw new Error('Unmanifested package bytes');
  for(const [name,digest] of Object.entries(manifest.files)) {
    if(!/^[a-zA-Z0-9][a-zA-Z0-9.-]*$/.test(name)||!/^([a-f0-9]{64})$/.test(digest))throw new Error('Unsafe file or hash');
    if(createHash('sha256').update(files[name]).digest('hex')!==digest)throw new Error('Package hash mismatch');
  }
  if(manifest.engine_sha256!==manifest.files['engine.mjs']||manifest.renderer_sha256!==manifest.files['renderer.mjs'])throw new Error('Runtime digest mismatch');
}
export async function verifyCatalog() {
  const catalog=JSON.parse(await readFile(resolve(root,'catalog.json')));
  if(catalog.format!==1||!Array.isArray(catalog.releases))throw new Error('Invalid catalog');
  const seen=new Set();
  for(const release of catalog.releases) {
    const key=`${release.id}/${release.version}`;if(seen.has(key))throw new Error('Duplicate release');seen.add(key);
    if(!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(release.id)||!/^\d+\.\d+\.\d+$/.test(release.version))throw new Error('Unsafe package path');
    const folder=resolve(root,'exhibits',release.id,release.version), files={};
    for(const name of await readdir(folder)) {if(!(await lstat(resolve(folder,name))).isFile())throw new Error('No symlinks or directories in replay package');if(name!=='manifest.json')files[name]=await readFile(resolve(folder,name));}
    validateRelease(release,JSON.parse(await readFile(resolve(folder,'manifest.json'))),files);
  }
  // CI checks the PR base: accepted version directories may never be overwritten,
  // and accepted identity/version records must remain addressable in the archive.
  const base=process.env.CURATED_BASE_SHA;
  if(base && !/^0+$/.test(base)) {
    if(!/^[a-f0-9]{40}$/.test(base))throw new Error('Invalid immutable baseline');
    const repo=resolve(root,'..');
    const old=execFileSync('git',['ls-tree','-r','--name-only',base,'--','math-playground/exhibits'],{cwd:repo,encoding:'utf8'}).trim().split('\n');
    const accepted=new Set(old.filter(name=>name.endsWith('/manifest.json')).map(name=>name.slice(0,-'manifest.json'.length)));
    const changed=execFileSync('git',['diff','--name-only',base,'HEAD','--','math-playground/exhibits'],{cwd:repo,encoding:'utf8'}).trim().split('\n');
    for(const name of changed)if([...accepted].some(folder=>name.startsWith(folder)))throw new Error('Accepted exhibit bytes changed: '+name);
    if(execFileSync('git',['ls-tree','--name-only',base,'--','math-playground/catalog.json'],{cwd:repo,encoding:'utf8'}).trim()) {
      const previous=JSON.parse(execFileSync('git',['show',`${base}:math-playground/catalog.json`],{cwd:repo,encoding:'utf8'}));
      for(const release of previous.releases)if(!seen.has(`${release.id}/${release.version}`))throw new Error('Archived release removed from catalog');
    }
  }
  return catalog;
}
if(process.argv[1] && import.meta.url===pathToFileURL(process.argv[1]).href) {
  const catalog=await verifyCatalog();console.log(`Verified ${catalog.releases.length} accepted replay package(s).`);
}
