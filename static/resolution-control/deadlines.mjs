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
    const fact = parseFact(commitment.schedule?.deadline);
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
    categories: categories.map(([key, label]) => ({key, label, entries: groups[key]}))
  };
}

function element(tag, content, className) {
  const el = document.createElement(tag);
  if (content !== undefined) el.textContent = content;
  if (className) el.className = className;
  return el;
}

// DOM is constructed with textContent only. No fetched record becomes HTML.
export function renderDeadlineOverview(container, workspace, observedAt) {
  const overview = deadlineGroups(workspace, observedAt);
  container.replaceChildren();
  const summary = element('p', overview.count
    ? `${overview.count} recorded obligations · as of ${overview.observedAt}. Time groups indicate deadline proximity, not completion or verified readiness.`
    : `No obligations recorded. Observed ${overview.observedAt}.`, 'muted rc-deadline-summary');
  container.append(summary);
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
}
