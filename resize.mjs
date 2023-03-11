const url=new URL('resize.wasm',import.meta.url);
await (await fetch(url)).arrayBuffer();
const src=()=>`(async()=>{
  const mod=await WebAssembly.compileStreaming(await fetch('${url}',{cache:'force-cache'}));
  const wasm=(await WebAssembly.instantiate(mod,{wbg:{}})).exports;
  const malloc=wasm.__wbindgen_malloc;const free=wasm.__wbindgen_free;
  const fn=({data,width,height,w,h,hq})=>{
    try{
      const r=wasm.__wbindgen_add_to_stack_pointer(-16);
      const n1=data.length;const p1=malloc(n1);
      new Uint8Array(wasm.memory.buffer).set(data,p1);
      wasm.resize(r,p1,n1,width,height,w,h,hq);
      const arr=new Int32Array(wasm.memory.buffer);
      const p2=arr[r/4];const n2=arr[r/4+1];
      const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
      free(p2,n2);
      return res;
    }finally{
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  };
  onmessage=async msg=>postMessage(fn(msg.data));
  postMessage('ready');
})();`
const worker=await new Promise(r=>{
  const worker=new Worker(URL.createObjectURL(new Blob([src()],{type:'application/javascript'})),{type:'module'});
  worker.onmessage=msg=>{
    if(msg.data==='ready'){
      worker.onmessage=null;
      r(worker);
    }
  };
});
/**
 * Resizes the supplied ImageData rgba array.
 * @param {Uint8Array} data
 * @param {number} sourceWidth
 * @param {number} sourceHeight
 * @param {number} targetWidth
 * @param {number} targetHeight
 * @param {boolean} hq
 * @return {Promise<Uint8Array>}
 */
const resize=(data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq=true)=>new Promise(r=>{
  worker.onmessage=msg=>{
    worker.onmessage=null;
    r(msg.data);
  }
  worker.postMessage({data,width:sourceWidth,height:sourceHeight,w:targetWidth,h:targetHeight,hq});
});
export {
  resize
};
