let a1=`utf-8`,ai=`cors`,ak=4,a3=null,ad=`auto`,ab=3,ae=`default`,a0=`undefined`,ao=152,a8=1,ag=`granted`,ah=`same-origin`,aj=`error`,a7=`function`,af=`denied`,a9=`Object`,a4=0,a6=`string`,a2=Error,ac=FinalizationRegistry,am=Object,ap=Object.getPrototypeOf,an=Promise,al=Reflect,a5=Uint8Array,aa=undefined;var y=((b,c,d,e)=>{a.closure260_externref_shim(b,c,d,e)});function v(b,c){try{return b.apply(this,c)}catch(b){const c=u(b);a.__wbindgen_exn_store(c)}}var d=(()=>{if(c===a3||c.byteLength===a4){c=new a5(a.memory.buffer)};return c});var u=(b=>{const c=a.__externref_table_alloc();a.__wbindgen_export_2.set(c,b);return c});var q=((b,c,d)=>{a.closure215_externref_shim(b,c,d)});var W=(async(a,b)=>{if(typeof Response===a7&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===a7){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var s=(b=>a.__wbindgen_export_2.get(b));var o=((b,c,d)=>{a.closure151_externref_shim(b,c,d)});var _=(b=>{if(a!==aa)return a;if(typeof b!==a0){if(ap(b)===am.prototype){({module:b}=b)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const c=X();Y(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return Z(d,b)});var r=((b,c,d)=>{a.closure316_externref_shim(b,c,d)});var n=((b,c,d,e)=>{const f={a:b,b:c,cnt:a8,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=a4;try{return e(c,f.b,...b)}finally{if(--f.cnt===a4){a.__wbindgen_export_3.get(f.dtor)(c,f.b);m.unregister(f)}else{f.a=c}}};g.original=f;m.register(g,f,f);return g});var e=((a,c)=>{a=a>>>a4;return b.decode(d().subarray(a,a+ c))});var $=(async(b)=>{if(a!==aa)return a;if(typeof b!==a0){if(ap(b)===am.prototype){({module_or_path:b}=b)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof b===a0){b=new URL(`leptos-tutorial_bg.wasm`,import.meta.url)};const c=X();if(typeof b===a6||typeof Request===a7&&b instanceof Request||typeof URL===a7&&b instanceof URL){b=fetch(b)};Y(c);const {instance:d,module:e}=await W(await b,c);return Z(d,e)});var x=(a=>()=>{throw new a2(`${a} is not defined`)});var X=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_cb_drop=(a=>{const b=a.original;if(b.cnt--==a8){b.a=a4;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=e(a,b);return c});b.wbg.__wbindgen_is_undefined=(a=>{const b=a===aa;return b});b.wbg.__wbindgen_is_null=(a=>{const b=a===a3;return b});b.wbg.__wbindgen_is_falsy=(a=>{const b=!a;return b});b.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6=((a,b,c)=>{var d=t(b,c);a.innerHTML=d});b.wbg.__wbg_removeAttribute_c80e298b60689065=function(){return v(((a,b,c)=>{var d=t(b,c);a.removeAttribute(d)}),arguments)};b.wbg.__wbg_setAttribute_d5540a19be09f8dc=function(){return v(((a,b,c,d,e)=>{var f=t(b,c);var g=t(d,e);a.setAttribute(f,g)}),arguments)};b.wbg.__wbg_before_ac3792b457802cbf=function(){return v(((a,b)=>{a.before(b)}),arguments)};b.wbg.__wbg_remove_5b68b70c39041e2a=(a=>{a.remove()});b.wbg.__wbg_append_e2fc366a9f0be0e9=function(){return v(((a,b)=>{a.append(b)}),arguments)};b.wbg.__wbg_body_b3bb488e8e54bf4b=(a=>{const b=a.body;return w(b)?a4:u(b)});b.wbg.__wbg_createComment_7a1d9856e50567bb=((a,b,c)=>{var d=t(b,c);const e=a.createComment(d);return e});b.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55=(a=>{const b=a.createDocumentFragment();return b});b.wbg.__wbg_createElement_5921e9eb06b9ec89=function(){return v(((a,b,c)=>{var d=t(b,c);const e=a.createElement(d);return e}),arguments)};b.wbg.__wbg_createTextNode_8bce33cf33bf8f6e=((a,b,c)=>{var d=t(b,c);const e=a.createTextNode(d);return e});b.wbg.__wbg_instanceof_Window_5012736c80a01584=(a=>{let b;try{b=a instanceof Window}catch(a){b=!1}const c=b;return c});b.wbg.__wbg_document_8554450897a855b9=(a=>{const b=a.document;return w(b)?a4:u(b)});b.wbg.__wbg_localStorage_90db5cb66e840248=function(){return v((a=>{const b=a.localStorage;return w(b)?a4:u(b)}),arguments)};b.wbg.__wbg_confirm_8c568ed39db7e399=function(){return v(((a,b,c)=>{var d=t(b,c);const e=a.confirm(d);return e}),arguments)};b.wbg.__wbg_new_95093d1a71aba61d=function(){return v((()=>{const a=new Range();return a}),arguments)};b.wbg.__wbg_deleteContents_45ba9b733b3b97ea=function(){return v((a=>{a.deleteContents()}),arguments)};b.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41=function(){return v(((a,b)=>{a.setEndBefore(b)}),arguments)};b.wbg.__wbg_setStartBefore_a28dcb3c6ece9e07=function(){return v(((a,b)=>{a.setStartBefore(b)}),arguments)};b.wbg.__wbg_target_b7cb1739bee70928=(a=>{const b=a.target;return w(b)?a4:u(b)});b.wbg.__wbg_cancelBubble_0374b329f66f59b5=(a=>{const b=a.cancelBubble;return b});b.wbg.__wbg_composedPath_d1052062308beae5=(a=>{const b=a.composedPath();return b});b.wbg.__wbg_preventDefault_c55d86c27b2dfa6e=(a=>{a.preventDefault()});b.wbg.__wbg_setdata_27c6828c5a5a5ce4=((a,b,c)=>{var d=t(b,c);a.data=d});b.wbg.__wbg_parentNode_3e06cf96d7693d57=(a=>{const b=a.parentNode;return w(b)?a4:u(b)});b.wbg.__wbg_childNodes_031aa96d5e3d21b0=(a=>{const b=a.childNodes;return b});b.wbg.__wbg_previousSibling_076df2178284ef97=(a=>{const b=a.previousSibling;return w(b)?a4:u(b)});b.wbg.__wbg_nextSibling_f6396d6fd0b97830=(a=>{const b=a.nextSibling;return w(b)?a4:u(b)});b.wbg.__wbg_settextContent_cd38ea7d4e0f7260=((a,b,c)=>{var d=t(b,c);a.textContent=d});b.wbg.__wbg_appendChild_ac45d1abddf1b89b=function(){return v(((a,b)=>{const c=a.appendChild(b);return c}),arguments)};b.wbg.__wbg_cloneNode_629a1b180e91c467=function(){return v((a=>{const b=a.cloneNode();return b}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c=(a=>{let b;try{b=a instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});b.wbg.__wbg_host_fdfe1258b06fe937=(a=>{const b=a.host;return b});b.wbg.__wbg_view_2a901bda0727aeb3=(a=>{const b=a.view;return w(b)?a4:u(b)});b.wbg.__wbg_respond_a799bab31a44f2d7=function(){return v(((a,b)=>{a.respond(b>>>a4)}),arguments)};b.wbg.__wbg_value_d4a95e7a0d390578=((b,c)=>{const d=c.value;const e=j(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const f=g;l().setInt32(b+ ak*a8,f,!0);l().setInt32(b+ ak*a4,e,!0)});b.wbg.__wbg_getItem_cab39762abab3e70=function(){return v(((b,c,d,e)=>{var f=t(d,e);const h=c.getItem(f);var i=w(h)?a4:j(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var k=g;l().setInt32(b+ ak*a8,k,!0);l().setInt32(b+ ak*a4,i,!0)}),arguments)};b.wbg.__wbg_setItem_9482185c870abba6=function(){return v(((a,b,c,d,e)=>{var f=t(b,c);var g=t(d,e);a.setItem(f,g)}),arguments)};b.wbg.__wbg_append_d510a297e3ba948e=function(){return v(((a,b)=>{a.append(b)}),arguments)};b.wbg.__wbg_byobRequest_b32c77640da946ac=(a=>{const b=a.byobRequest;return w(b)?a4:u(b)});b.wbg.__wbg_close_aca7442e6619206b=function(){return v((a=>{a.close()}),arguments)};b.wbg.__wbg_warn_2b3adb99ce26c314=typeof console.warn==a7?console.warn:x(`console.warn`);b.wbg.__wbg_length_4919f4a62b9b1e94=(a=>{const b=a.length;return b});b.wbg.__wbg_addEventListener_e167f012cbedfa4e=function(){return v(((a,b,c,d)=>{var e=t(b,c);a.addEventListener(e,d)}),arguments)};b.wbg.__wbg_addEventListener_14b036ff7cb8747c=function(){return v(((a,b,c,d,e)=>{var f=t(b,c);a.addEventListener(f,d,e)}),arguments)};b.wbg.__wbg_close_cef2400b120c9c73=function(){return v((a=>{a.close()}),arguments)};b.wbg.__wbg_enqueue_6f3d433b5e457aea=function(){return v(((a,b)=>{a.enqueue(b)}),arguments)};b.wbg.__wbg_get_5419cf6b954aa11d=((a,b)=>{const c=a[b>>>a4];return c});b.wbg.__wbindgen_is_function=(a=>{const b=typeof a===a7;return b});b.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43=((a,b)=>{var c=t(a,b);const d=new Function(c);return d});b.wbg.__wbg_get_ef828680c64da212=function(){return v(((a,b)=>{const c=al.get(a,b);return c}),arguments)};b.wbg.__wbg_call_a9ef466721e824f2=function(){return v(((a,b)=>{const c=a.call(b);return c}),arguments)};b.wbg.__wbg_self_bf91bf94d9e04084=function(){return v((()=>{const a=self.self;return a}),arguments)};b.wbg.__wbg_window_52dd9f07d03fd5f8=function(){return v((()=>{const a=window.window;return a}),arguments)};b.wbg.__wbg_globalThis_05c129bf37fcf1be=function(){return v((()=>{const a=globalThis.globalThis;return a}),arguments)};b.wbg.__wbg_global_3eca19bb09e9c484=function(){return v((()=>{const a=global.global;return a}),arguments)};b.wbg.__wbg_new_70a2f23d1565c04c=((a,b)=>{var c=t(a,b);const d=new a2(c);return d});b.wbg.__wbg_call_3bfa248576352471=function(){return v(((a,b,c)=>{const d=a.call(b,c);return d}),arguments)};b.wbg.__wbg_is_4b64bc96710d6a0f=((a,b)=>{const c=am.is(a,b);return c});b.wbg.__wbg_new_1073970097e5a420=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=a4;try{return y(d,c.b,a,b)}finally{c.a=d}};const e=new an(d);return e}finally{c.a=c.b=a4}});b.wbg.__wbg_resolve_0aad7c1484731c99=(a=>{const b=an.resolve(a);return b});b.wbg.__wbg_then_748f75edfb032440=((a,b)=>{const c=a.then(b);return c});b.wbg.__wbg_buffer_ccaed51a635d8a2d=(a=>{const b=a.buffer;return b});b.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730=((a,b,c)=>{const d=new a5(a,b>>>a4,c>>>a4);return d});b.wbg.__wbg_set_ec2fcf81bc573fd9=((a,b,c)=>{a.set(b,c>>>a4)});b.wbg.__wbg_length_9254c4bd3b9f23c4=(a=>{const b=a.length;return b});b.wbg.__wbg_buffer_95102df5554646dc=(a=>{const b=a.buffer;return b});b.wbg.__wbg_byteLength_5d623ba3d92a3a9c=(a=>{const b=a.byteLength;return b});b.wbg.__wbg_byteOffset_ec0928143c619cd7=(a=>{const b=a.byteOffset;return b});b.wbg.__wbg_set_e864d25d9b399c9f=function(){return v(((a,b,c)=>{const d=al.set(a,b,c);return d}),arguments)};b.wbg.__wbindgen_debug_string=((b,c)=>{const d=f(c);const e=j(d,a.__wbindgen_malloc,a.__wbindgen_realloc);const h=g;l().setInt32(b+ ak*a8,h,!0);l().setInt32(b+ ak*a4,e,!0)});b.wbg.__wbindgen_throw=((a,b)=>{throw new a2(e(a,b))});b.wbg.__wbindgen_rethrow=(a=>{throw a});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return b});b.wbg.__wbg_queueMicrotask_848aa4969108a57e=(a=>{const b=a.queueMicrotask;return b});b.wbg.__wbg_queueMicrotask_c5419c06eab41e73=typeof queueMicrotask==a7?queueMicrotask:x(`queueMicrotask`);b.wbg.__wbindgen_closure_wrapper456=((a,b,c)=>{const d=n(a,b,ao,o);return d});b.wbg.__wbindgen_closure_wrapper458=((a,b,c)=>{const d=n(a,b,ao,p);return d});b.wbg.__wbindgen_closure_wrapper460=((a,b,c)=>{const d=n(a,b,ao,o);return d});b.wbg.__wbindgen_closure_wrapper462=((a,b,c)=>{const d=n(a,b,ao,o);return d});b.wbg.__wbindgen_closure_wrapper656=((a,b,c)=>{const d=n(a,b,216,q);return d});b.wbg.__wbindgen_closure_wrapper3229=((a,b,c)=>{const d=n(a,b,317,r);return d});b.wbg.__wbindgen_init_externref_table=(()=>{const b=a.__wbindgen_export_2;const c=b.grow(ak);b.set(a4,aa);b.set(c+ a4,aa);b.set(c+ a8,a3);b.set(c+ 2,!0);b.set(c+ ab,!1)});return b});var w=(a=>a===aa||a===a3);var f=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==a3){return `${a}`};if(b==a6){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==a3){return `Symbol`}else{return `Symbol(${b})`}};if(b==a7){const b=a.name;if(typeof b==a6&&b.length>a4){return `Function(${b})`}else{return `Function`}};if(Array.isArray(a)){const b=a.length;let c=`[`;if(b>a4){c+=f(a[a4])};for(let d=a8;d<b;d++){c+=`, `+ f(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>a8){d=c[a8]}else{return toString.call(a)};if(d==a9){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return a9}};if(a instanceof a2){return `${a.name}: ${a.message}\n${a.stack}`};return d});var p=((b,c)=>{a._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4d7dcfee5d536029(b,c)});var Z=((b,d)=>{a=b.exports;$.__wbindgen_wasm_module=d;k=a3;c=a3;a.__wbindgen_start();return a});var j=((a,b,c)=>{if(c===aa){const c=h.encode(a);const e=b(c.length,a8)>>>a4;d().subarray(e,e+ c.length).set(c);g=c.length;return e};let e=a.length;let f=b(e,a8)>>>a4;const j=d();let k=a4;for(;k<e;k++){const b=a.charCodeAt(k);if(b>127)break;j[f+ k]=b};if(k!==e){if(k!==a4){a=a.slice(k)};f=c(f,e,e=k+ a.length*ab,a8)>>>a4;const b=d().subarray(f+ k,f+ e);const g=i(a,b);k+=g.written;f=c(f,e,k,a8)>>>a4};g=k;return f});var Y=((a,b)=>{});var t=((a,b)=>{if(a===a4){return s(b)}else{return e(a,b)}});var l=(()=>{if(k===a3||k.buffer.detached===!0||k.buffer.detached===aa&&k.buffer!==a.memory.buffer){k=new DataView(a.memory.buffer)};return k});let a;const b=typeof TextDecoder!==a0?new TextDecoder(a1,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw a2(`TextDecoder not available`)}};if(typeof TextDecoder!==a0){b.decode()};let c=a3;let g=a4;const h=typeof TextEncoder!==a0?new TextEncoder(a1):{encode:()=>{throw a2(`TextEncoder not available`)}};const i=typeof h.encodeInto===a7?((a,b)=>h.encodeInto(a,b)):((a,b)=>{const c=h.encode(a);b.set(c);return {read:a.length,written:c.length}});let k=a3;const m=typeof ac===a0?{register:()=>{},unregister:()=>{}}:new ac(b=>{a.__wbindgen_export_3.get(b.dtor)(b.a,b.b)});const z=[`blob`,`arraybuffer`];const A=[ad,`ltr`,`rtl`];const B=[ae,af,ag];const C=[ag,af,`prompt`];const D=[`byob`];const E=[`bytes`];const F=[``,`no-referrer`,`no-referrer-when-downgrade`,`origin`,`origin-when-cross-origin`,`unsafe-url`,ah,`strict-origin`,`strict-origin-when-cross-origin`];const G=[ae,`no-store`,`reload`,`no-cache`,`force-cache`,`only-if-cached`];const H=[`omit`,ah,`include`];const I=[ah,`no-cors`,ai,`navigate`];const J=[`follow`,aj,`manual`];const K=[`border-box`,`content-box`,`device-pixel-content-box`];const L=[`basic`,ai,ae,aj,`opaque`,`opaqueredirect`];const M=[ad,`instant`,`smooth`];const N=[`open`,`closed`];const O=[`user`,`environment`,`left`,`right`];const P=[`hidden`,`visible`];const Q=typeof ac===a0?{register:()=>{},unregister:()=>{}}:new ac(b=>a.__wbg_intounderlyingbytesource_free(b>>>a4,a8));class R{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a4;Q.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingbytesource_free(b,a4)}type(){const b=a.intounderlyingbytesource_type(this.__wbg_ptr);var c=t(b[a4],b[a8]);if(b[a4]!==a4){a.__wbindgen_free(b[a4],b[a8],a8)};return c}autoAllocateChunkSize(){const b=a.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return b>>>a4}start(b){a.intounderlyingbytesource_start(this.__wbg_ptr,b)}pull(b){const c=a.intounderlyingbytesource_pull(this.__wbg_ptr,b);return c}cancel(){const b=this.__destroy_into_raw();a.intounderlyingbytesource_cancel(b)}}const S=typeof ac===a0?{register:()=>{},unregister:()=>{}}:new ac(b=>a.__wbg_intounderlyingsink_free(b>>>a4,a8));class T{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a4;S.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsink_free(b,a4)}write(b){const c=a.intounderlyingsink_write(this.__wbg_ptr,b);return c}close(){const b=this.__destroy_into_raw();const c=a.intounderlyingsink_close(b);return c}abort(b){const c=this.__destroy_into_raw();const d=a.intounderlyingsink_abort(c,b);return d}}const U=typeof ac===a0?{register:()=>{},unregister:()=>{}}:new ac(b=>a.__wbg_intounderlyingsource_free(b>>>a4,a8));class V{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a4;U.unregister(this);return a}free(){const b=this.__destroy_into_raw();a.__wbg_intounderlyingsource_free(b,a4)}pull(b){const c=a.intounderlyingsource_pull(this.__wbg_ptr,b);return c}cancel(){const b=this.__destroy_into_raw();a.intounderlyingsource_cancel(b)}}export default $;export{R as IntoUnderlyingByteSource,T as IntoUnderlyingSink,V as IntoUnderlyingSource,_ as initSync}