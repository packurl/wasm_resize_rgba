import {resize} from "./resize.mjs";
onmessage=async({data:{data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq}})=>{
  const r=resize(data,sourceWidth,sourceHeight,targetWidth,targetHeight,hq);
  postMessage(r,[r.buffer]);
}
postMessage('ready');
