const url=new URL('resize.wasm',import.meta.url);
await (await fetch(url)).arrayBuffer();
const worker=await new Promise(r=>{
  // For browsers that don't support type: module on workers (firefox < 114, safari < 15)
  // const worker=new Worker(new URL('./resize_worker_script.js',import.meta.url));
  const worker=new Worker(new URL('./resize_worker_script.mjs',import.meta.url),{type:'module'});
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
 * @param {boolean} [hq=true]
 * @return {Promise<Uint8Array>}
 */
const resize=(data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq=true)=>new Promise(r=>{
  worker.onmessage=msg=>{
    worker.onmessage=null;
    r(msg.data);
  }
  worker.postMessage({data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq});
});

export {resize};
export default resize;
