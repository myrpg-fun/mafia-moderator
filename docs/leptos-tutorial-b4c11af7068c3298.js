let aw=444,ad=1,a5=0,ah=`Object`,a8=`undefined`,ae=3,aj=`auto`,a9=`utf-8`,al=`denied`,ab=`function`,ap=`error`,af=`string`,a6=null,ak=`default`,aq=4,ao=`cors`,av=345,am=`granted`,an=`same-origin`,ag=Array,aa=Error,ai=FinalizationRegistry,at=Object,ax=Object.getPrototypeOf,au=Promise,ar=Reflect,a7=Uint8Array,ac=undefined;var s=((a,b,c)=>{d.closure340_externref_shim(a,b,c)});var r=((a,b,c,e)=>{const f={a:a,b:b,cnt:ad,dtor:c};const g=(...a)=>{f.cnt++;const b=f.a;f.a=a5;try{return e(b,f.b,...a)}finally{if(--f.cnt===a5){d.__wbindgen_export_3.get(f.dtor)(b,f.b);q.unregister(f)}else{f.a=b}}};g.original=f;q.register(g,f,f);return g});var p=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==a6){return `${a}`};if(b==af){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==a6){return `Symbol`}else{return `Symbol(${b})`}};if(b==ab){const b=a.name;if(typeof b==af&&b.length>a5){return `Function(${b})`}else{return `Function`}};if(ag.isArray(a)){const b=a.length;let c=`[`;if(b>a5){c+=p(a[a5])};for(let d=ad;d<b;d++){c+=`, `+ p(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>ad){d=c[ad]}else{return toString.call(a)};if(d==ah){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return ah}};if(a instanceof aa){return `${a.name}: ${a.message}\n${a.stack}`};return d});var o=((a,b)=>{a=a>>>a5;return n.decode(g().subarray(a,a+ b))});var j=((a,b,c)=>{if(c===ac){const c=h.encode(a);const d=b(c.length,ad)>>>a5;g().subarray(d,d+ c.length).set(c);e=c.length;return d};let d=a.length;let f=b(d,ad)>>>a5;const j=g();let k=a5;for(;k<d;k++){const b=a.charCodeAt(k);if(b>127)break;j[f+ k]=b};if(k!==d){if(k!==a5){a=a.slice(k)};f=c(f,d,d=k+ a.length*ae,ad)>>>a5;const b=g().subarray(f+ k,f+ d);const e=i(a,b);k+=e.written;f=c(f,d,k,ad)>>>a5};e=k;return f});var a4=(async(a)=>{if(d!==ac)return d;if(typeof a!==a8){if(ax(a)===at.prototype){({module_or_path:a}=a)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof a===a8){a=new URL(`leptos-tutorial_bg.wasm`,import.meta.url)};const b=a0();if(typeof a===af||typeof Request===ab&&a instanceof Request||typeof URL===ab&&a instanceof URL){a=fetch(a)};a1(b);const {instance:c,module:e}=await $(await a,b);return a2(c,e)});var a1=((a,b)=>{});var y=(a=>d.__wbindgen_export_2.get(a));var w=((a,b,c)=>{d.closure447_externref_shim(a,b,c)});var u=((a,b,c,e)=>{const f={a:a,b:b,cnt:ad,dtor:c};const g=(...a)=>{f.cnt++;try{return e(f.a,f.b,...a)}finally{if(--f.cnt===a5){d.__wbindgen_export_3.get(f.dtor)(f.a,f.b);f.a=a5;q.unregister(f)}}};g.original=f;q.register(g,f,f);return g});var g=(()=>{if(f===a6||f.byteLength===a5){f=new a7(d.memory.buffer)};return f});var D=((a,b,c,e)=>{d.closure511_externref_shim(a,b,c,e)});var m=(()=>{if(l===a6||l.buffer.detached===!0||l.buffer.detached===ac&&l.buffer!==d.memory.buffer){l=new DataView(d.memory.buffer)};return l});var k=(a=>a===ac||a===a6);var C=(a=>()=>{throw new aa(`${a} is not defined`)});var t=((a,b)=>{d._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0bc140932080af6d(a,b)});var z=((a,b)=>{if(a===a5){return y(b)}else{return o(a,b)}});var x=((a,b,c)=>{d.closure478_externref_shim(a,b,c)});var a2=((a,b)=>{d=a.exports;a4.__wbindgen_wasm_module=b;l=a6;f=a6;d.__wbindgen_start();return d});function B(a,b){try{return a.apply(this,b)}catch(a){const b=A(a);d.__wbindgen_exn_store(b)}}var a3=(a=>{if(d!==ac)return d;if(typeof a!==a8){if(ax(a)===at.prototype){({module:a}=a)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const b=a0();a1(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return a2(c,a)});var a0=(()=>{const f={};f.wbg={};f.wbg.__wbindgen_string_get=((a,b)=>{const c=b;const f=typeof c===af?c:ac;var g=k(f)?a5:j(f,d.__wbindgen_malloc,d.__wbindgen_realloc);var h=e;m().setInt32(a+ aq*ad,h,!0);m().setInt32(a+ aq*a5,g,!0)});f.wbg.__wbindgen_cb_drop=(a=>{const b=a.original;if(b.cnt--==ad){b.a=a5;return !0};const c=!1;return c});f.wbg.__wbg_createNewGameLog_90145a93e82cac7f=((a,c)=>{const d=b(a,c!==a5);return d});f.wbg.__wbg_createNewUser_a10836dece60adff=((b,c,d,e)=>{var f=z(b,c);var g=z(d,e);const h=a(f,g);return h});f.wbg.__wbindgen_string_new=((a,b)=>{const c=o(a,b);return c});f.wbg.__wbindgen_number_new=(a=>{const b=a;return b});f.wbg.__wbg_set_f975102236d3c502=((a,b,c)=>{a[b]=c});f.wbg.__wbindgen_is_undefined=(a=>{const b=a===ac;return b});f.wbg.__wbindgen_is_null=(a=>{const b=a===a6;return b});f.wbg.__wbindgen_is_falsy=(a=>{const b=!a;return b});f.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6=((a,b,c)=>{var d=z(b,c);a.innerHTML=d});f.wbg.__wbg_removeAttribute_c80e298b60689065=function(){return B(((a,b,c)=>{var d=z(b,c);a.removeAttribute(d)}),arguments)};f.wbg.__wbg_setAttribute_d5540a19be09f8dc=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);var g=z(d,e);a.setAttribute(f,g)}),arguments)};f.wbg.__wbg_before_ac3792b457802cbf=function(){return B(((a,b)=>{a.before(b)}),arguments)};f.wbg.__wbg_remove_5b68b70c39041e2a=(a=>{a.remove()});f.wbg.__wbg_append_e2fc366a9f0be0e9=function(){return B(((a,b)=>{a.append(b)}),arguments)};f.wbg.__wbg_body_b3bb488e8e54bf4b=(a=>{const b=a.body;return k(b)?a5:A(b)});f.wbg.__wbg_createComment_7a1d9856e50567bb=((a,b,c)=>{var d=z(b,c);const e=a.createComment(d);return e});f.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55=(a=>{const b=a.createDocumentFragment();return b});f.wbg.__wbg_createElement_5921e9eb06b9ec89=function(){return B(((a,b,c)=>{var d=z(b,c);const e=a.createElement(d);return e}),arguments)};f.wbg.__wbg_createTextNode_8bce33cf33bf8f6e=((a,b,c)=>{var d=z(b,c);const e=a.createTextNode(d);return e});f.wbg.__wbg_instanceof_Window_5012736c80a01584=(a=>{let b;try{b=a instanceof Window}catch(a){b=!1}const c=b;return c});f.wbg.__wbg_document_8554450897a855b9=(a=>{const b=a.document;return k(b)?a5:A(b)});f.wbg.__wbg_localStorage_90db5cb66e840248=function(){return B((a=>{const b=a.localStorage;return k(b)?a5:A(b)}),arguments)};f.wbg.__wbg_confirm_8c568ed39db7e399=function(){return B(((a,b,c)=>{var d=z(b,c);const e=a.confirm(d);return e}),arguments)};f.wbg.__wbg_clearInterval_df3409c32c572e85=((a,b)=>{a.clearInterval(b)});f.wbg.__wbg_setInterval_76a7ba11bc095d2d=function(){return B(((a,b,c)=>{const d=a.setInterval(b,c);return d}),arguments)};f.wbg.__wbg_new_95093d1a71aba61d=function(){return B((()=>{const a=new Range();return a}),arguments)};f.wbg.__wbg_deleteContents_45ba9b733b3b97ea=function(){return B((a=>{a.deleteContents()}),arguments)};f.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41=function(){return B(((a,b)=>{a.setEndBefore(b)}),arguments)};f.wbg.__wbg_setStartBefore_a28dcb3c6ece9e07=function(){return B(((a,b)=>{a.setStartBefore(b)}),arguments)};f.wbg.__wbg_target_b7cb1739bee70928=(a=>{const b=a.target;return k(b)?a5:A(b)});f.wbg.__wbg_cancelBubble_0374b329f66f59b5=(a=>{const b=a.cancelBubble;return b});f.wbg.__wbg_composedPath_d1052062308beae5=(a=>{const b=a.composedPath();return b});f.wbg.__wbg_preventDefault_c55d86c27b2dfa6e=(a=>{a.preventDefault()});f.wbg.__wbg_setdata_27c6828c5a5a5ce4=((a,b,c)=>{var d=z(b,c);a.data=d});f.wbg.__wbg_parentNode_3e06cf96d7693d57=(a=>{const b=a.parentNode;return k(b)?a5:A(b)});f.wbg.__wbg_childNodes_031aa96d5e3d21b0=(a=>{const b=a.childNodes;return b});f.wbg.__wbg_previousSibling_076df2178284ef97=(a=>{const b=a.previousSibling;return k(b)?a5:A(b)});f.wbg.__wbg_nextSibling_f6396d6fd0b97830=(a=>{const b=a.nextSibling;return k(b)?a5:A(b)});f.wbg.__wbg_settextContent_cd38ea7d4e0f7260=((a,b,c)=>{var d=z(b,c);a.textContent=d});f.wbg.__wbg_appendChild_ac45d1abddf1b89b=function(){return B(((a,b)=>{const c=a.appendChild(b);return c}),arguments)};f.wbg.__wbg_cloneNode_629a1b180e91c467=function(){return B((a=>{const b=a.cloneNode();return b}),arguments)};f.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c=(a=>{let b;try{b=a instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});f.wbg.__wbg_host_fdfe1258b06fe937=(a=>{const b=a.host;return b});f.wbg.__wbg_view_2a901bda0727aeb3=(a=>{const b=a.view;return k(b)?a5:A(b)});f.wbg.__wbg_respond_a799bab31a44f2d7=function(){return B(((a,b)=>{a.respond(b>>>a5)}),arguments)};f.wbg.__wbg_newwithsrc_4f8c76cff241f93a=function(){return B(((a,b)=>{var c=z(a,b);const d=new Audio(c);return d}),arguments)};f.wbg.__wbg_value_d4a95e7a0d390578=((a,b)=>{const c=b.value;const f=j(c,d.__wbindgen_malloc,d.__wbindgen_realloc);const g=e;m().setInt32(a+ aq*ad,g,!0);m().setInt32(a+ aq*a5,f,!0)});f.wbg.__wbg_getItem_cab39762abab3e70=function(){return B(((a,b,c,f)=>{var g=z(c,f);const h=b.getItem(g);var i=k(h)?a5:j(h,d.__wbindgen_malloc,d.__wbindgen_realloc);var l=e;m().setInt32(a+ aq*ad,l,!0);m().setInt32(a+ aq*a5,i,!0)}),arguments)};f.wbg.__wbg_setItem_9482185c870abba6=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);var g=z(d,e);a.setItem(f,g)}),arguments)};f.wbg.__wbg_append_d510a297e3ba948e=function(){return B(((a,b)=>{a.append(b)}),arguments)};f.wbg.__wbg_byobRequest_b32c77640da946ac=(a=>{const b=a.byobRequest;return k(b)?a5:A(b)});f.wbg.__wbg_close_aca7442e6619206b=function(){return B((a=>{a.close()}),arguments)};f.wbg.__wbg_log_b103404cc5920657=typeof console.log==ab?console.log:C(`console.log`);f.wbg.__wbg_warn_2b3adb99ce26c314=typeof console.warn==ab?console.warn:C(`console.warn`);f.wbg.__wbg_length_4919f4a62b9b1e94=(a=>{const b=a.length;return b});f.wbg.__wbg_addEventListener_e167f012cbedfa4e=function(){return B(((a,b,c,d)=>{var e=z(b,c);a.addEventListener(e,d)}),arguments)};f.wbg.__wbg_addEventListener_14b036ff7cb8747c=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);a.addEventListener(f,d,e)}),arguments)};f.wbg.__wbg_close_cef2400b120c9c73=function(){return B((a=>{a.close()}),arguments)};f.wbg.__wbg_enqueue_6f3d433b5e457aea=function(){return B(((a,b)=>{a.enqueue(b)}),arguments)};f.wbg.__wbg_play_69733added0ad2db=function(){return B((a=>{const b=a.play();return b}),arguments)};f.wbg.__wbg_queueMicrotask_848aa4969108a57e=(a=>{const b=a.queueMicrotask;return b});f.wbg.__wbindgen_is_function=(a=>{const b=typeof a===ab;return b});f.wbg.__wbg_queueMicrotask_c5419c06eab41e73=typeof queueMicrotask==ab?queueMicrotask:C(`queueMicrotask`);f.wbg.__wbg_get_5419cf6b954aa11d=((a,b)=>{const c=a[b>>>a5];return c});f.wbg.__wbg_length_f217bbbf7e8e4df4=(a=>{const b=a.length;return b});f.wbg.__wbg_new_034f913e7636e987=(()=>{const a=new ag();return a});f.wbg.__wbg_valueOf_e90147bfd81601d7=(a=>{const b=a.valueOf();return b});f.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43=((a,b)=>{var c=z(a,b);const d=new Function(c);return d});f.wbg.__wbg_get_ef828680c64da212=function(){return B(((a,b)=>{const c=ar.get(a,b);return c}),arguments)};f.wbg.__wbg_call_a9ef466721e824f2=function(){return B(((a,b)=>{const c=a.call(b);return c}),arguments)};f.wbg.__wbg_new_e69b5f66fda8f13c=(()=>{const a=new at();return a});f.wbg.__wbg_self_bf91bf94d9e04084=function(){return B((()=>{const a=self.self;return a}),arguments)};f.wbg.__wbg_window_52dd9f07d03fd5f8=function(){return B((()=>{const a=window.window;return a}),arguments)};f.wbg.__wbg_globalThis_05c129bf37fcf1be=function(){return B((()=>{const a=globalThis.globalThis;return a}),arguments)};f.wbg.__wbg_global_3eca19bb09e9c484=function(){return B((()=>{const a=global.global;return a}),arguments)};f.wbg.__wbg_set_425e70f7c64ac962=((a,b,c)=>{a[b>>>a5]=c});f.wbg.__wbg_from_91a67a5f04c98a54=(a=>{const b=ag.from(a);return b});f.wbg.__wbg_new_70a2f23d1565c04c=((a,b)=>{var c=z(a,b);const d=new aa(c);return d});f.wbg.__wbg_call_3bfa248576352471=function(){return B(((a,b,c)=>{const d=a.call(b,c);return d}),arguments)};f.wbg.__wbg_is_4b64bc96710d6a0f=((a,b)=>{const c=at.is(a,b);return c});f.wbg.__wbg_new_1073970097e5a420=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=a5;try{return D(d,c.b,a,b)}finally{c.a=d}};const e=new au(d);return e}finally{c.a=c.b=a5}});f.wbg.__wbg_resolve_0aad7c1484731c99=(a=>{const b=au.resolve(a);return b});f.wbg.__wbg_then_748f75edfb032440=((a,b)=>{const c=a.then(b);return c});f.wbg.__wbg_then_4866a7d9f55d8f3e=((a,b,c)=>{const d=a.then(b,c);return d});f.wbg.__wbg_buffer_ccaed51a635d8a2d=(a=>{const b=a.buffer;return b});f.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730=((a,b,c)=>{const d=new a7(a,b>>>a5,c>>>a5);return d});f.wbg.__wbg_set_ec2fcf81bc573fd9=((a,b,c)=>{a.set(b,c>>>a5)});f.wbg.__wbg_length_9254c4bd3b9f23c4=(a=>{const b=a.length;return b});f.wbg.__wbg_buffer_95102df5554646dc=(a=>{const b=a.buffer;return b});f.wbg.__wbg_byteLength_5d623ba3d92a3a9c=(a=>{const b=a.byteLength;return b});f.wbg.__wbg_byteOffset_ec0928143c619cd7=(a=>{const b=a.byteOffset;return b});f.wbg.__wbg_set_e864d25d9b399c9f=function(){return B(((a,b,c)=>{const d=ar.set(a,b,c);return d}),arguments)};f.wbg.__wbindgen_debug_string=((a,b)=>{const c=p(b);const f=j(c,d.__wbindgen_malloc,d.__wbindgen_realloc);const g=e;m().setInt32(a+ aq*ad,g,!0);m().setInt32(a+ aq*a5,f,!0)});f.wbg.__wbindgen_throw=((a,b)=>{throw new aa(o(a,b))});f.wbg.__wbindgen_rethrow=(a=>{throw a});f.wbg.__wbindgen_memory=(()=>{const a=d.memory;return a});f.wbg.__wbindgen_closure_wrapper807=((a,b,c)=>{const d=r(a,b,341,s);return d});f.wbg.__wbindgen_closure_wrapper813=((a,b,c)=>{const d=r(a,b,av,s);return d});f.wbg.__wbindgen_closure_wrapper815=((a,b,c)=>{const d=r(a,b,av,s);return d});f.wbg.__wbindgen_closure_wrapper819=((a,b,c)=>{const d=r(a,b,av,t);return d});f.wbg.__wbindgen_closure_wrapper1146=((a,b,c)=>{const d=u(a,b,aw,v);return d});f.wbg.__wbindgen_closure_wrapper1148=((a,b,c)=>{const d=r(a,b,aw,w);return d});f.wbg.__wbindgen_closure_wrapper2920=((a,b,c)=>{const d=r(a,b,479,x);return d});f.wbg.__wbindgen_init_externref_table=(()=>{const a=d.__wbindgen_export_2;const b=a.grow(aq);a.set(a5,ac);a.set(b+ a5,ac);a.set(b+ ad,a6);a.set(b+ 2,!0);a.set(b+ ae,!1)});f[`./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js`]=c;return f});var v=((a,b)=>{d._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h04a10468e90f4bc2(a,b)});var $=(async(a,b)=>{if(typeof Response===ab&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===ab){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var A=(a=>{const b=d.__externref_table_alloc();d.__wbindgen_export_2.set(b,a);return b});import{createNewUser as a,createNewGameLog as b}from"./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js";import*as c from"./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js";let d;let e=a5;let f=a6;const h=typeof TextEncoder!==a8?new TextEncoder(a9):{encode:()=>{throw aa(`TextEncoder not available`)}};const i=typeof h.encodeInto===ab?((a,b)=>h.encodeInto(a,b)):((a,b)=>{const c=h.encode(a);b.set(c);return {read:a.length,written:c.length}});let l=a6;const n=typeof TextDecoder!==a8?new TextDecoder(a9,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw aa(`TextDecoder not available`)}};if(typeof TextDecoder!==a8){n.decode()};const q=typeof ai===a8?{register:()=>{},unregister:()=>{}}:new ai(a=>{d.__wbindgen_export_3.get(a.dtor)(a.a,a.b)});const E=[`blob`,`arraybuffer`];const F=[aj,`ltr`,`rtl`];const G=[ak,al,am];const H=[am,al,`prompt`];const I=[`byob`];const J=[`bytes`];const K=[``,`no-referrer`,`no-referrer-when-downgrade`,`origin`,`origin-when-cross-origin`,`unsafe-url`,an,`strict-origin`,`strict-origin-when-cross-origin`];const L=[ak,`no-store`,`reload`,`no-cache`,`force-cache`,`only-if-cached`];const M=[`omit`,an,`include`];const N=[an,`no-cors`,ao,`navigate`];const O=[`follow`,ap,`manual`];const P=[`border-box`,`content-box`,`device-pixel-content-box`];const Q=[`basic`,ao,ak,ap,`opaque`,`opaqueredirect`];const R=[aj,`instant`,`smooth`];const S=[`open`,`closed`];const T=[`user`,`environment`,`left`,`right`];const U=[`hidden`,`visible`];const V=typeof ai===a8?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingbytesource_free(a>>>a5,ad));class W{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a5;V.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingbytesource_free(a,a5)}type(){const a=d.intounderlyingbytesource_type(this.__wbg_ptr);var b=z(a[a5],a[ad]);if(a[a5]!==a5){d.__wbindgen_free(a[a5],a[ad],ad)};return b}autoAllocateChunkSize(){const a=d.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return a>>>a5}start(a){d.intounderlyingbytesource_start(this.__wbg_ptr,a)}pull(a){const b=d.intounderlyingbytesource_pull(this.__wbg_ptr,a);return b}cancel(){const a=this.__destroy_into_raw();d.intounderlyingbytesource_cancel(a)}}const X=typeof ai===a8?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingsink_free(a>>>a5,ad));class Y{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a5;X.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingsink_free(a,a5)}write(a){const b=d.intounderlyingsink_write(this.__wbg_ptr,a);return b}close(){const a=this.__destroy_into_raw();const b=d.intounderlyingsink_close(a);return b}abort(a){const b=this.__destroy_into_raw();const c=d.intounderlyingsink_abort(b,a);return c}}const Z=typeof ai===a8?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingsource_free(a>>>a5,ad));class _{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a5;Z.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingsource_free(a,a5)}pull(a){const b=d.intounderlyingsource_pull(this.__wbg_ptr,a);return b}cancel(){const a=this.__destroy_into_raw();d.intounderlyingsource_cancel(a)}}export default a4;export{W as IntoUnderlyingByteSource,Y as IntoUnderlyingSink,_ as IntoUnderlyingSource,a3 as initSync}