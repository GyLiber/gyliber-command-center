// Tests the real frontend against synthetic API fixtures, never production credentials.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve, sep } from 'node:path';
import { chromium } from 'playwright';

const root = fileURLToPath(new URL('../../', import.meta.url));
const prefix = '/api/math-playground';
const id = '0123456789abcdef0123456789abcdef';
let drafts = [], writes = [], provider = 'openai', draftError = null;
const manifests = Object.fromEntries(await Promise.all(['giant-pi','snug-tails','creature-shuffle','memory-cloud'].map(async (id) =>
  [id, JSON.parse(await readFile(resolve(root, `math-playground/exhibits/${id}/0.1.0/manifest.json`)))])));
const server = createServer(async (request, response) => {
  try {
    const path = new URL(request.url, 'http://localhost').pathname;
    const send = (status, content, type='application/json') => {
      response.writeHead(status, {'Content-Type':type, 'Content-Security-Policy':"default-src 'self'; script-src 'self'; style-src 'self'; connect-src 'self'; object-src 'none'; base-uri 'none'"});
      response.end(typeof content === 'string' || Buffer.isBuffer(content) ? content : JSON.stringify(content));
    };
    if (path === prefix) return send(200, {csrf:'test-csrf', ai_ready:true, ai_provider:provider, publishing_ready:true, storage_ready:true, exhibits:drafts});
    if (path.startsWith(prefix) && request.method === 'POST') {
      assert.equal(request.headers['x-math-csrf'], 'test-csrf');
      let body=''; for await (const chunk of request) body += chunk;
      const data=JSON.parse(body); writes.push({path,data});
      if (path === `${prefix}/drafts`) {
        assert.equal(data.files[0].name, 'concept.tex'); assert.equal(data.provider_consent, true);
        assert.equal(data.ai_provider, provider);
        if (draftError) return send(502,{error:draftError});
        const exhibit=structuredClone(manifests['giant-pi']); exhibit.id=id; exhibit.review='ai_draft_unverified';
        exhibit.concept.title='<img src=x onerror=alert(1)>'; exhibit.concept.statement='For every Euclidean circle of radius r > 0, C = 2πr.';
        exhibit.concept.source_file='concept.tex'; exhibit.concept.source_quote=data.files[0].content;
        drafts=[exhibit]; return send(201,exhibit);
      }
      if (path.endsWith('/publish')) {
        assert.equal(data.mathematics_reviewed,true); assert.equal(data.public_code_consent,true);
        drafts[0].repository_commit='a'.repeat(40); return send(200,{commit:'a'.repeat(40)});
      }
    }
    if (path === `${prefix}/exhibits/${id}` && request.method === 'DELETE') {
      assert.equal(request.headers['x-math-csrf'],'test-csrf'); drafts=[]; return send(204,'');
    }
    if (path === `${prefix}/runtime/renderer.mjs` || path === `${prefix}/exhibits/${id}/renderer.mjs`)
      return send(200,await readFile(resolve(root,'math-playground/runtime/renderer.mjs')),'text/javascript');
    if (path === `${prefix}/exhibits/${id}/engine.mjs`)
      return send(200,await readFile(resolve(root,'math-playground/exhibits/giant-pi/0.1.0/engine.mjs')),'text/javascript');
    for (const [slug, manifest] of Object.entries(manifests)) {
      if (path === `${prefix}/demos/${slug}`) return send(200,manifest);
      if (path === `${prefix}/demos/${slug}/engine.mjs`) return send(200,await readFile(resolve(root,`math-playground/exhibits/${slug}/0.1.0/engine.mjs`)),'text/javascript');
    }
    const file=path==='/command/math-playground' ? resolve(root,'static/math-playground.html') : resolve(root,`.${path}`);
    if (!file.startsWith(resolve(root,'static')+sep)) return send(404,{});
    const type=file.endsWith('.css')?'text/css':file.endsWith('.mjs')?'text/javascript':'text/html';
    return send(200,await readFile(file),type);
  } catch { response.writeHead(500, {'Content-Type':'application/json'}); response.end('{"error":"fixture_failure"}'); }
});
await new Promise((done) => server.listen(0,'127.0.0.1',done));
const base=`http://127.0.0.1:${server.address().port}`;
let browser;
try {
  browser = await chromium.launch({headless:true});
  test('member page plays models, reveals formal mathematics, uploads, reviews and preserves exact code', async () => {
    const page = await browser.newPage({ viewport:{width:1440,height:1150}, reducedMotion:'reduce' });
    const errors=[]; page.on('pageerror',(error)=>errors.push(error.message));
    await page.goto(`${base}/command/math-playground`);
    await page.locator('canvas').waitFor();
    assert.equal(await page.locator('#formal-drawer').getAttribute('open'),null);
    await page.getByRole('slider',{name:'Unroll the ribbon'}).fill('1');
    assert.match(await page.locator('.scene-description').textContent(),/whole ribbon/);
    await page.locator('#formal-drawer > summary').click();
    assert.match(await page.locator('#formal-statement').textContent(),/For every/);
    assert.match(await page.locator('#proof-status').textContent(),/0.3.0/);
    await page.getByRole('button',{name:'Snug tails',exact:true}).click();
    await page.getByRole('slider',{name:'Blanket snugness'}).waitFor();
    await page.getByRole('slider',{name:'Choose a creature'}).fill('10');
    await page.getByRole('slider',{name:'Blanket snugness'}).fill('0.1');
    assert.match(await page.locator('.scene-description').textContent(),/outside/);
    await page.getByRole('button',{name:'Creature shuffle',exact:true}).click();
    await page.getByRole('slider',{name:'How many creatures?'}).waitFor();
    await page.getByRole('slider',{name:'How many creatures?'}).fill('6');
    assert.equal(await page.getByRole('slider',{name:'Shuffle their places'}).getAttribute('max'),'719');
    await page.getByRole('button',{name:'Memory cloud',exact:true}).click();
    await page.getByRole('slider',{name:'Give the idea room'}).waitFor();
    assert.equal(await page.locator('#scene-kind').textContent(),'MNEMONIC METAPHOR');
    await page.locator('#tex-files').setInputFiles({name:'concept.tex',mimeType:'text/plain',buffer:Buffer.from('C = 2\\pi r')});
    await page.locator('#provider-consent').check();
    await page.waitForFunction(()=>!document.getElementById('generate').disabled);
    await page.locator('#generate').click();
    await page.locator('#release-panel').waitFor({state:'visible'});
    assert.equal(await page.locator('#scene-title img').count(),0);
    assert.match(await page.locator('#review-status').textContent(),/unverified/);
    const downloadPromise=page.waitForEvent('download'); await page.locator('#download').click();
    const download=await downloadPromise, packet=JSON.parse(await readFile(await download.path()));
    assert.ok(packet.files['engine.mjs'].includes('export const config'));
    assert.ok(packet.files['renderer.mjs'].includes('export function mount'));
    assert.ok(!JSON.stringify(packet).includes('concept.tex'));
    assert.equal(await page.locator('#publish').isDisabled(),true);
    await page.locator('#math-reviewed').check(); await page.locator('#public-code').check();
    await page.locator('#publish').click();
    await page.waitForFunction(()=>document.getElementById('repository-receipt').querySelector('a'));
    assert.match(await page.locator('#repository-receipt a').getAttribute('href'),/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa/);
    assert.equal(writes.length,2);
    page.once('dialog',(dialog)=>dialog.accept()); await page.locator('#delete-exhibit').click();
    await page.waitForFunction(()=>document.getElementById('release-panel').hidden);
    assert.equal(drafts.length,0);
    // Consent must follow the actual configured provider, including a provider change.
    provider='gemini'; await page.locator('#refresh-shelf').click();
    await page.waitForFunction(()=>document.getElementById('provider-consent-text').textContent.includes('Google Gemini'));
    assert.equal(await page.locator('#provider-consent').isChecked(),false);
    assert.match(await page.locator('#provider-consent-text').textContent(),/human review/);
    assert.match(await page.locator('#service-status').textContent(),/configured/);
    await page.locator('#provider-consent').check();
    for (const [code, message] of [
      ['ai_provider_key_invalid', 'Google Gemini rejected the API key'],
      ['ai_provider_request_rejected', 'rejected the request configuration'],
      ['ai_provider_model_unavailable', 'model was not found'],
      ['ai_provider_timeout', 'did not respond within 90 seconds'],
      ['ai_provider_unavailable', 'temporarily unreachable or unavailable'],
    ]) {
      draftError=code; const before=writes.length;
      await page.waitForFunction(()=>!document.getElementById('generate').disabled);
      await page.locator('#generate').click();
      await page.waitForFunction((text)=>document.getElementById('notice').textContent.includes(text),message);
      assert.equal(writes.length,before+1);
      assert.equal(drafts.length,0);
      assert.ok(!(await page.locator('#notice').textContent()).includes('billing'));
    }
    draftError=null;
    await page.setViewportSize({width:390,height:844});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth <= window.innerWidth),true);
    assert.deepEqual(errors,[]);
    if (process.env.PLAYGROUND_SCREENSHOT) await page.screenshot({path:process.env.PLAYGROUND_SCREENSHOT,fullPage:true});
    await page.close();
  });
} finally {
  // node:test awaits pending tests before process exit; cleanup belongs in an after hook.
  const { after } = await import('node:test');
  after(async()=>{ await browser?.close(); await new Promise((done)=>server.close(done)); });
}
