import * as engine from './engine.mjs';
import { mount } from './renderer.mjs';
import { concept } from './formal.mjs';
const dispose=mount(document.getElementById('scene'),engine);
for(const key of ['notation','hypotheses','statement','visual_mapping','limitations']) {
  const p=document.createElement('p');p.textContent=concept[key];document.getElementById('formal').append(p);
}
window.addEventListener('pagehide',dispose,{once:true});
