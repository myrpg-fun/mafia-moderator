let a9=0,af=`string`,aj=`auto`,av=429,am=`granted`,aw=509,an=`same-origin`,ak=`default`,ap=`error`,ab=`function`,ad=1,al=`denied`,ah=`Object`,a5=`undefined`,aq=4,ao=`cors`,a8=null,a6=`utf-8`,ae=3,ag=Array,a7=Error,ai=FinalizationRegistry,at=Object,ax=Object.getPrototypeOf,au=Promise,ar=Reflect,aa=Uint8Array,ac=undefined;var p=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==a8){return `${a}`};if(b==af){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==a8){return `Symbol`}else{return `Symbol(${b})`}};if(b==ab){const b=a.name;if(typeof b==af&&b.length>a9){return `Function(${b})`}else{return `Function`}};if(ag.isArray(a)){const b=a.length;let c=`[`;if(b>a9){c+=p(a[a9])};for(let d=ad;d<b;d++){c+=`, `+ p(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>ad){d=c[ad]}else{return toString.call(a)};if(d==ah){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return ah}};if(a instanceof a7){return `${a.name}: ${a.message}\n${a.stack}`};return d});var A=(a=>{const b=d.__externref_table_alloc();d.__wbindgen_export_2.set(b,a);return b});var y=(a=>d.__wbindgen_export_2.get(a));var x=((a,b,c)=>{d.closure543_externref_shim(a,b,c)});var s=((a,b,c)=>{d.closure428_externref_shim(a,b,c)});var g=(()=>{if(f===a8||f.byteLength===a9){f=new aa(d.memory.buffer)};return f});var v=((a,b)=>{d._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h04a10468e90f4bc2(a,b)});var r=((a,b,c,e)=>{const f={a:a,b:b,cnt:ad,dtor:c};const g=(...a)=>{f.cnt++;const b=f.a;f.a=a9;try{return e(b,f.b,...a)}finally{if(--f.cnt===a9){d.__wbindgen_export_3.get(f.dtor)(b,f.b);q.unregister(f)}else{f.a=b}}};g.original=f;q.register(g,f,f);return g});var a1=((a,b)=>{});function B(a,b){try{return a.apply(this,b)}catch(a){const b=A(a);d.__wbindgen_exn_store(b)}}var o=(()=>{if(n===a8||n.buffer.detached===!0||n.buffer.detached===ac&&n.buffer!==d.memory.buffer){n=new DataView(d.memory.buffer)};return n});var h=((a,b)=>{a=a>>>a9;return e.decode(g().subarray(a,a+ b))});var t=((a,b)=>{d._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0bc140932080af6d(a,b)});var m=(a=>a===ac||a===a8);var u=((a,b,c,e)=>{const f={a:a,b:b,cnt:ad,dtor:c};const g=(...a)=>{f.cnt++;try{return e(f.a,f.b,...a)}finally{if(--f.cnt===a9){d.__wbindgen_export_3.get(f.dtor)(f.a,f.b);f.a=a9;q.unregister(f)}}};g.original=f;q.register(g,f,f);return g});var z=((a,b)=>{if(a===a9){return y(b)}else{return h(a,b)}});var l=((a,b,c)=>{if(c===ac){const c=j.encode(a);const d=b(c.length,ad)>>>a9;g().subarray(d,d+ c.length).set(c);i=c.length;return d};let d=a.length;let e=b(d,ad)>>>a9;const f=g();let h=a9;for(;h<d;h++){const b=a.charCodeAt(h);if(b>127)break;f[e+ h]=b};if(h!==d){if(h!==a9){a=a.slice(h)};e=c(e,d,d=h+ a.length*ae,ad)>>>a9;const b=g().subarray(e+ h,e+ d);const f=k(a,b);h+=f.written;e=c(e,d,h,ad)>>>a9};i=h;return e});var a0=(()=>{const e={};e.wbg={};e.wbg.__wbindgen_cb_drop=(a=>{const b=a.original;if(b.cnt--==ad){b.a=a9;return !0};const c=!1;return c});e.wbg.__wbg_createNewGameLog_90145a93e82cac7f=((a,c)=>{const d=b(a,c!==a9);return d});e.wbg.__wbg_createNewUser_a10836dece60adff=((b,c,d,e)=>{var f=z(b,c);var g=z(d,e);const h=a(f,g);return h});e.wbg.__wbindgen_string_new=((a,b)=>{const c=h(a,b);return c});e.wbg.__wbindgen_string_get=((a,b)=>{const c=b;const e=typeof c===af?c:ac;var f=m(e)?a9:l(e,d.__wbindgen_malloc,d.__wbindgen_realloc);var g=i;o().setInt32(a+ aq*ad,g,!0);o().setInt32(a+ aq*a9,f,!0)});e.wbg.__wbindgen_number_new=(a=>{const b=a;return b});e.wbg.__wbg_set_f975102236d3c502=((a,b,c)=>{a[b]=c});e.wbg.__wbindgen_is_undefined=(a=>{const b=a===ac;return b});e.wbg.__wbindgen_is_null=(a=>{const b=a===a8;return b});e.wbg.__wbindgen_is_falsy=(a=>{const b=!a;return b});e.wbg.__wbg_setinnerHTML_ea7e3c6a3c4790c6=((a,b,c)=>{var d=z(b,c);a.innerHTML=d});e.wbg.__wbg_removeAttribute_c80e298b60689065=function(){return B(((a,b,c)=>{var d=z(b,c);a.removeAttribute(d)}),arguments)};e.wbg.__wbg_setAttribute_d5540a19be09f8dc=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);var g=z(d,e);a.setAttribute(f,g)}),arguments)};e.wbg.__wbg_before_ac3792b457802cbf=function(){return B(((a,b)=>{a.before(b)}),arguments)};e.wbg.__wbg_remove_5b68b70c39041e2a=(a=>{a.remove()});e.wbg.__wbg_append_e2fc366a9f0be0e9=function(){return B(((a,b)=>{a.append(b)}),arguments)};e.wbg.__wbg_body_b3bb488e8e54bf4b=(a=>{const b=a.body;return m(b)?a9:A(b)});e.wbg.__wbg_createComment_7a1d9856e50567bb=((a,b,c)=>{var d=z(b,c);const e=a.createComment(d);return e});e.wbg.__wbg_createDocumentFragment_5d919bd9d2e05b55=(a=>{const b=a.createDocumentFragment();return b});e.wbg.__wbg_createElement_5921e9eb06b9ec89=function(){return B(((a,b,c)=>{var d=z(b,c);const e=a.createElement(d);return e}),arguments)};e.wbg.__wbg_createTextNode_8bce33cf33bf8f6e=((a,b,c)=>{var d=z(b,c);const e=a.createTextNode(d);return e});e.wbg.__wbg_instanceof_Window_5012736c80a01584=(a=>{let b;try{b=a instanceof Window}catch(a){b=!1}const c=b;return c});e.wbg.__wbg_document_8554450897a855b9=(a=>{const b=a.document;return m(b)?a9:A(b)});e.wbg.__wbg_localStorage_90db5cb66e840248=function(){return B((a=>{const b=a.localStorage;return m(b)?a9:A(b)}),arguments)};e.wbg.__wbg_confirm_8c568ed39db7e399=function(){return B(((a,b,c)=>{var d=z(b,c);const e=a.confirm(d);return e}),arguments)};e.wbg.__wbg_clearInterval_df3409c32c572e85=((a,b)=>{a.clearInterval(b)});e.wbg.__wbg_setInterval_76a7ba11bc095d2d=function(){return B(((a,b,c)=>{const d=a.setInterval(b,c);return d}),arguments)};e.wbg.__wbg_new_95093d1a71aba61d=function(){return B((()=>{const a=new Range();return a}),arguments)};e.wbg.__wbg_deleteContents_45ba9b733b3b97ea=function(){return B((a=>{a.deleteContents()}),arguments)};e.wbg.__wbg_setEndBefore_7d55a9db0e0f4c41=function(){return B(((a,b)=>{a.setEndBefore(b)}),arguments)};e.wbg.__wbg_setStartBefore_a28dcb3c6ece9e07=function(){return B(((a,b)=>{a.setStartBefore(b)}),arguments)};e.wbg.__wbg_target_b7cb1739bee70928=(a=>{const b=a.target;return m(b)?a9:A(b)});e.wbg.__wbg_cancelBubble_0374b329f66f59b5=(a=>{const b=a.cancelBubble;return b});e.wbg.__wbg_composedPath_d1052062308beae5=(a=>{const b=a.composedPath();return b});e.wbg.__wbg_preventDefault_c55d86c27b2dfa6e=(a=>{a.preventDefault()});e.wbg.__wbg_setdata_27c6828c5a5a5ce4=((a,b,c)=>{var d=z(b,c);a.data=d});e.wbg.__wbg_parentNode_3e06cf96d7693d57=(a=>{const b=a.parentNode;return m(b)?a9:A(b)});e.wbg.__wbg_childNodes_031aa96d5e3d21b0=(a=>{const b=a.childNodes;return b});e.wbg.__wbg_previousSibling_076df2178284ef97=(a=>{const b=a.previousSibling;return m(b)?a9:A(b)});e.wbg.__wbg_nextSibling_f6396d6fd0b97830=(a=>{const b=a.nextSibling;return m(b)?a9:A(b)});e.wbg.__wbg_settextContent_cd38ea7d4e0f7260=((a,b,c)=>{var d=z(b,c);a.textContent=d});e.wbg.__wbg_appendChild_ac45d1abddf1b89b=function(){return B(((a,b)=>{const c=a.appendChild(b);return c}),arguments)};e.wbg.__wbg_cloneNode_629a1b180e91c467=function(){return B((a=>{const b=a.cloneNode();return b}),arguments)};e.wbg.__wbg_instanceof_ShadowRoot_72d8e783f8e0093c=(a=>{let b;try{b=a instanceof ShadowRoot}catch(a){b=!1}const c=b;return c});e.wbg.__wbg_host_fdfe1258b06fe937=(a=>{const b=a.host;return b});e.wbg.__wbg_view_2a901bda0727aeb3=(a=>{const b=a.view;return m(b)?a9:A(b)});e.wbg.__wbg_respond_a799bab31a44f2d7=function(){return B(((a,b)=>{a.respond(b>>>a9)}),arguments)};e.wbg.__wbg_newwithsrc_4f8c76cff241f93a=function(){return B(((a,b)=>{var c=z(a,b);const d=new Audio(c);return d}),arguments)};e.wbg.__wbg_value_d4a95e7a0d390578=((a,b)=>{const c=b.value;const e=l(c,d.__wbindgen_malloc,d.__wbindgen_realloc);const f=i;o().setInt32(a+ aq*ad,f,!0);o().setInt32(a+ aq*a9,e,!0)});e.wbg.__wbg_getItem_cab39762abab3e70=function(){return B(((a,b,c,e)=>{var f=z(c,e);const g=b.getItem(f);var h=m(g)?a9:l(g,d.__wbindgen_malloc,d.__wbindgen_realloc);var j=i;o().setInt32(a+ aq*ad,j,!0);o().setInt32(a+ aq*a9,h,!0)}),arguments)};e.wbg.__wbg_setItem_9482185c870abba6=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);var g=z(d,e);a.setItem(f,g)}),arguments)};e.wbg.__wbg_append_d510a297e3ba948e=function(){return B(((a,b)=>{a.append(b)}),arguments)};e.wbg.__wbg_byobRequest_b32c77640da946ac=(a=>{const b=a.byobRequest;return m(b)?a9:A(b)});e.wbg.__wbg_close_aca7442e6619206b=function(){return B((a=>{a.close()}),arguments)};e.wbg.__wbg_log_b103404cc5920657=typeof console.log==ab?console.log:C(`console.log`);e.wbg.__wbg_warn_2b3adb99ce26c314=typeof console.warn==ab?console.warn:C(`console.warn`);e.wbg.__wbg_length_4919f4a62b9b1e94=(a=>{const b=a.length;return b});e.wbg.__wbg_addEventListener_e167f012cbedfa4e=function(){return B(((a,b,c,d)=>{var e=z(b,c);a.addEventListener(e,d)}),arguments)};e.wbg.__wbg_addEventListener_14b036ff7cb8747c=function(){return B(((a,b,c,d,e)=>{var f=z(b,c);a.addEventListener(f,d,e)}),arguments)};e.wbg.__wbg_close_cef2400b120c9c73=function(){return B((a=>{a.close()}),arguments)};e.wbg.__wbg_enqueue_6f3d433b5e457aea=function(){return B(((a,b)=>{a.enqueue(b)}),arguments)};e.wbg.__wbg_play_69733added0ad2db=function(){return B((a=>{const b=a.play();return b}),arguments)};e.wbg.__wbg_queueMicrotask_848aa4969108a57e=(a=>{const b=a.queueMicrotask;return b});e.wbg.__wbindgen_is_function=(a=>{const b=typeof a===ab;return b});e.wbg.__wbg_queueMicrotask_c5419c06eab41e73=typeof queueMicrotask==ab?queueMicrotask:C(`queueMicrotask`);e.wbg.__wbg_get_5419cf6b954aa11d=((a,b)=>{const c=a[b>>>a9];return c});e.wbg.__wbg_length_f217bbbf7e8e4df4=(a=>{const b=a.length;return b});e.wbg.__wbg_new_034f913e7636e987=(()=>{const a=new ag();return a});e.wbg.__wbg_valueOf_e90147bfd81601d7=(a=>{const b=a.valueOf();return b});e.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43=((a,b)=>{var c=z(a,b);const d=new Function(c);return d});e.wbg.__wbg_get_ef828680c64da212=function(){return B(((a,b)=>{const c=ar.get(a,b);return c}),arguments)};e.wbg.__wbg_call_a9ef466721e824f2=function(){return B(((a,b)=>{const c=a.call(b);return c}),arguments)};e.wbg.__wbg_new_e69b5f66fda8f13c=(()=>{const a=new at();return a});e.wbg.__wbg_self_bf91bf94d9e04084=function(){return B((()=>{const a=self.self;return a}),arguments)};e.wbg.__wbg_window_52dd9f07d03fd5f8=function(){return B((()=>{const a=window.window;return a}),arguments)};e.wbg.__wbg_globalThis_05c129bf37fcf1be=function(){return B((()=>{const a=globalThis.globalThis;return a}),arguments)};e.wbg.__wbg_global_3eca19bb09e9c484=function(){return B((()=>{const a=global.global;return a}),arguments)};e.wbg.__wbg_set_425e70f7c64ac962=((a,b,c)=>{a[b>>>a9]=c});e.wbg.__wbg_from_91a67a5f04c98a54=(a=>{const b=ag.from(a);return b});e.wbg.__wbg_new_70a2f23d1565c04c=((a,b)=>{var c=z(a,b);const d=new a7(c);return d});e.wbg.__wbg_call_3bfa248576352471=function(){return B(((a,b,c)=>{const d=a.call(b,c);return d}),arguments)};e.wbg.__wbg_is_4b64bc96710d6a0f=((a,b)=>{const c=at.is(a,b);return c});e.wbg.__wbg_new_1073970097e5a420=((a,b)=>{try{var c={a:a,b:b};var d=(a,b)=>{const d=c.a;c.a=a9;try{return D(d,c.b,a,b)}finally{c.a=d}};const e=new au(d);return e}finally{c.a=c.b=a9}});e.wbg.__wbg_resolve_0aad7c1484731c99=(a=>{const b=au.resolve(a);return b});e.wbg.__wbg_then_748f75edfb032440=((a,b)=>{const c=a.then(b);return c});e.wbg.__wbg_then_4866a7d9f55d8f3e=((a,b,c)=>{const d=a.then(b,c);return d});e.wbg.__wbg_buffer_ccaed51a635d8a2d=(a=>{const b=a.buffer;return b});e.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730=((a,b,c)=>{const d=new aa(a,b>>>a9,c>>>a9);return d});e.wbg.__wbg_set_ec2fcf81bc573fd9=((a,b,c)=>{a.set(b,c>>>a9)});e.wbg.__wbg_length_9254c4bd3b9f23c4=(a=>{const b=a.length;return b});e.wbg.__wbg_buffer_95102df5554646dc=(a=>{const b=a.buffer;return b});e.wbg.__wbg_byteLength_5d623ba3d92a3a9c=(a=>{const b=a.byteLength;return b});e.wbg.__wbg_byteOffset_ec0928143c619cd7=(a=>{const b=a.byteOffset;return b});e.wbg.__wbg_set_e864d25d9b399c9f=function(){return B(((a,b,c)=>{const d=ar.set(a,b,c);return d}),arguments)};e.wbg.__wbindgen_debug_string=((a,b)=>{const c=p(b);const e=l(c,d.__wbindgen_malloc,d.__wbindgen_realloc);const f=i;o().setInt32(a+ aq*ad,f,!0);o().setInt32(a+ aq*a9,e,!0)});e.wbg.__wbindgen_throw=((a,b)=>{throw new a7(h(a,b))});e.wbg.__wbindgen_rethrow=(a=>{throw a});e.wbg.__wbindgen_memory=(()=>{const a=d.memory;return a});e.wbg.__wbindgen_closure_wrapper988=((a,b,c)=>{const d=r(a,b,av,s);return d});e.wbg.__wbindgen_closure_wrapper990=((a,b,c)=>{const d=r(a,b,av,t);return d});e.wbg.__wbindgen_closure_wrapper992=((a,b,c)=>{const d=r(a,b,av,s);return d});e.wbg.__wbindgen_closure_wrapper994=((a,b,c)=>{const d=r(a,b,av,s);return d});e.wbg.__wbindgen_closure_wrapper1270=((a,b,c)=>{const d=u(a,b,aw,v);return d});e.wbg.__wbindgen_closure_wrapper1272=((a,b,c)=>{const d=r(a,b,aw,w);return d});e.wbg.__wbindgen_closure_wrapper3044=((a,b,c)=>{const d=r(a,b,544,x);return d});e.wbg.__wbindgen_init_externref_table=(()=>{const a=d.__wbindgen_export_2;const b=a.grow(aq);a.set(a9,ac);a.set(b+ a9,ac);a.set(b+ ad,a8);a.set(b+ 2,!0);a.set(b+ ae,!1)});e[`./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js`]=c;return e});var a3=(a=>{if(d!==ac)return d;if(typeof a!==a5){if(ax(a)===at.prototype){({module:a}=a)}else{console.warn(`using deprecated parameters for \`initSync()\`; pass a single object instead`)}};const b=a0();a1(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const c=new WebAssembly.Instance(a,b);return a2(c,a)});var $=(async(a,b)=>{if(typeof Response===ab&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===ab){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve Wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var a4=(async(a)=>{if(d!==ac)return d;if(typeof a!==a5){if(ax(a)===at.prototype){({module_or_path:a}=a)}else{console.warn(`using deprecated parameters for the initialization function; pass a single object instead`)}};if(typeof a===a5){a=new URL(`leptos-tutorial_bg.wasm`,import.meta.url)};const b=a0();if(typeof a===af||typeof Request===ab&&a instanceof Request||typeof URL===ab&&a instanceof URL){a=fetch(a)};a1(b);const {instance:c,module:e}=await $(await a,b);return a2(c,e)});var a2=((a,b)=>{d=a.exports;a4.__wbindgen_wasm_module=b;n=a8;f=a8;d.__wbindgen_start();return d});var D=((a,b,c,e)=>{d.closure576_externref_shim(a,b,c,e)});var w=((a,b,c)=>{d.closure512_externref_shim(a,b,c)});var C=(a=>()=>{throw new a7(`${a} is not defined`)});import{createNewUser as a,createNewGameLog as b}from"./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js";import*as c from"./snippets/leptos-tutorial-263066c41cc99bde/src/js/GoogleSheetsAPI.js";let d;const e=typeof TextDecoder!==a5?new TextDecoder(a6,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw a7(`TextDecoder not available`)}};if(typeof TextDecoder!==a5){e.decode()};let f=a8;let i=a9;const j=typeof TextEncoder!==a5?new TextEncoder(a6):{encode:()=>{throw a7(`TextEncoder not available`)}};const k=typeof j.encodeInto===ab?((a,b)=>j.encodeInto(a,b)):((a,b)=>{const c=j.encode(a);b.set(c);return {read:a.length,written:c.length}});let n=a8;const q=typeof ai===a5?{register:()=>{},unregister:()=>{}}:new ai(a=>{d.__wbindgen_export_3.get(a.dtor)(a.a,a.b)});const E=[`blob`,`arraybuffer`];const F=[aj,`ltr`,`rtl`];const G=[ak,al,am];const H=[am,al,`prompt`];const I=[`byob`];const J=[`bytes`];const K=[``,`no-referrer`,`no-referrer-when-downgrade`,`origin`,`origin-when-cross-origin`,`unsafe-url`,an,`strict-origin`,`strict-origin-when-cross-origin`];const L=[ak,`no-store`,`reload`,`no-cache`,`force-cache`,`only-if-cached`];const M=[`omit`,an,`include`];const N=[an,`no-cors`,ao,`navigate`];const O=[`follow`,ap,`manual`];const P=[`border-box`,`content-box`,`device-pixel-content-box`];const Q=[`basic`,ao,ak,ap,`opaque`,`opaqueredirect`];const R=[aj,`instant`,`smooth`];const S=[`open`,`closed`];const T=[`user`,`environment`,`left`,`right`];const U=[`hidden`,`visible`];const V=typeof ai===a5?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingbytesource_free(a>>>a9,ad));class W{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a9;V.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingbytesource_free(a,a9)}type(){const a=d.intounderlyingbytesource_type(this.__wbg_ptr);var b=z(a[a9],a[ad]);if(a[a9]!==a9){d.__wbindgen_free(a[a9],a[ad],ad)};return b}autoAllocateChunkSize(){const a=d.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);return a>>>a9}start(a){d.intounderlyingbytesource_start(this.__wbg_ptr,a)}pull(a){const b=d.intounderlyingbytesource_pull(this.__wbg_ptr,a);return b}cancel(){const a=this.__destroy_into_raw();d.intounderlyingbytesource_cancel(a)}}const X=typeof ai===a5?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingsink_free(a>>>a9,ad));class Y{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a9;X.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingsink_free(a,a9)}write(a){const b=d.intounderlyingsink_write(this.__wbg_ptr,a);return b}close(){const a=this.__destroy_into_raw();const b=d.intounderlyingsink_close(a);return b}abort(a){const b=this.__destroy_into_raw();const c=d.intounderlyingsink_abort(b,a);return c}}const Z=typeof ai===a5?{register:()=>{},unregister:()=>{}}:new ai(a=>d.__wbg_intounderlyingsource_free(a>>>a9,ad));class _{__destroy_into_raw(){const a=this.__wbg_ptr;this.__wbg_ptr=a9;Z.unregister(this);return a}free(){const a=this.__destroy_into_raw();d.__wbg_intounderlyingsource_free(a,a9)}pull(a){const b=d.intounderlyingsource_pull(this.__wbg_ptr,a);return b}cancel(){const a=this.__destroy_into_raw();d.intounderlyingsource_cancel(a)}}export default a4;export{W as IntoUnderlyingByteSource,Y as IntoUnderlyingSink,_ as IntoUnderlyingSource,a3 as initSync}