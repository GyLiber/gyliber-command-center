const $ = (id) => document.getElementById(id);
let catalog = null, current = null, cleanup = null, selectedFiles = [], selectionVersion = 0, sceneVersion = 0;
const messages = {
  authentication_required: 'Your member session has ended. Sign in again to continue.',
  ai_authoring_not_configured: 'AI authoring has not been activated for this deployment yet.',
  ai_provider_unavailable: 'The AI provider is temporarily unreachable or unavailable. No automatic retry occurred. Try the demonstrations and report this message if it persists.',
  ai_provider_key_invalid: 'Google Gemini rejected the API key. Replace MATH_GEMINI_API_KEY in Render with a valid Google AI Studio key, then redeploy. Do not share the key in chat.',
  ai_provider_request_rejected: 'The AI provider rejected the request configuration. Report this message to Sol; repeated retries will use the daily request budget.',
  ai_provider_model_unavailable: 'The configured AI model was not found or does not support this operation. Check the selected model setting in Render, then redeploy.',
  ai_provider_timeout: 'The AI provider did not respond within 90 seconds. No automatic retry occurred. Try a smaller self-contained excerpt later.',
  ai_provider_access_denied: 'The AI key or model access was refused. Check the selected provider’s configuration.',
  ai_provider_quota_reached: 'The provider’s quota or credit limit was reached. No paid fallback was used. Try the demonstrations; do not repeatedly retry this request.',
  ai_provider_changed_refresh_consent: 'The selected AI provider changed. Refresh and review the upload consent again.',
  ai_request_refused: 'The AI provider declined this request. Choose a different mathematical excerpt.',
  ai_response_incomplete: 'The AI response was incomplete. Select a smaller self-contained concept.',
  source_quote_not_found: 'The AI could not provide a reliable source anchor. Try a clearer, smaller excerpt.',
  daily_authoring_limit_reached: 'The eight-request budget for the past 24 hours has been reached. Try again tomorrow.',
  authoring_busy_retry_later: 'Two authoring requests are already running. Retry shortly.',
  playground_storage_unavailable: 'The private exhibit store is temporarily unavailable.',
  repository_publishing_not_configured: 'Repository publishing has not been activated yet. You can download the exact package.',
  repository_publish_failed_retry_safely: 'Git could not confirm publication. Retry safely; the existing package will not be overwritten.',
  invalid_request_token: 'The request token expired. Refresh the page before continuing.',
};
function notice(message) { $('notice').textContent = message; }
async function api(path, options = {}) {
  const response = await fetch(path, { credentials: 'same-origin', cache: 'no-store', ...options,
    headers: { Accept: 'application/json', ...(options.body ? { 'Content-Type': 'application/json' } : {}),
      ...(options.method && options.method !== 'GET' ? { 'X-Math-CSRF': catalog?.csrf || '' } : {}), ...options.headers } });
  if (response.status === 204) return null;
  const data = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(messages[data.error] || (data.error ? data.error.replaceAll('_', ' ') : `Request failed (${response.status}).`));
  return data;
}
function renderShelf() {
  const list = $('exhibit-list'); list.replaceChildren();
  for (const exhibit of catalog.exhibits) {
    const button = document.createElement('button'); button.type = 'button'; button.className = 'shelf-card';
    const title = document.createElement('strong'); title.textContent = exhibit.concept.title;
    const state = document.createElement('span'); state.textContent = exhibit.repository_commit ? 'Saved to Git · member reviewed' : 'AI draft · expires after 24 hours';
    button.append(title, state); button.addEventListener('click', () => showExhibit(exhibit).catch((error) => notice(error.message))); list.append(button);
  }
  if (!catalog.exhibits.length) {
    const text = document.createElement('p'); text.className = 'muted small'; text.textContent = 'Your new exhibits will live here. Try a demonstration while the AI service is being prepared.'; list.append(text);
  }
}
function updateButtons() {
  $('generate').disabled = !catalog?.ai_ready || !selectedFiles.length || !$('provider-consent').checked;
  $('publish').disabled = !current || current.demo || Boolean(current.repository_commit) || !catalog?.publishing_ready
    || !$('math-reviewed').checked || !$('public-code').checked;
}
async function refresh() {
  const nextCatalog = await api('/api/math-playground');
  if (catalog && catalog.ai_provider !== nextCatalog.ai_provider) $('provider-consent').checked = false;
  catalog = nextCatalog;
  const provider = catalog.ai_provider === 'gemini' ? 'Google Gemini' : 'OpenAI';
  $('provider-consent-text').textContent = catalog.ai_provider === 'gemini'
    ? 'I agree to send these non-sensitive study notes to Google Gemini. Its unpaid service may use inputs and outputs to improve products, including human review. I have permission to share this material.'
    : 'These are low-risk study notes I have permission to share. I agree to send the selected text to OpenAI.';
  $('service-status').textContent = catalog.ai_ready
    ? `${provider} authoring is configured; account access and quota are checked when you create a draft. Eight attempts per member per 24 hours. No automatic provider fallback.`
    : 'AI authoring needs activation. The playable demonstrations are ready now; uploading to AI is disabled until the service is configured.';
  renderShelf(); updateButtons();
}
function renderFormal(exhibit) {
  const c = exhibit.concept;
  const fields = { 'formal-type': c.concept_type, 'formal-notation': c.notation, 'formal-hypotheses': c.hypotheses,
    'formal-statement': c.statement, 'formal-mapping': c.visual_mapping, 'formal-limitations': c.limitations,
    'source-name': c.source_file, 'source-quote': c.source_quote };
  for (const [id, text] of Object.entries(fields)) $(id).textContent = text;
  $('review-status').textContent = exhibit.demo ? 'Sol-reviewed synthetic demonstration. It was not generated from your coursework.'
    : exhibit.repository_commit ? 'Mathematics reviewed by the saving member; this is not machine verification.'
      : 'AI draft — unverified. Check the source, hypotheses and conclusion before relying on it.';
  $('proof-status').textContent = c.concept_type === 'theorem' ? 'Proof deferred to exhibit capability v0.3.0. Animation is not a proof.'
    : 'This is a concept statement; no theorem proof is claimed.';
  $('source-digest').textContent = exhibit.source_sha256 ? `Source SHA-256: ${exhibit.source_sha256}` : 'Source: original synthetic demonstration.';
  $('artifact-digest').textContent = `Engine SHA-256: ${exhibit.engine_sha256}\nRenderer SHA-256: ${exhibit.renderer_sha256}`;
  $('repository-receipt').replaceChildren();
  if (exhibit.repository_commit) {
    const link = document.createElement('a'); link.textContent = `View immutable Git snapshot ${exhibit.repository_commit.slice(0, 12)}`;
    link.href = `https://github.com/GyLiber/gyliber-command-center/tree/${exhibit.repository_commit}/math-playground/exhibits/${exhibit.id}/${exhibit.version}`;
    link.target = '_blank'; link.rel = 'noopener noreferrer'; $('repository-receipt').append(link);
  } else $('repository-receipt').textContent = exhibit.demo ? 'Reviewed engine is stored in the application repository.' : 'Private draft; code has not been published to Git.';
}
function mountScene(exhibit, renderer, engine, ticket) {
  const scene = $('scene'); scene.classList.remove('dim-legacy');
  if (exhibit.version !== '0.1.0') return renderer.mount(scene, engine);
  // Old stored code stays exact. A separate, explicitly chosen display filter
  // can dim its light palette without changing the downloaded/public package.
  let dispose = null;
  const note = document.createElement('p'); note.className = 'legacy-scene';
  note.textContent = 'Older palette. Choose a dim view, or show the original light colours. Downloads retain the original palette.';
  const actions = document.createElement('div'); actions.className = 'actions legacy-scene';
  for (const [dim, label] of [[true, 'Dim original colours'], [false, 'Show original (light colours)']]) {
    const button = document.createElement('button'); button.type = 'button'; button.className = 'secondary-btn'; button.textContent = label;
    button.addEventListener('click', () => {
      if (ticket !== sceneVersion) return;
      scene.classList.toggle('dim-legacy', dim); dispose = renderer.mount(scene, engine);
      // Use the legacy renderer's existing pause control; do not rewrite its code.
      for (const control of scene.querySelectorAll('button')) if (control.textContent === 'Pause motion') control.click();
      $('scene-version').textContent = `visual ${exhibit.version} · reveal ${exhibit.formal_version} · ${dim ? 'dim view' : 'original colours'}`;
    });
    actions.append(button);
  }
  scene.replaceChildren(note, actions);
  return () => { dispose?.(); scene.classList.remove('dim-legacy'); scene.replaceChildren(); };
}
async function showExhibit(exhibit) {
  const ticket = ++sceneVersion;
  const prefix = exhibit.demo ? `/api/math-playground/demos/${exhibit.id}` : `/api/math-playground/exhibits/${exhibit.id}`;
  const [engine, renderer] = await Promise.all([import(`${prefix}/engine.mjs`), import(exhibit.demo ? `/api/math-playground/runtime/${encodeURIComponent(exhibit.version)}/renderer.mjs` : `${prefix}/renderer.mjs`)]);
  if (ticket !== sceneVersion) return;
  cleanup?.(); current = exhibit; cleanup = mountScene(exhibit, renderer, engine, ticket);
  $('scene-title').textContent = exhibit.concept.title;
  $('scene-kind').textContent = exhibit.concept.kind === 'metaphor' ? 'MNEMONIC METAPHOR'
    : !exhibit.demo && !exhibit.repository_commit ? 'UNVERIFIED MODEL DRAFT' : 'MATHEMATICAL MODEL';
  $('scene-kind').classList.toggle('metaphor', exhibit.concept.kind === 'metaphor');
  $('scene-version').textContent = `visual ${exhibit.version} · reveal ${exhibit.formal_version}`;
  $('formal-drawer').open = false; renderFormal(exhibit);
  $('release-panel').hidden = Boolean(exhibit.demo);
  $('math-reviewed').checked = false; $('public-code').checked = false;
  $('publishing-status').textContent = exhibit.repository_commit ? 'The exact code is saved in Git. The private source mapping remains on your shelf.'
    : catalog?.publishing_ready ? 'Drafts expire after 24 hours. A reviewed, saved exhibit stays on your private shelf.'
      : 'Git publishing needs activation. Drafts expire after 24 hours; download the package to keep a local copy.';
  for (const button of document.querySelectorAll('[data-demo]')) button.setAttribute('aria-pressed', String(exhibit.demo && button.dataset.demo === exhibit.id));
  updateButtons();
}
async function demo(id) { const exhibit = await api(`/api/math-playground/demos/${id}`); exhibit.demo = true; await showExhibit(exhibit); }
for (const button of document.querySelectorAll('[data-demo]')) button.addEventListener('click', () => demo(button.dataset.demo).catch((error) => notice(error.message)));
$('refresh-shelf').addEventListener('click', () => refresh().catch((error) => notice(error.message)));
for (const id of ['provider-consent', 'math-reviewed', 'public-code']) $(id).addEventListener('change', updateButtons);
$('tex-files').addEventListener('change', async (event) => {
  const ticket = ++selectionVersion; selectedFiles = []; updateButtons();
  try {
    const files = Array.from(event.target.files);
    if (!files.length || files.length > 8 || files.some((file) => !file.name.endsWith('.tex') || file.size > 32768)
      || files.reduce((sum, file) => sum + file.size, 0) > 65536) throw new Error('Choose 1–8 .tex files, up to 32 KiB each and 64 KiB combined. Export a self-contained concept excerpt if needed.');
    const names = new Set(files.map((file) => file.name));
    if (names.size !== files.length) throw new Error('Each selected file needs a distinct filename.');
    const parsed = await Promise.all(files.map(async (file) => ({ name: file.name, content: new TextDecoder('utf-8', { fatal: true }).decode(await file.arrayBuffer()) })));
    if (ticket !== selectionVersion) return;
    selectedFiles = parsed;
    $('file-summary').textContent = `${files.map((file) => file.name).join(', ')} · ${files.reduce((sum, file) => sum + file.size, 0)} bytes. No upload has been sent yet.`;
  } catch (error) { if (ticket === selectionVersion) { $('file-summary').textContent = error.message; selectedFiles = []; } }
  updateButtons();
});
$('upload-form').addEventListener('submit', async (event) => {
  event.preventDefault(); if (!catalog?.ai_ready || !selectedFiles.length || !$('provider-consent').checked) return;
  const button = $('generate'); button.disabled = true; button.textContent = 'Reading the mathematics…'; $('tex-files').disabled = true;
  notice('Creating one concept draft. This can take up to 90 seconds. The provider receives all selected text; LaTeX is never executed.');
  try {
    const exhibit = await api('/api/math-playground/drafts', { method: 'POST', body: JSON.stringify({ files: selectedFiles, provider_consent: true, ai_provider: catalog.ai_provider || 'openai' }) });
    selectedFiles = []; $('tex-files').value = ''; $('file-summary').textContent = 'Draft prepared. Original uploads were not saved on the server.';
    await refresh(); await showExhibit(exhibit); notice('Your draft is ready. Play first; reveal the formal mathematics when you want to inspect it. Review before saving.');
  } catch (error) { notice(error.message); }
  finally { button.textContent = 'Create a playful draft ✧'; $('tex-files').disabled = false; updateButtons(); }
});
$('publish').addEventListener('click', async () => {
  const exhibit = current; if (!exhibit || exhibit.demo) return;
  $('publish').disabled = true; $('publish').textContent = 'Saving the exact code…';
  try {
    const result = await api(`/api/math-playground/exhibits/${exhibit.id}/publish`, { method: 'POST', body: JSON.stringify({ mathematics_reviewed: $('math-reviewed').checked, public_code_consent: $('public-code').checked }) });
    exhibit.repository_commit = result.commit; await refresh(); if (current?.id === exhibit.id) await showExhibit(exhibit);
    notice(`Saved to Git at ${result.commit.slice(0, 12)}. Your source and formal statement were not published.`);
  } catch (error) { notice(error.message); }
  finally { $('publish').textContent = 'Save code to Git'; updateButtons(); }
});
$('download').addEventListener('click', async () => {
  const exhibit = current; if (!exhibit || exhibit.demo) return;
  try {
    const prefix = `/api/math-playground/exhibits/${exhibit.id}`;
    const fetchCode = async (name) => { const response = await fetch(`${prefix}/${name}`, { credentials: 'same-origin', cache: 'no-store' }); if (!response.ok) throw new Error('Code download unavailable.'); return response.text(); };
    const [engine, renderer] = await Promise.all([fetchCode('engine.mjs'), fetchCode('renderer.mjs')]);
    const manifest = { format: 1, id: exhibit.id, engine_version: exhibit.version, kind: exhibit.concept.kind,
      palette: exhibit.concept.palette, engine_sha256: exhibit.engine_sha256, renderer_sha256: exhibit.renderer_sha256 };
    const blob = new Blob([JSON.stringify({ manifest, files: { 'engine.mjs': engine, 'renderer.mjs': renderer } }, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob), link = document.createElement('a'); link.href = url; link.download = `math-exhibit-${exhibit.id}-${exhibit.version}.json`; link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000); notice('Downloaded the exact engine and renderer. The package excludes your private source mapping.');
  } catch (error) { notice(error.message); }
});
$('delete-exhibit').addEventListener('click', async () => {
  const exhibit = current; if (!exhibit || exhibit.demo) return;
  if (!confirm('Delete this private exhibit and its source mapping? Any published public code stays in Git history.')) return;
  try {
    await api(`/api/math-playground/exhibits/${exhibit.id}`, { method: 'DELETE' }); await refresh();
    if (current?.id === exhibit.id) await demo('giant-pi'); notice('Private exhibit deleted. Published code, if any, remains in Git.');
  } catch (error) { notice(error.message); }
});
window.addEventListener('pagehide', () => { cleanup?.(); selectedFiles = []; });
try { await refresh(); } catch (error) { notice(error.message); }
try { await demo('giant-pi'); } catch (error) { $('scene').textContent = 'Scene unavailable. Sign in or refresh to try again.'; notice(error.message); }
