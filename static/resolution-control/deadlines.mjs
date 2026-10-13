// Read-only deadline overview. The backend remains the sole authority for
// owner, revision, schedule facts and readiness; no synthetic dates or writes.
const OFFSET_MS = 2 * 60 * 60 * 1000;
const DAY_MS = 24 * 60 * 60 * 1000;
const categories = [
  ['overdue', 'Overdue'],
  ['today', 'Today'],
  ['soon', 'Next 7 days'],
  ['later', 'Later'],
  ['unknown', 'Date unknown']
];
const localIso = timestamp => new Date(timestamp + OFFSET_MS).toISOString();
const calendarDay = timestamp => localIso(timestamp).slice(0, 10);
const dayNumber = day => Date.parse(day + 'T00:00:00Z') / DAY_MS;

function parseFact(fact) {
  if (!fact) return null;
  if (fact.precision === 'date_only' && /^\d{4}-\d{2}-\d{2}$/.test(fact.value)) {
    return {day: fact.value, time: null, sortTime: -1};
  }
  if (fact.precision === 'instant') {
    const instant = Date.parse(fact.value);
    if (Number.isFinite(instant)) {
      const local = localIso(instant);
      return {day: local.slice(0, 10), time: local.slice(11, 16) + ' SAST (UTC+02:00)', sortTime: instant, instant};
    }
  }
  return null;
}

// Today's precision is derived from the server's observed_at, not the client
// clock or browser timezone. Date-only facts have no presumed midnight cutoff.
export function deadlineGroups(workspace, observedAt) {
  const observed = Date.parse(observedAt);
  if (!Number.isFinite(observed)) throw new Error('Missing authoritative observation time.');
  const today = calendarDay(observed);
  const todayNumber = dayNumber(today);
  const groups = Object.fromEntries(categories.map(([key]) => [key, []]));
  for (const commitment of workspace.commitments) {
    // A preferred finish without a hard deadline is a TARGET, never an
    // institutional due date. Include it by its own date rather than hiding it.
    const hardDeadline = parseFact(commitment.schedule?.deadline);
    const targetOnly = !hardDeadline && !!commitment.schedule?.earliest_finish;
    const fact = hardDeadline || parseFact(commitment.schedule?.earliest_finish);
    const kind = hardDeadline ? 'deadline' : targetOnly && fact ? 'target' : 'unknown';
    let group = 'unknown';
    if (fact) {
      const distance = dayNumber(fact.day) - todayNumber;
      if (distance < 0 || (distance === 0 && fact.instant !== undefined && fact.instant < observed)) group = 'overdue';
      else if (distance === 0) group = 'today';
      else if (distance <= 7) group = 'soon';
      else group = 'later';
    }
    groups[group].push({
      id: commitment.id,
      title: commitment.spec.title,
      area: commitment.spec.area || 'Area unspecified',
      kind,
      day: fact?.day ?? null,
      time: fact?.time ?? null,
      sortTime: fact?.sortTime ?? -1
    });
  }
  for (const entries of Object.values(groups)) entries.sort((a, b) =>
    (a.day ?? '9999-99-99').localeCompare(b.day ?? '9999-99-99') ||
    a.sortTime - b.sortTime || a.title.localeCompare(b.title) || a.id.localeCompare(b.id));
  return {
    observedAt: localIso(observed).replace('T', ' ').slice(0, 19) + ' SAST (UTC+02:00)',
    today,
    count: workspace.commitments.length,
    deadlines: Object.values(groups).flat().filter(x=>x.kind==='deadline').length,
    targets: Object.values(groups).flat().filter(x=>x.kind==='target').length,
    categories: categories.map(([key, label]) => ({key, label, entries: groups[key]}))
  };
}

function element(tag, content, className) {
  const el = document.createElement(tag);
  if (content !== undefined) el.textContent = content;
  if (className) el.className = className;
  return el;
}

// Filtering preserves the authoritative observation and group order. It never
// mutates the source, infers missing dates or searches outside this tab.
export function filterDeadlineGroups(overview, {query = '', area = '', period = 'all'} = {}) {
  const terms = query.trim().normalize('NFKC').toLowerCase().split(/\s+/u).filter(Boolean);
  const filtered = overview.categories.map(group => ({...group, entries: group.entries.filter(entry => {
    const searchable = `${entry.title} ${entry.area} ${entry.day ?? ''}`.normalize('NFKC').toLowerCase();
    return (period === 'all' || period === group.key) && (!area || area === entry.area) && terms.every(term => searchable.includes(term));
  })}));
  return {...overview, categories: filtered, shown: filtered.reduce((sum, group) => sum + group.entries.length, 0)};
}

// DOM is constructed with textContent only. No fetched record becomes HTML.
export function renderDeadlineOverview(container, workspace, observedAt, filters = {}) {
  const overview = filterDeadlineGroups(deadlineGroups(workspace, observedAt), filters);
  container.replaceChildren();
  const summary = element('p', overview.count
    ? `${overview.count} recorded obligations (${overview.deadlines} hard deadlines, ${overview.targets} target-only dates) · as of ${overview.observedAt}. Time groups indicate due-date or target proximity, not completion or verified readiness.`
    : `No obligations recorded. Observed ${overview.observedAt}.`, 'muted rc-deadline-summary');
  container.append(summary);
  if (overview.count && !overview.shown) container.append(element('p', 'No obligations match these filters. Choose Show all obligations to return to the complete overview.', 'muted'));
  for (const group of overview.categories) {
    if (!group.entries.length) continue;
    const section = element('section', undefined, 'rc-deadline-group');
    section.dataset.urgency = group.key;
    const heading = element('h3', `${group.label} · ${group.entries.length}`);
    section.append(heading);
    const list = element('ol', undefined, 'rc-deadline-list');
    for (const entry of group.entries) {
      const item = element('li', undefined, 'rc-deadline-item');
      item.dataset.urgency = group.key;
      const when = element('div', undefined, 'rc-deadline-when');
      when.append(element('strong', entry.day ?? 'Unknown date'));
      when.append(element('span', entry.kind==='target' ? 'Internal/preferred finish target · no hard deadline' : entry.kind==='deadline' ? 'Hard deadline' : 'No recorded deadline or target', 'rc-deadline-kind'));
      when.append(element('span', entry.day ? entry.time ?? 'Time not specified' : 'Date and time unknown', 'muted'));
      const info = element('div', undefined, 'rc-deadline-info');
      info.append(element('span', entry.area, 'muted'));
      info.append(element('strong', entry.title));
      const jump = element('button', 'View obligation');
      jump.type = 'button';
      jump.setAttribute('aria-label', `View obligation: ${entry.title}`);
      jump.addEventListener('click', () => {
        // Match by attribute value, not an interpolated CSS selector.
        const card = [...document.querySelectorAll('#ledger [data-commitment]')]
          .find(node => node.dataset.commitment === entry.id);
        if (!card) return;
        card.scrollIntoView({block: 'center', behavior: 'auto'});
        card.focus({preventScroll: true});
      });
      item.append(when, info, jump);
      list.append(item);
    }
    section.append(list);
    container.append(section);
  }
  return overview;
}

export function createDeadlineUi() {
  const get = id => document.getElementById(id);
  const search = get('deadline-search'), area = get('deadline-area'), period = get('deadline-period');
  const reset = get('deadline-reset'), status = get('deadline-filter-status'), container = get('deadline-overview');
  let workspace = null, observedAt = null;
  function display() {
    if (!workspace) return;
    const overview = renderDeadlineOverview(container, workspace, observedAt, {query:search.value, area:area.value, period:period.value});
    const active = !!search.value.trim() || !!area.value || period.value !== 'all';
    status.textContent = active ? `Filtered view: ${overview.shown} of ${overview.count} obligations shown. The ledger is unchanged.` : `All ${overview.count} recorded obligations shown.`;
    reset.disabled = !active;
  }
  search.addEventListener('input', display);
  area.addEventListener('change', display);
  period.addEventListener('change', display);
  reset.addEventListener('click', () => {search.value=''; area.value=''; period.value='all'; display(); search.focus();});
  return {
    render(nextWorkspace, nextObservedAt) {
      // Build selectable areas from the same complete snapshot, before filters.
      const previousArea = area.value;
      const areas = [...new Set(nextWorkspace.commitments.map(c => c.spec.area || 'Area unspecified'))].sort((a,b)=>a.localeCompare(b));
      area.replaceChildren(new Option('All modules / areas', ''));
      for (const name of areas) area.add(new Option(name, name));
      if (areas.includes(previousArea)) area.value = previousArea;
      workspace = nextWorkspace; observedAt = nextObservedAt;
      search.disabled = area.disabled = period.disabled = false;
      display();
    },
    clear() {
      workspace = observedAt = null;
      search.value = ''; period.value = 'all';
      area.replaceChildren(new Option('All modules / areas', ''));
      search.disabled = area.disabled = period.disabled = reset.disabled = true;
      container.replaceChildren(); status.textContent = '';
    }
  };
}
