const resize=(async()=>{
  const imports={
    wbg:{
      __wbg_log_23c49b2eaab74338:(p,n)=>{
        console.log(new TextDecoder().decode(new Uint8Array(wasm.memory.buffer).subarray(p,p+n)));
      }
    }
  };
  const {instance: {exports: wasm}}=await WebAssembly.instantiateStreaming(await fetch('./resize.wasm',{cache: 'force-cache'}),imports);
  const malloc=wasm.__wbindgen_malloc;
  const free=wasm.__wbindgen_free;
  const pointer=wasm.__wbindgen_add_to_stack_pointer;
  return (data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq=true)=>{
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
})();
