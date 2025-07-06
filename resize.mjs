const url=new URL('resize.wasm',import.meta.url);
let wasm;
const imports={
  wbg:{
    __wbg_log_cb485ef4b8fcc41d:(p,n)=>{
      console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer).subarray(p,p+n)));
    },
    __wbindgen_init_externref_table:function(){
      const table=wasm.__wbindgen_export_0;
      const offset=table.grow(4);
      table.set(0);
      table.set(offset);
      table.set(offset+1,null);
      table.set(offset+2,true);
      table.set(offset+3,false);
    }
  }
};
const {instance}=await WebAssembly.instantiateStreaming(await fetch(url,{cache: 'force-cache'}),imports);
wasm=instance.exports;
const malloc=wasm.__wbindgen_malloc;
const free=wasm.__wbindgen_free;
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
  new Uint8Array(wasm.memory.buffer).set(data,p1);
  const [p2,n2]=wasm.resize(p1,n1,sourceWidth,sourceHeight,targetWidth,targetHeight,hq);
  const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
  free(p2,n2);
  return res;
};
export {resize};
export default resize;
