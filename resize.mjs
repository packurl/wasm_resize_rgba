const url=new URL('resize.wasm',import.meta.url);
const imports={wbg:{}};
const {instance: {exports: wasm}}=await WebAssembly.instantiateStreaming(await fetch(url,{cache: 'force-cache'}),imports);
const malloc=wasm.__wbindgen_malloc;
const free=wasm.__wbindgen_free;
const pointer=wasm.__wbindgen_add_to_stack_pointer;
/**
 * Resizes the supplied ImageData rgba array.
 * @param {Uint8Array} data
 * @param {number} sourceWidth
 * @param {number} sourceHeight
 * @param {number} targetWidth
 * @param {number} targetHeight
 * @param {boolean} hq
 * @return {Uint8Array}
 */
const resize=(data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq=true)=>{
  const n1=data.length;
  const p1=malloc(n1,1);
  const r=pointer(-16);
  try{
    new Uint8Array(wasm.memory.buffer).set(data,p1);
    wasm.resize(r,p1,n1,sourceWidth,sourceHeight,targetWidth,targetHeight,hq);
    const arr=new Int32Array(wasm.memory.buffer);
    const p2=arr[r/4];const n2=arr[r/4+1];
    const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
    free(p2,n2);
    return res;
  }finally{
    pointer(16);
  }
};
export {resize};
export default resize;
