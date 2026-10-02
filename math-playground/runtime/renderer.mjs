// Reviewed renderer. Mathematical results come only from the exhibit engine.
export function mount(container, engine) {
  const palette = {
    candy: ['#f6aed5', '#b39cff', '#9fe4d0', '#ffe68a'],
    ocean: ['#8dddeb', '#879af3', '#b6f0cc', '#fecba6'],
    sunset: ['#ffa69e', '#ffcd75', '#cbb4ff', '#f4a8d1'],
  }[engine.config.palette];
  const canvas = document.createElement('canvas');
  canvas.width = 1000; canvas.height = 520;
  canvas.setAttribute('role', 'img');
  canvas.setAttribute('aria-label', 'Interactive mathematics scene');
  const controls = document.createElement('div'); controls.className = 'scene-controls';
  const output = document.createElement('p'); output.className = 'scene-description';
  output.setAttribute('aria-live', 'polite');
  const motion = document.createElement('button'); motion.type = 'button'; motion.className = 'secondary-btn';
  let paused = matchMedia('(prefers-reduced-motion: reduce)').matches;
  motion.textContent = paused ? 'Animate' : 'Pause motion';
  motion.addEventListener('click', () => { paused = !paused; motion.textContent = paused ? 'Animate' : 'Pause motion'; draw(); });
  const input = {};
  function slider(key, labelText, min, max, step, initial) {
    const label = document.createElement('label');
    const name = document.createElement('span'); name.textContent = labelText;
    const value = document.createElement('output'); value.textContent = initial;
    const range = document.createElement('input'); range.type = 'range';
    range.min = min; range.max = max; range.step = step; range.value = initial;
    range.setAttribute('aria-label', labelText);
    input[key] = Number(initial);
    range.addEventListener('input', () => { input[key] = Number(range.value); value.textContent = range.value; draw(); });
    label.append(name, value, range); controls.append(label); return range;
  }
  switch (engine.config.kind) {
    case 'circle': slider('radius', 'Make Pi bigger', 0.5, 3, 0.05, 1.5); slider('unwrap', 'Unroll the ribbon', 0, 1, 0.01, 0); break;
    case 'sequence': slider('epsilon', 'Blanket snugness', 0.02, 0.5, 0.01, 0.15); slider('n', 'Choose a creature', 1, 100, 1, 10); break;
    case 'permutation': {
      const size = slider('n', 'How many creatures?', 2, 6, 1, 4);
      const arrangement = slider('index', 'Shuffle their places', 0, 23, 1, 0);
      size.addEventListener('input', () => {
        arrangement.max = engine.factorial(Number(size.value)) - 1;
        if (Number(arrangement.value) > Number(arrangement.max)) {
          arrangement.value = 0; arrangement.dispatchEvent(new Event('input'));
        }
      });
      break;
    }
    case 'metaphor': slider('spread', 'Give the idea room', 0, 1, 0.01, 0.5); break;
  }
  controls.append(motion); container.replaceChildren(canvas, controls, output);
  const ctx = canvas.getContext('2d');
  let phase = 0, last = null, frame;
  function circle(x, y, radius, fill) {
    ctx.beginPath(); ctx.arc(x, y, radius, 0, 2 * Math.PI); ctx.fillStyle = fill; ctx.fill();
  }
  function face(x, y, size, color, happy = true, wobble = 0) {
    circle(x, y, size, color);
    for (const side of [-1, 1]) {
      circle(x + side * size * .3, y - size * .1, size * .2, '#fff9f2');
      circle(x + side * size * .3 + wobble * 2, y - size * .08, size * .09, '#23334c');
    }
    ctx.beginPath(); ctx.strokeStyle = '#23334c'; ctx.lineWidth = Math.max(2, size * .06);
    ctx.arc(x, y + size * .13, size * .22, happy ? 0 : Math.PI, happy ? Math.PI : 2 * Math.PI); ctx.stroke();
    ctx.fillStyle = '#ee7bb0';
    ctx.beginPath(); ctx.ellipse(x - size * .6, y + size * .16, size * .12, size * .06, 0, 0, 7); ctx.fill();
  }
  function setDescription(text) { if (output.textContent !== text) output.textContent = text; }
  function draw() {
    const state = engine.evaluate(input, phase);
    const wiggle = Math.sin(phase * Math.PI * 2);
    ctx.clearRect(0, 0, 1000, 520);
    ctx.fillStyle = '#fbf5ed'; ctx.fillRect(0, 0, 1000, 520);
    // Soft confetti, generated deterministically from the package identifier.
    for (let i = 0; i < 18; i++) {
      const offset = engine.config.id.charCodeAt(i % engine.config.id.length);
      circle((i * 131 + offset * 7) % 1000, (i * 79 + offset * 3) % 490, 4 + i % 5, palette[i % 4] + '60');
    }
    if (state.kind === 'circle') {
      const scale = 36; const radius = state.radius * scale;
      face(250, 245 + wiggle * 3, radius, palette[0], true, wiggle);
      ctx.font = `bold ${radius * .9}px Georgia`; ctx.textAlign = 'center'; ctx.fillStyle = '#694c88';
      ctx.fillText('π', 250, 245 + radius * .85);
      const ribbonY = 400;
      ctx.lineWidth = 14; ctx.lineCap = 'round'; ctx.strokeStyle = palette[1];
      ctx.beginPath(); ctx.arc(250, 245 + wiggle * 3, radius,
        -Math.PI / 2 + state.unwrap * Math.PI * 2, 3 * Math.PI / 2); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(120, ribbonY); ctx.lineTo(120 + state.ribbonLength * scale, ribbonY); ctx.stroke();
      face(120 + state.ribbonLength * scale, ribbonY, 12, palette[2], true);
      setDescription(state.unwrap > .99 ? 'The whole ribbon is one trip around Pi. Make Pi bigger: the ribbon grows with it.'
        : 'Pi is wearing a ribbon. Slide to unroll it, then change Pi’s size.');
    } else if (state.kind === 'sequence') {
      const baseline = 440, height = 320;
      ctx.fillStyle = palette[2] + 'a0'; ctx.fillRect(40, baseline - height * state.epsilon, 920, height * state.epsilon + 18);
      for (let i = 0; i < state.terms.length; i++) {
        const x = 60 + i * 18, y = baseline - height * state.terms[i];
        face(x, y + Math.sin(phase * 6.28 + i) * 2, i === state.n - 1 ? 15 : 7,
          state.terms[i] < state.epsilon ? palette[2] : palette[0], true, wiggle);
      }
      const selectedY = baseline - height * state.value;
      face(880, Math.max(80, selectedY - 35), 36, state.inside ? palette[2] : palette[0], state.inside, wiggle);
      setDescription(`Creature ${state.n} is ${state.inside ? 'snug inside' : 'outside'} the blanket. Every creature from ${state.threshold} onward fits. The screen shows the first 50; your selected creature is enlarged.`);
    } else if (state.kind === 'permutation') {
      const x0 = 500 - (state.n - 1) * 65;
      for (const [position, creature] of state.order.entries()) {
        const x = x0 + position * 130, y = 250 + Math.sin(phase * 6.28 + creature) * 12;
        face(x, y, 44, palette[creature % 4], true, wiggle);
        // Distinct accessories keep creatures identifiable even when colors repeat.
        for (let dot = 0; dot <= creature; dot++) circle(x - creature * 6 + dot * 12, y - 61, 4, '#694c88');
        ctx.font = 'bold 18px system-ui'; ctx.textAlign = 'center'; ctx.fillStyle = '#23334c';
        ctx.fillText(String(creature + 1), x, y + 75);
      }
      setDescription(`Arrangement ${state.index + 1} of ${state.count}. The same creatures each appear exactly once; only their places change.`);
    } else {
      const radius = 60 + state.spread * 130;
      for (let i = 0; i < 6; i++) {
        const angle = i * Math.PI / 3 + wiggle * .05;
        ctx.beginPath(); ctx.moveTo(500, 260); ctx.lineTo(500 + Math.cos(angle) * radius, 260 + Math.sin(angle) * radius);
        ctx.strokeStyle = palette[1]; ctx.lineWidth = 4; ctx.stroke();
        face(500 + Math.cos(angle) * radius, 260 + Math.sin(angle) * radius, 26, palette[i % 4], true, wiggle);
      }
      face(500, 260, 45, palette[0], true, wiggle);
      setDescription('A memory creature for an abstract idea. These shapes and connections do not assert a mathematical relationship. Reveal the concept to read what the scene stands for.');
    }
    canvas.setAttribute('aria-label', output.textContent);
  }
  function tick(now) {
    if (!paused && !document.hidden) {
      if (last !== null) phase = (phase + Math.min(now - last, 100) / 6000) % 1;
      draw();
    }
    last = now; frame = requestAnimationFrame(tick);
  }
  draw(); frame = requestAnimationFrame(tick);
  return () => { cancelAnimationFrame(frame); container.replaceChildren(); };
}
