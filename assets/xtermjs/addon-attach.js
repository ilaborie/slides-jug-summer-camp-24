!function(e,t){"object"==typeof exports&&"object"==typeof module?module.exports=t():"function"==typeof define&&define.amd?define([],t):"object"==typeof exports?exports.AttachAddon=t():e.AttachAddon=t()}(self,(()=>(()=>{"use strict";var e={};return(()=>{var t=e;function s(e,t,s){return e.addEventListener(t,s),{dispose:()=>{s&&e.removeEventListener(t,s)}}}Object.defineProperty(t,"__esModule",{value:!0}),t.AttachAddon=void 0,t.AttachAddon=class{constructor(e,t){this._disposables=[],this._socket=e,this._socket.binaryType="arraybuffer",this._bidirectional=!(t&&!1===t.bidirectional)}activate(e){this._disposables.push(s(this._socket,"message",(t=>{const s=t.data;e.write("string"==typeof s?s:new Uint8Array(s))}))),this._bidirectional&&(this._disposables.push(e.onData((e=>this._sendData(e)))),this._disposables.push(e.onBinary((e=>this._sendBinary(e))))),this._disposables.push(s(this._socket,"close",(()=>this.dispose()))),this._disposables.push(s(this._socket,"error",(()=>this.dispose())))}dispose(){for(const e of this._disposables)e.dispose()}_sendData(e){this._checkOpenSocket()&&this._socket.send(e)}_sendBinary(e){if(!this._checkOpenSocket())return;const t=new Uint8Array(e.length);for(let s=0;s<e.length;++s)t[s]=255&e.charCodeAt(s);this._socket.send(t)}_checkOpenSocket(){switch(this._socket.readyState){case WebSocket.OPEN:return!0;case WebSocket.CONNECTING:throw new Error("Attach addon was loaded before socket was open");case WebSocket.CLOSING:return console.warn("Attach addon socket is closing"),!1;case WebSocket.CLOSED:throw new Error("Attach addon socket is closed");default:throw new Error("Unexpected socket state")}}}})(),e})()));
//# sourceMappingURL=addon-attach.js.map