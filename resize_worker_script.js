importScripts('./resize_for_importScripts.js');
(async()=>{
  const fn=await resize;
  onmessage=async msg=>{
    postMessage(fn(msg.data.data,msg.data.sourceWidth,msg.data.sourceHeight,msg.data.targetWidth,msg.data.targetHeight,msg.data.hq));
  }
  postMessage('ready');
})();
