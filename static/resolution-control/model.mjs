const bytes = new TextEncoder();
export function text(value, max, required = false) {
  value = String(value ?? '').trim();
  if (!value) { if (required) throw new Error('Fill the required field.'); return null; }
  if (bytes.encode(value).length > max || /[\x00-\x08\x0b\x0c\x0e-\x1f\x7f]/u.test(value)) throw new Error(`Keep this field within ${max} UTF-8 bytes, without control characters.`);
  return value;
}
export function reference(value) {
  value = text(value, 2048);
  if (!value) return null;
  if (/^https?:/i.test(value)) {
    let url; try { url = new URL(value); } catch { throw new Error('Use a complete HTTP(S) URL or a text reference.'); }
    if (url.username || url.password || url.search || url.hash || /\s/u.test(value)) throw new Error('Remove credentials, query parameters and fragments from reference URLs.');
    return {kind:'web', value:url.href};
  }
  return {kind:'text', value};
}
export function timeFact(precision, value) {
  if (precision === 'unknown') return null;
  if (precision === 'date_only' && /^\d{4}-\d{2}-\d{2}$/.test(value)) return {precision, value};
  if (precision === 'instant' && /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(?::\d{2})?$/.test(value)) return {precision, value:`${value}${value.length===16?":00":""}+02:00`};
  throw new Error('Enter a complete date/time for the selected precision. Exact times use UTC+02:00.');
}
export function schedule(data) {
  const deadline = timeFact(data.get('deadline_precision'), data.get('deadline'));
  const earliest_finish = timeFact(data.get('earliest_precision'), data.get('earliest'));
  const raw = String(data.get('buffer') ?? '').trim();
  if (raw && (!/^\d+$/.test(raw) || Number(raw) > 525600)) throw new Error('Buffer must be whole minutes from 0 to 525600, or blank for unknown.');
  const buffer_minutes = raw ? Number(raw) : null;
  if (deadline?.precision === 'instant' && earliest_finish?.precision === 'instant') {
    const d = Date.parse(deadline.value), e = Date.parse(earliest_finish.value);
    if (!Number.isFinite(d) || !Number.isFinite(e) || e > d - (buffer_minutes ?? 0)*60000) throw new Error('Earliest finish must be at or before deadline minus the protected buffer.');
  }
  return {deadline, earliest_finish, buffer_minutes};
}
export function plan(data) {
  return {instruction:text(data.get('instruction'),2048,true), expected_artifact:text(data.get('expected_artifact'),2048), verification_method:text(data.get('verification_method'),2048), start_reference:reference(data.get('start_reference'))};
}
export function exactLocal(value) { const local=new Date(Date.parse(value)+120*60000).toISOString().slice(0,19); if(!/^\d{4}-/.test(local))throw new Error('This time is outside the supported UTC+02:00 input range.');return local; }
export function displayTime(fact) { if(!fact)return 'Unknown';if(fact.precision==='date_only')return `${fact.value} · date only`;try{return `${exactLocal(fact.value).replace('T',' ')} · UTC+02:00`;}catch{return `${fact.value} · UTC (outside UTC+02:00 input range)`;} }
export function errorMessage(code) {
  return ({authentication_required:'Your session has ended. Sign in again.', reauthentication_required:'Sign out and sign in again to refresh your identity.', feature_disabled:'Resolution Control is disabled on this deployment. No changes were saved.', storage_unavailable:'Private storage is unavailable. Refresh before continuing.', csrf_required:'The request token was rejected. Refresh and review before trying again.', same_origin_required:'Open this page on the site’s own address.', refresh_required:'Another edit or recovery changed this workspace. Refresh and reload the item for review; your draft is preserved.', deleted_generation:'This request belongs to a replaced or deleted workspace. Refresh and review.', capacity_reached:'Workspace capacity has been reached. Export and review before a deliberate reset.', not_found:'The referenced item is no longer available. Refresh and review.', transition_rejected:'This transition is not permitted. Check the action state and prerequisites.', invalid_request:'Check required values, reference safety and deadline/target/buffer controls.', payload_too_large:'The request exceeds the permitted size.'})[code] ?? 'The request was rejected. Refresh and review.';
}
