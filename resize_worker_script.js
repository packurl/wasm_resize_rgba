importScripts('./resize_for_importScripts.js');
(async()=>{
  const fn=await resize;
  onmessage=async({data:{data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq}})=>{
    const r=fn(data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq);
    postMessage(r,[r.buffer]);
  }
  postMessage('ready');
})();
