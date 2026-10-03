// Tests the real frontend against synthetic API fixtures, never production credentials.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { createHash } from 'node:crypto';
import { readFile, mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { resolve, sep } from 'node:path';
import { chromium } from 'playwright';

const root = fileURLToPath(new URL('../../', import.meta.url));
const prefix = '/api/math-playground';
const id = '0123456789abcdef0123456789abcdef';
const legacyId = 'abcdef0123456789abcdef0123456789';
const hash = (value) => createHash('sha256').update(value).digest('hex');
const legacy = JSON.parse(await readFile(resolve(root,'math-playground/exhibits/giant-pi/0.1.0/manifest.json')));
let drafts = [], writes = [], provider = 'openai', draftError = null;
const manifests = Object.fromEntries(await Promise.all(['giant-pi','snug-tails','creature-shuffle','memory-cloud'].map(async (id) =>
  [id, JSON.parse(await readFile(resolve(root, `math-playground/exhibits/${id}/0.2.0/manifest.json`)))])));
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
    if ([`${prefix}/runtime/renderer.mjs`, `${prefix}/runtime/0.2.0/renderer.mjs`, `${prefix}/exhibits/${id}/renderer.mjs`].includes(path))
      return send(200,await readFile(resolve(root,'math-playground/runtime/renderer-0.2.0.mjs')),'text/javascript');
    if ([`${prefix}/runtime/0.1.0/renderer.mjs`, `${prefix}/exhibits/${legacyId}/renderer.mjs`].includes(path))
      return send(200,await readFile(resolve(root,'math-playground/runtime/renderer.mjs')),'text/javascript');
    if ([`${prefix}/exhibits/${id}/engine.mjs`, `${prefix}/exhibits/${legacyId}/engine.mjs`].includes(path))
      return send(200,await readFile(resolve(root,'math-playground/exhibits/giant-pi/0.1.0/engine.mjs')),'text/javascript');
    for (const [slug, manifest] of Object.entries(manifests)) {
      if (path === `${prefix}/demos/${slug}`) return send(200,manifest);
      if (path === `${prefix}/demos/${slug}/engine.mjs`) return send(200,await readFile(resolve(root,`math-playground/exhibits/${slug}/0.2.0/engine.mjs`)),'text/javascript');
    }
    const file=path==='/command/math-playground' ? resolve(root,'static/math-playground.html') : resolve(root,`.${path}`);
    if (!file.startsWith(resolve(root,'static')+sep)) return send(404,{});
    const type=file.endsWith('.css')?'text/css':file.endsWith('.mjs')?'text/javascript':'text/html';
    return send(200,await readFile(file),type);
  } catch { response.writeHead(500, {'Content-Type':'application/json'}); response.end('{"error":"fixture_failure"}'); }
});
await new Promise((done) => server.listen(0,'127.0.0.1',done));
const base=`http://127.0.0.1:${server.address().port}`;
async function assertDarkScene(page) {
  const pixel=await page.locator('canvas').evaluate(c=>Array.from(c.getContext('2d').getImageData(0,0,1,1).data));
  assert.deepEqual(pixel,[21,30,39,255]);
  assert.equal(await page.getByRole('button',{name:'Animate',exact:true}).count(),1);
}
async function assertTextContrast(page) {
  const results=await page.evaluate(()=>{
    const rgb=s=>s.match(/[\d.]+/g).map(Number);
    const lum=c=>c.slice(0,3).map(v=>{v/=255;return v<=.04045?v/12.92:((v+.055)/1.055)**2.4}).reduce((n,v,i)=>n+v*[.2126,.7152,.0722][i],0);
    return [...document.querySelectorAll('.intro,.badge,.version,.scene-description,.formal dt,.formal dd,.formal pre,.review-status,.proof-note,.demo-tabs button')].filter(e=>e.getClientRects().length).map(e=>{
      let p=e,bg; do {
        const style=getComputedStyle(p);bg=rgb(style.backgroundColor);
        if(style.backgroundImage!=='none') {
          const stops=[...style.backgroundImage.matchAll(/rgba?\([^)]+\)/g)].map(m=>rgb(m[0]));
          if(stops.length) bg=stops.reduce((a,b)=>lum(a)>lum(b)?a:b);
        }
        p=p.parentElement;
      } while(bg[3]===0&&p);
      const fg=rgb(getComputedStyle(e).color),a=lum(fg),b=lum(bg);
      return {tag:e.className||e.tagName,ratio:(Math.max(a,b)+.05)/(Math.min(a,b)+.05)};
    });
  });
  for (const {tag,ratio} of results) assert.ok(ratio>=4.5,`${tag} contrast ${ratio}`);
}
let browser;
try {
  browser = await chromium.launch({headless:true});
  test('member page plays models, reveals formal mathematics, uploads, reviews and preserves exact code', async () => {
    const page = await browser.newPage({ viewport:{width:1440,height:1150}, reducedMotion:'reduce' });
    const errors=[]; page.on('pageerror',(error)=>errors.push(error.message));
    await page.goto(`${base}/command/math-playground`);
    await page.locator('canvas').waitFor();
    assert.equal(await page.locator('#formal-drawer').getAttribute('open'),null);
    await assertDarkScene(page);
    // Exercise every supported scene/palette with the actual renderer, without AI.
    const paletteChecks=await page.evaluate(async()=>{
      const renderer=await import('/api/math-playground/runtime/0.2.0/renderer.mjs');
      const checks=[];
      for (const slug of ['giant-pi','snug-tails','creature-shuffle','memory-cloud']) {
        const engine=await import(`/api/math-playground/demos/${slug}/engine.mjs`);
        for (const palette of ['candy','ocean','sunset']) {
          const container=document.createElement('div');document.body.append(container);
          const dispose=renderer.mount(container,{...engine,config:{...engine.config,palette}});
          const pixels=container.querySelector('canvas').getContext('2d').getImageData(0,0,1000,520).data;
          let bright=0;for(let i=0;i<pixels.length;i+=4) if(Math.min(pixels[i],pixels[i+1],pixels[i+2])>220) bright++;
          checks.push({slug,palette,bright});dispose();container.remove();
        }
      }
      return checks;
    });
    for (const {slug,palette,bright} of paletteChecks) assert.equal(bright,0,`${slug}/${palette} bright pixels`);
    await page.emulateMedia({reducedMotion:'no-preference'});
    assert.equal(await page.getByRole('button',{name:'Animate',exact:true}).count(),1);
    const still=await page.locator('canvas').evaluate(canvas=>canvas.toDataURL());
    await page.waitForTimeout(150);
    assert.equal(await page.locator('canvas').evaluate(canvas=>canvas.toDataURL()),still);
    await page.getByRole('button',{name:'Animate',exact:true}).click();
    await page.getByRole('button',{name:'Pause motion',exact:true}).waitFor();
    await page.emulateMedia({reducedMotion:'reduce'});
    await page.getByRole('button',{name:'Animate',exact:true}).waitFor();

    await page.getByRole('slider',{name:'Unroll the ribbon'}).fill('1');
    assert.match(await page.locator('.scene-description').textContent(),/whole ribbon/);
    await page.locator('#formal-drawer > summary').click();
    assert.match(await page.locator('#formal-statement').textContent(),/For every/);
    assert.match(await page.locator('#proof-status').textContent(),/0.3.0/);
    await assertTextContrast(page);
    await page.locator('[data-demo=giant-pi]').focus();
    await page.keyboard.press('Tab');
    assert.equal(await page.evaluate(()=>getComputedStyle(document.activeElement).outlineStyle),'solid');
    if (process.env.PLAYGROUND_SCREENSHOT_DIR) {
      await mkdir(process.env.PLAYGROUND_SCREENSHOT_DIR,{recursive:true});
      await page.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'dark-desktop.png'),fullPage:true});
    }

    await page.getByRole('button',{name:'Snug tails',exact:true}).click();
    await page.getByRole('slider',{name:'Blanket snugness'}).waitFor();
    await assertDarkScene(page);
    await page.getByRole('slider',{name:'Choose a creature'}).fill('10');
    await page.getByRole('slider',{name:'Blanket snugness'}).fill('0.1');
    assert.match(await page.locator('.scene-description').textContent(),/outside/);
    await page.getByRole('button',{name:'Creature shuffle',exact:true}).click();
    await page.getByRole('slider',{name:'How many creatures?'}).waitFor();
    await assertDarkScene(page);
    await page.getByRole('slider',{name:'How many creatures?'}).fill('6');
    assert.equal(await page.getByRole('slider',{name:'Shuffle their places'}).getAttribute('max'),'719');
    await page.getByRole('button',{name:'Memory cloud',exact:true}).click();
    await page.getByRole('slider',{name:'Give the idea room'}).waitFor();
    await assertDarkScene(page);
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
    assert.equal(download.suggestedFilename(),`math-exhibit-${id}-0.2.0.json`);
    assert.equal(hash(packet.files['engine.mjs']),packet.manifest.engine_sha256);
    assert.equal(hash(packet.files['renderer.mjs']),packet.manifest.renderer_sha256);

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
    await page.locator('#tex-files').setInputFiles({name:'concept.tex',mimeType:'text/plain',buffer:Buffer.from('C = 2\\pi r')});
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
    // Existing packages open behind a dark choice, with exact legacy downloads.
    drafts=[{...structuredClone(legacy),id:legacyId,review:'ai_draft_unverified'}];
    await page.locator('#refresh-shelf').click();
    await page.locator('.shelf-card').waitFor(); await page.locator('.shelf-card').click();
    await page.getByRole('button',{name:'Dim original colours',exact:true}).waitFor();
    assert.equal(await page.locator('canvas').count(),0);
    await page.getByRole('button',{name:'Dim original colours',exact:true}).click();
    await page.locator('.dim-legacy canvas').waitFor();
    assert.notEqual(await page.locator('canvas').evaluate(c=>getComputedStyle(c).filter),'none');
    const oldDownloadPromise=page.waitForEvent('download'); await page.locator('#download').click();
    const oldDownload=await oldDownloadPromise;
    const oldPacket=JSON.parse(await readFile(await oldDownload.path()));
    assert.equal(oldDownload.suggestedFilename(),`math-exhibit-${legacyId}-0.1.0.json`);
    assert.equal(oldPacket.files['renderer.mjs'],await readFile(resolve(root,'math-playground/runtime/renderer.mjs'),'utf8'));
    assert.equal(hash(oldPacket.files['renderer.mjs']),legacy.renderer_sha256);
    await page.getByRole('button',{name:'Pi’s ribbon',exact:true}).click();
    await page.getByRole('slider',{name:'Unroll the ribbon'}).waitFor();
    assert.equal(await page.locator('.dim-legacy').count(),0);
    await assertDarkScene(page);
    await page.locator('#formal-drawer > summary').click();
    await page.setViewportSize({width:390,height:844});
    assert.equal(await page.evaluate(()=>document.documentElement.scrollWidth <= window.innerWidth),true);
    await assertTextContrast(page);
    if (process.env.PLAYGROUND_SCREENSHOT_DIR) await page.screenshot({path:resolve(process.env.PLAYGROUND_SCREENSHOT_DIR,'dark-mobile.png'),fullPage:true});
    assert.deepEqual(errors,[]);

    if (process.env.PLAYGROUND_SCREENSHOT) await page.screenshot({path:process.env.PLAYGROUND_SCREENSHOT,fullPage:true});
    await page.close();
  });
} finally {
  // node:test awaits pending tests before process exit; cleanup belongs in an after hook.
  const { after } = await import('node:test');
  after(async()=>{ await browser?.close(); await new Promise((done)=>server.close(done)); });
}
