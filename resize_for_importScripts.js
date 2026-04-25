const resize=(async()=>{
  let wasm;
  const imports={
    js:{
      println:(ptr,len)=>console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer,ptr,len)))
    }
  };
  const {instance}=await WebAssembly.instantiateStreaming(await fetch('./resize.wasm',{cache:'force-cache'}),imports);
  wasm=instance.exports;
  const malloc=wasm.malloc;
  const free=wasm.free;
  return (data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq=true)=>{
    const n1=data.length;
    const p1=malloc(n1,1);
    new Uint8Array(wasm.memory.buffer).set(data,p1);
    const r=wasm.resize(p1,n1,sourceWidth,sourceHeight,targetWidth,targetHeight,hq);
    free(p1,n1);
    const ptr_and_len=new DataView(wasm.memory.buffer,r,8);
    const p2=ptr_and_len.getUint32(0,true);
    const n2=ptr_and_len.getUint32(4,true);
    free(r,8);
    const res=new Uint8Array(wasm.memory.buffer).subarray(p2,p2+n2).slice();
    free(p2,n2);
    return res;
  };
})();
