// Reviewed app-trusted renderer; motion never changes mathematical distances.
export function mount(container, engine) {
  const canvas=document.createElement('canvas'); canvas.width=1000; canvas.height=520;
  canvas.setAttribute('role','img'); canvas.setAttribute('aria-label','Metric couriers on the finite set {1, 3, 4}');
  const controls=document.createElement('div'); controls.className='scene-controls';
  const output=document.createElement('p'); output.className='scene-description'; output.setAttribute('aria-live','polite');
  const input={x:1,y:3,z:4}, selects={};
  for (const [key,name] of [['x','Start x'],['y','Via y'],['z','Finish z']]) {
    const label=document.createElement('label'), text=document.createElement('span'), select=document.createElement('select');
    text.textContent=name; select.setAttribute('aria-label',name);
    for (const point of engine.points) { const option=document.createElement('option'); option.value=point; option.textContent=point; select.append(option); }
    select.value=input[key]; selects[key]=select;
    select.addEventListener('change',()=>{input[key]=Number(select.value); phase=0; draw();});
    label.append(text,select); controls.append(label);
  }
  const actions=document.createElement('div'); actions.className='scene-controls';
  const motion=document.createElement('button'), reset=document.createElement('button');
  for (const button of [motion,reset]) {button.type='button';button.className='secondary-btn';}
  motion.textContent='Animate'; reset.textContent='Reset'; actions.append(motion,reset);
  container.replaceChildren(canvas,controls,actions,output);
  const ctx=canvas.getContext('2d'), preference=matchMedia('(prefers-reduced-motion: reduce)');
  let paused=true, disposed=false, frame=null, phase=0, last=null;
  const px=p=>170+(p-1)*210;
  function line(from,to,height,color) {
    ctx.beginPath();ctx.moveTo(px(from),height);ctx.lineTo(px(to),height);ctx.strokeStyle=color;ctx.lineWidth=6;ctx.stroke();
  }
  function draw() {
    if(disposed) return;
    const s=engine.evaluate(input);
    ctx.fillStyle='#151e27';ctx.fillRect(0,0,1000,520);
    ctx.fillStyle='#c2ccd5';ctx.font='24px system-ui';ctx.textAlign='center';ctx.fillText('A direct trip and a trip via a friend',500,54);
    line(s.x,s.z,290,'#74a3ae'); line(s.x,s.y,365,'#b5a4cd'); line(s.y,s.z,400,'#c6ad82');
    for(const [i,p] of engine.points.entries()) {
      const x=px(p), y=190;
      ctx.fillStyle=['#74a3ae','#b5a4cd','#c6ad82'][i];ctx.beginPath();ctx.arc(x,y,38,0,Math.PI*2);ctx.fill();
      ctx.fillStyle='#19232e';for(const side of [-1,1]){ctx.beginPath();ctx.arc(x+side*12,y-7,5,0,Math.PI*2);ctx.fill();}
      ctx.strokeStyle='#19232e';ctx.lineWidth=3;ctx.beginPath();ctx.arc(x,y+7,12,0,Math.PI);ctx.stroke();
      ctx.fillStyle='#c2ccd5';ctx.font='22px system-ui';ctx.fillText(`Point ${p}`,x,253);
    }
    const t=(phase%4)/4;
    const along=(a,b,q)=>px(a)+(px(b)-px(a))*q;
    // Progress along each route uses actual engine lengths; zero routes stay put.
    const travelled=t*s.detour;
    const detourX=s.detour===0?px(s.x):travelled<s.first?along(s.x,s.y,travelled/s.first):along(s.y,s.z,s.second===0?1:(travelled-s.first)/s.second);
    for(const [x,y,color] of [[along(s.x,s.z,t),290,'#74a3ae'],[detourX,travelled<s.first?365:400,'#c6ad82']]) {
      ctx.fillStyle=color;ctx.beginPath();ctx.arc(x,y,10,0,Math.PI*2);ctx.fill();
    }
    ctx.fillStyle='#c2ccd5';ctx.font='22px system-ui';ctx.fillText(`${s.direct} ≤ ${s.first} + ${s.second} = ${s.detour}`,500,464);
    const message=`Start ${s.x}, via ${s.y}, finish ${s.z}. Direct distance ${s.direct}; detour ${s.first} + ${s.second} = ${s.detour}. ${s.equality?'Equal lengths.':'The detour is longer by '+s.slack+'.'} Distances use |a − b|; vertical lanes and faces are decoration.`;
    if(output.textContent!==message) output.textContent=message;
  }
  function stop() {if(frame!==null)cancelAnimationFrame(frame);frame=null;last=null;}
  function tick(time) {
    frame=null;if(disposed||paused||document.hidden)return;
    if(last!==null)phase+=Math.min((time-last)/1000,.1);last=time;draw();frame=requestAnimationFrame(tick);
  }
  function resume() {if(!disposed&&!paused&&!document.hidden&&frame===null)frame=requestAnimationFrame(tick);}
  function setPaused(value) {paused=value;motion.textContent=paused?'Animate':'Pause motion';stop();draw();resume();}
  motion.addEventListener('click',()=>setPaused(!paused));
  reset.addEventListener('click',()=>{Object.assign(input,{x:1,y:3,z:4});for(const key of Object.keys(selects))selects[key].value=input[key];phase=0;setPaused(true);});
  const visibility=()=>{stop();resume();}, reduced=()=>{if(preference.matches)setPaused(true);};
  document.addEventListener('visibilitychange',visibility);preference.addEventListener('change',reduced);draw();
  return ()=>{disposed=true;stop();document.removeEventListener('visibilitychange',visibility);preference.removeEventListener('change',reduced);container.replaceChildren();};
}
