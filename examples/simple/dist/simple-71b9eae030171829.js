let O=0,V=`string`,Q=1,W=`Object`,M=`utf-8`,K=null,T=`number`,L=`undefined`,_=170,U=`boolean`,Z=4,R=`function`,Y=16,I=Array,N=Error,X=FinalizationRegistry,S=Int32Array,P=Uint8Array,J=undefined;var C=(async(a,b)=>{if(typeof Response===R&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===R){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==T||b==U||a==K){return `${a}`};if(b==V){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==K){return `Symbol`}else{return `Symbol(${b})`}};if(b==R){const b=a.name;if(typeof b==V&&b.length>O){return `Function(${b})`}else{return `Function`}};if(I.isArray(a)){const b=a.length;let c=`[`;if(b>O){c+=u(a[O])};for(let d=Q;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>Q){d=c[Q]}else{return toString.call(a)};if(d==W){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return W}};if(a instanceof N){return `${a.name}: ${a.message}\n${a.stack}`};return d});var x=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h66f55f3d39bf4336(b,c)});var E=((a,b)=>{});var z=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h8304a21f80c173fe(b,c,k(d))});var k=(a=>{if(d===b.length)b.push(b.length+ Q);const c=d;d=b[c];b[c]=a;return c});var f=(a=>{const b=c(a);e(a);return b});function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var H=(async(b)=>{if(a!==J)return a;if(typeof b===L){b=new URL(`simple-71b9eae030171829_bg.wasm`,import.meta.url)};const c=D();if(typeof b===V||typeof Request===R&&b instanceof Request||typeof URL===R&&b instanceof URL){b=fetch(b)};E(c);const {instance:d,module:e}=await C(await b,c);return F(d,e)});var r=(()=>{if(q===K||q.byteLength===O){q=new S(a.memory.buffer)};return q});var p=(a=>a===J||a===K);var c=(a=>b[a]);var G=(b=>{if(a!==J)return a;const c=D();E(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return F(d,b)});var D=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==Q){b.a=O;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===V?e:J;var g=p(f)?O:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ Q]=h;r()[b/Z+ O]=g});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===T?d:J;t()[a/8+ Q]=p(e)?O:e;r()[a/Z+ O]=!p(e)});b.wbg.__wbg_warn_4221b1eeb4c424c2=((a,b)=>{console.warn(j(a,b))});b.wbg.__wbg_info_71a5de3030bb4c02=((a,b)=>{console.info(j(a,b))});b.wbg.__wbg_debug_8582c7d8fc50b6cf=((a,b)=>{console.debug(j(a,b))});b.wbg.__wbg_trace_cbd26c32e6945675=((a,b)=>{console.trace(j(a,b))});b.wbg.__wbg_error_ca2474234fef2329=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,Q)}});b.wbg.__wbg_new_ad8b60ed55d95a64=(()=>{const a=new N();return k(a)});b.wbg.__wbg_stack_1c80e3b436be887a=((b,d)=>{const e=c(d).stack;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_queueMicrotask_f61ee94ee663068b=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_f82fc5d1e8f816ae=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===R;return b});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===U?(b?Q:O):2;return d});b.wbg.__wbg_instanceof_WebGl2RenderingContext_b1bbc94623ae057f=(a=>{let b;try{b=c(a) instanceof WebGL2RenderingContext}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_bindVertexArray_68196ec68ffa5d9c=((a,b)=>{c(a).bindVertexArray(c(b))});b.wbg.__wbg_bufferData_325ab331c8e0735f=((a,b,d,e)=>{c(a).bufferData(b>>>O,c(d),e>>>O)});b.wbg.__wbg_createVertexArray_aa1c03bf14f520f1=(a=>{const b=c(a).createVertexArray();return p(b)?O:k(b)});b.wbg.__wbg_texImage2D_480bb656acf1c931=function(){return B(((a,b,d,e,f,g,h,i,j,k)=>{c(a).texImage2D(b>>>O,d,e,f,g,h,i>>>O,j>>>O,c(k))}),arguments)};b.wbg.__wbg_texSubImage2D_01a3d1b8ac6e4702=function(){return B(((a,b,d,e,f,g,h,i,j,k)=>{c(a).texSubImage2D(b>>>O,d,e,f,g,h,i>>>O,j>>>O,c(k))}),arguments)};b.wbg.__wbg_texSubImage2D_c96538f70e56e139=function(){return B(((a,b,d,e,f,g,h,i,j,k)=>{c(a).texSubImage2D(b>>>O,d,e,f,g,h,i>>>O,j>>>O,k)}),arguments)};b.wbg.__wbg_activeTexture_5f08f4188abc9410=((a,b)=>{c(a).activeTexture(b>>>O)});b.wbg.__wbg_attachShader_427e1d01a628e522=((a,b,d)=>{c(a).attachShader(c(b),c(d))});b.wbg.__wbg_bindBuffer_0285be79ac8a4f9f=((a,b,d)=>{c(a).bindBuffer(b>>>O,c(d))});b.wbg.__wbg_bindTexture_20fd21916229a80c=((a,b,d)=>{c(a).bindTexture(b>>>O,c(d))});b.wbg.__wbg_blendEquationSeparate_46b6aa4bdcf2d9ef=((a,b,d)=>{c(a).blendEquationSeparate(b>>>O,d>>>O)});b.wbg.__wbg_blendFuncSeparate_0490d8414b0a2e98=((a,b,d,e,f)=>{c(a).blendFuncSeparate(b>>>O,d>>>O,e>>>O,f>>>O)});b.wbg.__wbg_clear_ceb93ecc4e5d5e06=((a,b)=>{c(a).clear(b>>>O)});b.wbg.__wbg_clearColor_e4c61a3089043306=((a,b,d,e,f)=>{c(a).clearColor(b,d,e,f)});b.wbg.__wbg_colorMask_ae97e907cdf41404=((a,b,d,e,f)=>{c(a).colorMask(b!==O,d!==O,e!==O,f!==O)});b.wbg.__wbg_compileShader_2191687ded033138=((a,b)=>{c(a).compileShader(c(b))});b.wbg.__wbg_createBuffer_ee6e74ae50f1fbc8=(a=>{const b=c(a).createBuffer();return p(b)?O:k(b)});b.wbg.__wbg_createProgram_869004e7019cca34=(a=>{const b=c(a).createProgram();return p(b)?O:k(b)});b.wbg.__wbg_createShader_eceb4217c94a1056=((a,b)=>{const d=c(a).createShader(b>>>O);return p(d)?O:k(d)});b.wbg.__wbg_createTexture_084c9c8b377793c3=(a=>{const b=c(a).createTexture();return p(b)?O:k(b)});b.wbg.__wbg_deleteBuffer_c66740fb719fb9a7=((a,b)=>{c(a).deleteBuffer(c(b))});b.wbg.__wbg_deleteProgram_af78790615a2a7d7=((a,b)=>{c(a).deleteProgram(c(b))});b.wbg.__wbg_deleteShader_1d9cfbdc150762cb=((a,b)=>{c(a).deleteShader(c(b))});b.wbg.__wbg_deleteTexture_547a7269a7254d2a=((a,b)=>{c(a).deleteTexture(c(b))});b.wbg.__wbg_detachShader_83214b1314cfc677=((a,b,d)=>{c(a).detachShader(c(b),c(d))});b.wbg.__wbg_disable_3598de08841268c2=((a,b)=>{c(a).disable(b>>>O)});b.wbg.__wbg_disableVertexAttribArray_2550e62f2714837b=((a,b)=>{c(a).disableVertexAttribArray(b>>>O)});b.wbg.__wbg_drawElements_77b947be75fe30f4=((a,b,d,e,f)=>{c(a).drawElements(b>>>O,d,e>>>O,f)});b.wbg.__wbg_enable_98a8863abbaa7bd2=((a,b)=>{c(a).enable(b>>>O)});b.wbg.__wbg_enableVertexAttribArray_5f8e190ba41f4f30=((a,b)=>{c(a).enableVertexAttribArray(b>>>O)});b.wbg.__wbg_getAttribLocation_08f436d8fc4fe68d=((a,b,d,e)=>{const f=c(a).getAttribLocation(c(b),j(d,e));return f});b.wbg.__wbg_getError_4f4b3a8c71d4c39e=(a=>{const b=c(a).getError();return b});b.wbg.__wbg_getExtension_e64ba7e30e8a6eae=function(){return B(((a,b,d)=>{const e=c(a).getExtension(j(b,d));return p(e)?O:k(e)}),arguments)};b.wbg.__wbg_getParameter_4d51a40deebd7a8c=function(){return B(((a,b)=>{const d=c(a).getParameter(b>>>O);return k(d)}),arguments)};b.wbg.__wbg_getProgramInfoLog_a9fede9be1e3a6ce=((b,d,e)=>{const f=c(d).getProgramInfoLog(c(e));var g=p(f)?O:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ Q]=h;r()[b/Z+ O]=g});b.wbg.__wbg_getProgramParameter_a13e6d88ec9e039a=((a,b,d)=>{const e=c(a).getProgramParameter(c(b),d>>>O);return k(e)});b.wbg.__wbg_getShaderInfoLog_63c64bf03382de2d=((b,d,e)=>{const f=c(d).getShaderInfoLog(c(e));var g=p(f)?O:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ Q]=h;r()[b/Z+ O]=g});b.wbg.__wbg_getShaderParameter_0fb2d525889d5a24=((a,b,d)=>{const e=c(a).getShaderParameter(c(b),d>>>O);return k(e)});b.wbg.__wbg_getSupportedExtensions_3d93fae4cd24d995=(a=>{const b=c(a).getSupportedExtensions();return p(b)?O:k(b)});b.wbg.__wbg_getUniformLocation_009db1591e93ef17=((a,b,d,e)=>{const f=c(a).getUniformLocation(c(b),j(d,e));return p(f)?O:k(f)});b.wbg.__wbg_linkProgram_578651eb0388616a=((a,b)=>{c(a).linkProgram(c(b))});b.wbg.__wbg_pixelStorei_8848825419f61cec=((a,b,d)=>{c(a).pixelStorei(b>>>O,d)});b.wbg.__wbg_scissor_b2312f494abc5032=((a,b,d,e,f)=>{c(a).scissor(b,d,e,f)});b.wbg.__wbg_shaderSource_d241264f221df907=((a,b,d,e)=>{c(a).shaderSource(c(b),j(d,e))});b.wbg.__wbg_texParameteri_b747d8d506fcd0d2=((a,b,d,e)=>{c(a).texParameteri(b>>>O,d>>>O,e)});b.wbg.__wbg_uniform1i_3f1af04af82891ff=((a,b,d)=>{c(a).uniform1i(c(b),d)});b.wbg.__wbg_uniform2f_cf18347e12a5f103=((a,b,d,e)=>{c(a).uniform2f(c(b),d,e)});b.wbg.__wbg_useProgram_b28955d541019a7a=((a,b)=>{c(a).useProgram(c(b))});b.wbg.__wbg_vertexAttribPointer_df897e1c10d6b71b=((a,b,d,e,f,g,h)=>{c(a).vertexAttribPointer(b>>>O,d,e>>>O,f!==O,g,h)});b.wbg.__wbg_viewport_f542dcd30d88e69d=((a,b,d,e,f)=>{c(a).viewport(b,d,e,f)});b.wbg.__wbg_instanceof_Window_cee7a886d55e7df5=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_eb7fd66bde3ee213=(a=>{const b=c(a).document;return p(b)?O:k(b)});b.wbg.__wbg_location_b17760ac7977a47a=(a=>{const b=c(a).location;return k(b)});b.wbg.__wbg_navigator_b1003b77e05fcee9=(a=>{const b=c(a).navigator;return k(b)});b.wbg.__wbg_innerHeight_a9719febb72ddaf3=function(){return B((a=>{const b=c(a).innerHeight;return k(b)}),arguments)};b.wbg.__wbg_devicePixelRatio_3ced5021c4480dd9=(a=>{const b=c(a).devicePixelRatio;return b});b.wbg.__wbg_speechSynthesis_7840ff17f187ef7a=function(){return B((a=>{const b=c(a).speechSynthesis;return k(b)}),arguments)};b.wbg.__wbg_localStorage_3d538af21ea07fcc=function(){return B((a=>{const b=c(a).localStorage;return p(b)?O:k(b)}),arguments)};b.wbg.__wbg_performance_4ca1873776fdb3d2=(a=>{const b=c(a).performance;return p(b)?O:k(b)});b.wbg.__wbg_matchMedia_d9cdff718d3e526e=function(){return B(((a,b,d)=>{const e=c(a).matchMedia(j(b,d));return p(e)?O:k(e)}),arguments)};b.wbg.__wbg_open_8db78e14935e215b=function(){return B(((a,b,d,e,f)=>{const g=c(a).open(j(b,d),j(e,f));return p(g)?O:k(g)}),arguments)};b.wbg.__wbg_requestAnimationFrame_fdbeaff9e8f3f77d=function(){return B(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_clearInterval_0216ca540cb1ad00=((a,b)=>{c(a).clearInterval(b)});b.wbg.__wbg_setTimeout_6ed7182ebad5d297=function(){return B(((a,b,d)=>{const e=c(a).setTimeout(c(b),d);return e}),arguments)};b.wbg.__wbg_body_874ccb42daaab363=(a=>{const b=c(a).body;return p(b)?O:k(b)});b.wbg.__wbg_createElement_03cf347ddad1c8c0=function(){return B(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_getElementById_77f2dfdddee12e05=((a,b,d)=>{const e=c(a).getElementById(j(b,d));return p(e)?O:k(e)});b.wbg.__wbg_setid_7daec2ce740ea365=((a,b,d)=>{c(a).id=j(b,d)});b.wbg.__wbg_scrollLeft_c2f39ef7a0b36f55=(a=>{const b=c(a).scrollLeft;return b});b.wbg.__wbg_clientWidth_7a325bdb8c723d9f=(a=>{const b=c(a).clientWidth;return b});b.wbg.__wbg_clientHeight_2b2a9874084502db=(a=>{const b=c(a).clientHeight;return b});b.wbg.__wbg_getBoundingClientRect_3b6c47996a55427e=(a=>{const b=c(a).getBoundingClientRect();return k(b)});b.wbg.__wbg_scrollTop_6f07539d4534a820=(a=>{const b=c(a).scrollTop;return b});b.wbg.__wbg_hidden_2366c29a55a4c50a=(a=>{const b=c(a).hidden;return b});b.wbg.__wbg_sethidden_04acac3815d1ba64=((a,b)=>{c(a).hidden=b!==O});b.wbg.__wbg_style_ca229e3326b3c3fb=(a=>{const b=c(a).style;return k(b)});b.wbg.__wbg_offsetTop_0636b250f8d731f3=(a=>{const b=c(a).offsetTop;return b});b.wbg.__wbg_offsetLeft_0150ee05891dfb7e=(a=>{const b=c(a).offsetLeft;return b});b.wbg.__wbg_offsetWidth_b5af4d8ba15fa071=(a=>{const b=c(a).offsetWidth;return b});b.wbg.__wbg_blur_3bef2a6e3b1f9734=function(){return B((a=>{c(a).blur()}),arguments)};b.wbg.__wbg_focus_d1373017540aae66=function(){return B((a=>{c(a).focus()}),arguments)};b.wbg.__wbg_instanceof_WebGlRenderingContext_468d6182819ad4c3=(a=>{let b;try{b=c(a) instanceof WebGLRenderingContext}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_bufferData_560eedbff09bce24=((a,b,d,e)=>{c(a).bufferData(b>>>O,c(d),e>>>O)});b.wbg.__wbg_texImage2D_a907a9e673209a09=function(){return B(((a,b,d,e,f,g,h,i,j,k)=>{c(a).texImage2D(b>>>O,d,e,f,g,h,i>>>O,j>>>O,c(k))}),arguments)};b.wbg.__wbg_texSubImage2D_ace8ef3e5beb9c66=function(){return B(((a,b,d,e,f,g,h,i,j,k)=>{c(a).texSubImage2D(b>>>O,d,e,f,g,h,i>>>O,j>>>O,c(k))}),arguments)};b.wbg.__wbg_activeTexture_5d70c5bfb1e18433=((a,b)=>{c(a).activeTexture(b>>>O)});b.wbg.__wbg_attachShader_877aa4ad5f81f5fb=((a,b,d)=>{c(a).attachShader(c(b),c(d))});b.wbg.__wbg_bindBuffer_8721bd9c00cbc8b8=((a,b,d)=>{c(a).bindBuffer(b>>>O,c(d))});b.wbg.__wbg_bindTexture_17a55d9204f92347=((a,b,d)=>{c(a).bindTexture(b>>>O,c(d))});b.wbg.__wbg_blendEquationSeparate_483c9bbff12702e9=((a,b,d)=>{c(a).blendEquationSeparate(b>>>O,d>>>O)});b.wbg.__wbg_blendFuncSeparate_2f3d44b3bd3604e9=((a,b,d,e,f)=>{c(a).blendFuncSeparate(b>>>O,d>>>O,e>>>O,f>>>O)});b.wbg.__wbg_clear_617b292bb8360c4a=((a,b)=>{c(a).clear(b>>>O)});b.wbg.__wbg_clearColor_6d87cdad9936f445=((a,b,d,e,f)=>{c(a).clearColor(b,d,e,f)});b.wbg.__wbg_colorMask_d8a9ecfb82a480cf=((a,b,d,e,f)=>{c(a).colorMask(b!==O,d!==O,e!==O,f!==O)});b.wbg.__wbg_compileShader_349b2f1607e1b7e9=((a,b)=>{c(a).compileShader(c(b))});b.wbg.__wbg_createBuffer_c48fee40bffd1848=(a=>{const b=c(a).createBuffer();return p(b)?O:k(b)});b.wbg.__wbg_createProgram_eb0e7dfb7c89e9b8=(a=>{const b=c(a).createProgram();return p(b)?O:k(b)});b.wbg.__wbg_createShader_6b7a22e75c6d4cc4=((a,b)=>{const d=c(a).createShader(b>>>O);return p(d)?O:k(d)});b.wbg.__wbg_createTexture_af9c1894db4f1ff4=(a=>{const b=c(a).createTexture();return p(b)?O:k(b)});b.wbg.__wbg_deleteBuffer_0788cfe1724454e7=((a,b)=>{c(a).deleteBuffer(c(b))});b.wbg.__wbg_deleteProgram_655d072ee71efb0c=((a,b)=>{c(a).deleteProgram(c(b))});b.wbg.__wbg_deleteShader_13c4ae9a9c93c31f=((a,b)=>{c(a).deleteShader(c(b))});b.wbg.__wbg_deleteTexture_38664338ad2770e7=((a,b)=>{c(a).deleteTexture(c(b))});b.wbg.__wbg_detachShader_480c7fff77236016=((a,b,d)=>{c(a).detachShader(c(b),c(d))});b.wbg.__wbg_disable_d9f43aa105b2d999=((a,b)=>{c(a).disable(b>>>O)});b.wbg.__wbg_disableVertexAttribArray_415331ebeb20bf62=((a,b)=>{c(a).disableVertexAttribArray(b>>>O)});b.wbg.__wbg_drawElements_201a313e5ea8a1c4=((a,b,d,e,f)=>{c(a).drawElements(b>>>O,d,e>>>O,f)});b.wbg.__wbg_enable_f6bb861e15562c7d=((a,b)=>{c(a).enable(b>>>O)});b.wbg.__wbg_enableVertexAttribArray_1e31054271daee48=((a,b)=>{c(a).enableVertexAttribArray(b>>>O)});b.wbg.__wbg_getAttribLocation_5394f71d757b6847=((a,b,d,e)=>{const f=c(a).getAttribLocation(c(b),j(d,e));return f});b.wbg.__wbg_getError_418a1642da6ca6b5=(a=>{const b=c(a).getError();return b});b.wbg.__wbg_getExtension_695813f4264a3da5=function(){return B(((a,b,d)=>{const e=c(a).getExtension(j(b,d));return p(e)?O:k(e)}),arguments)};b.wbg.__wbg_getParameter_d866a888cb0448b3=function(){return B(((a,b)=>{const d=c(a).getParameter(b>>>O);return k(d)}),arguments)};b.wbg.__wbg_getProgramInfoLog_5bbd3c3298d235e3=((b,d,e)=>{const f=c(d).getProgramInfoLog(c(e));var g=p(f)?O:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ Q]=h;r()[b/Z+ O]=g});b.wbg.__wbg_getProgramParameter_9f11dae51c820ff9=((a,b,d)=>{const e=c(a).getProgramParameter(c(b),d>>>O);return k(e)});b.wbg.__wbg_getShaderInfoLog_9534d164ba660552=((b,d,e)=>{const f=c(d).getShaderInfoLog(c(e));var g=p(f)?O:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/Z+ Q]=h;r()[b/Z+ O]=g});b.wbg.__wbg_getShaderParameter_0cfb9e3e9f43693a=((a,b,d)=>{const e=c(a).getShaderParameter(c(b),d>>>O);return k(e)});b.wbg.__wbg_getSupportedExtensions_69c26850565e0ddc=(a=>{const b=c(a).getSupportedExtensions();return p(b)?O:k(b)});b.wbg.__wbg_getUniformLocation_2890393c80bc543b=((a,b,d,e)=>{const f=c(a).getUniformLocation(c(b),j(d,e));return p(f)?O:k(f)});b.wbg.__wbg_linkProgram_5bd18d4ebd77a2ea=((a,b)=>{c(a).linkProgram(c(b))});b.wbg.__wbg_pixelStorei_98d826c8b851ed4f=((a,b,d)=>{c(a).pixelStorei(b>>>O,d)});b.wbg.__wbg_scissor_1a003e68fc69d37d=((a,b,d,e,f)=>{c(a).scissor(b,d,e,f)});b.wbg.__wbg_shaderSource_17ebc5d747730d79=((a,b,d,e)=>{c(a).shaderSource(c(b),j(d,e))});b.wbg.__wbg_texParameteri_ee2fc6b9f304c84a=((a,b,d,e)=>{c(a).texParameteri(b>>>O,d>>>O,e)});b.wbg.__wbg_uniform1i_76ac1d17923cb752=((a,b,d)=>{c(a).uniform1i(c(b),d)});b.wbg.__wbg_uniform2f_766ce5b04d89a3d8=((a,b,d,e)=>{c(a).uniform2f(c(b),d,e)});b.wbg.__wbg_useProgram_1532661e648379ca=((a,b)=>{c(a).useProgram(c(b))});b.wbg.__wbg_vertexAttribPointer_88b31f05ae55b02c=((a,b,d,e,f,g,h)=>{c(a).vertexAttribPointer(b>>>O,d,e>>>O,f!==O,g,h)});b.wbg.__wbg_viewport_037ea26b4fcd0cb2=((a,b,d,e,f)=>{c(a).viewport(b,d,e,f)});b.wbg.__wbg_instanceof_HtmlInputElement_189f182751dc1f5e=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_setautofocus_3ac9b87146e5cd21=((a,b)=>{c(a).autofocus=b!==O});b.wbg.__wbg_setsize_c0546cf8c51a0e77=((a,b)=>{c(a).size=b>>>O});b.wbg.__wbg_value_99f5294791d62576=((b,d)=>{const e=c(d).value;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_setvalue_bba31de32cdbb32c=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_setProperty_5144ddce66bbde41=function(){return B(((a,b,d,e,f)=>{c(a).setProperty(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_top_44a8c250dcea0251=(a=>{const b=c(a).top;return b});b.wbg.__wbg_left_52377628791ffcf6=(a=>{const b=c(a).left;return b});b.wbg.__wbg_matches_b18b6193e5512cde=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_cancel_162d845e80e9e115=(a=>{c(a).cancel()});b.wbg.__wbg_speak_eee36fad7b12bb07=((a,b)=>{c(a).speak(c(b))});b.wbg.__wbg_deltaX_5ddc4c69f2887db9=(a=>{const b=c(a).deltaX;return b});b.wbg.__wbg_deltaY_0ba2dcd707862292=(a=>{const b=c(a).deltaY;return b});b.wbg.__wbg_deltaMode_ed2d0b2e0a547b92=(a=>{const b=c(a).deltaMode;return b});b.wbg.__wbg_length_e5ff7777627bc19e=(a=>{const b=c(a).length;return b});b.wbg.__wbg_get_0aa6219f7b9f2100=((a,b)=>{const d=c(a)[b>>>O];return p(d)?O:k(d)});b.wbg.__wbg_preventDefault_657cbf753df1396c=(a=>{c(a).preventDefault()});b.wbg.__wbg_stopPropagation_806520d93e80bcf7=(a=>{c(a).stopPropagation()});b.wbg.__wbg_userAgent_6dbd12d01f988f5f=function(){return B(((b,d)=>{const e=c(d).userAgent;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_bindVertexArrayOES_fd441111fdf91a30=((a,b)=>{c(a).bindVertexArrayOES(c(b))});b.wbg.__wbg_createVertexArrayOES_51189386ff9903a2=(a=>{const b=c(a).createVertexArrayOES();return p(b)?O:k(b)});b.wbg.__wbg_identifier_1bbeeef4f8c3644c=(a=>{const b=c(a).identifier;return b});b.wbg.__wbg_pageX_1a662c5236b65a8c=(a=>{const b=c(a).pageX;return b});b.wbg.__wbg_pageY_7d28263e3ca120f8=(a=>{const b=c(a).pageY;return b});b.wbg.__wbg_force_a23dc68752bbd295=(a=>{const b=c(a).force;return b});b.wbg.__wbg_matches_f58a5f0b7cbb150c=(a=>{const b=c(a).matches;return b});b.wbg.__wbg_now_ef71656beb948bc8=(a=>{const b=c(a).now();return b});b.wbg.__wbg_length_82b5ad246042df8b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_item_f29aac628e55f885=((a,b)=>{const d=c(a).item(b>>>O);return p(d)?O:k(d)});b.wbg.__wbg_get_1a995928c199f987=((a,b)=>{const d=c(a)[b>>>O];return p(d)?O:k(d)});b.wbg.__wbg_type_023f810f636fd950=((b,d)=>{const e=c(d).type;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_width_faa64c8759fdff80=(a=>{const b=c(a).width;return b});b.wbg.__wbg_height_bdf3f02c617ef055=(a=>{const b=c(a).height;return b});b.wbg.__wbg_length_29ed7aaf6bb5a432=(a=>{const b=c(a).length;return b});b.wbg.__wbg_get_a5a83ff42873c81d=((a,b)=>{const d=c(a)[b>>>O];return p(d)?O:k(d)});b.wbg.__wbg_clientX_2d1be024f35f3981=(a=>{const b=c(a).clientX;return b});b.wbg.__wbg_clientY_967af1c2b0177a9f=(a=>{const b=c(a).clientY;return b});b.wbg.__wbg_ctrlKey_2817108375a63e7c=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_87d2a9527cefdf3d=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_metaKey_35f1fd33c4e0c5df=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_button_43d11b000a7126cf=(a=>{const b=c(a).button;return b});b.wbg.__wbg_getItem_5c179cd36e9529e8=function(){return B(((b,d,e,f)=>{const g=c(d).getItem(j(e,f));var h=p(g)?O:o(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var i=l;r()[b/Z+ Q]=i;r()[b/Z+ O]=h}),arguments)};b.wbg.__wbg_setItem_7b55989efb4d45f7=function(){return B(((a,b,d,e,f)=>{c(a).setItem(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_items_622c348119794691=(a=>{const b=c(a).items;return k(b)});b.wbg.__wbg_files_00ec146c4eb7c254=(a=>{const b=c(a).files;return p(b)?O:k(b)});b.wbg.__wbg_href_a5b902312c18d121=function(){return B(((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_origin_305402044aa148ce=function(){return B(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_protocol_85fb404fceb30ff2=function(){return B(((b,d)=>{const e=c(d).protocol;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_host_3f37d9558f3919b9=function(){return B(((b,d)=>{const e=c(d).host;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_hostname_0a8eb31c4e0261e9=function(){return B(((b,d)=>{const e=c(d).hostname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_port_be2aeb97d2bf49c7=function(){return B(((b,d)=>{const e=c(d).port;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_search_40927d5af164fdfe=function(){return B(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_hash_163703b5971e593c=function(){return B(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f}),arguments)};b.wbg.__wbg_touches_156ae51fb3534e87=(a=>{const b=c(a).touches;return k(b)});b.wbg.__wbg_changedTouches_2a84bde1dd7ea44a=(a=>{const b=c(a).changedTouches;return k(b)});b.wbg.__wbg_size_97217f6c840f58b2=(a=>{const b=c(a).size;return b});b.wbg.__wbg_type_6c31bd72c3383cba=((b,d)=>{const e=c(d).type;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_arrayBuffer_800ebf8dca614366=(a=>{const b=c(a).arrayBuffer();return k(b)});b.wbg.__wbg_addEventListener_f984e99465a6a7f4=function(){return B(((a,b,d,e)=>{c(a).addEventListener(j(b,d),c(e))}),arguments)};b.wbg.__wbg_removeEventListener_acfc154b998d806b=function(){return B(((a,b,d,e)=>{c(a).removeEventListener(j(b,d),c(e))}),arguments)};b.wbg.__wbg_instanceof_HtmlCanvasElement_1e81f71f630e46bc=(a=>{let b;try{b=c(a) instanceof HTMLCanvasElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_width_aa1ac55fb41db6ae=(a=>{const b=c(a).width;return b});b.wbg.__wbg_setwidth_233645b297bb3318=((a,b)=>{c(a).width=b>>>O});b.wbg.__wbg_height_bea901cd16645fb7=(a=>{const b=c(a).height;return b});b.wbg.__wbg_setheight_fcb491cf54e3527c=((a,b)=>{c(a).height=b>>>O});b.wbg.__wbg_getContext_dfc91ab0837db1d1=function(){return B(((a,b,d)=>{const e=c(a).getContext(j(b,d));return p(e)?O:k(e)}),arguments)};b.wbg.__wbg_keyCode_8c7511bf92389868=(a=>{const b=c(a).keyCode;return b});b.wbg.__wbg_altKey_580c95fbc9461164=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_ctrlKey_032bd6905bacba55=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_a84ea8856781bd54=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_metaKey_fe405998712e46a0=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_isComposing_750f53009cbfb833=(a=>{const b=c(a).isComposing;return b});b.wbg.__wbg_key_0527970a852413ca=((b,d)=>{const e=c(d).key;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_setvolume_f099e05c57d52daf=((a,b)=>{c(a).volume=b});b.wbg.__wbg_setrate_001282e5d4db3fa8=((a,b)=>{c(a).rate=b});b.wbg.__wbg_setpitch_0c37156ed77bab7a=((a,b)=>{c(a).pitch=b});b.wbg.__wbg_newwithtext_72d67a690afe4bfd=function(){return B(((a,b)=>{const c=new SpeechSynthesisUtterance(j(a,b));return k(c)}),arguments)};b.wbg.__wbg_dataTransfer_c3dfe779ef079bcc=(a=>{const b=c(a).dataTransfer;return p(b)?O:k(b)});b.wbg.__wbg_data_bb29dff4a6557791=((b,d)=>{const e=c(d).data;var f=p(e)?O:o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_name_9762a5bb951e00c1=((b,d)=>{const e=c(d).name;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbg_lastModified_679a43283963f658=(a=>{const b=c(a).lastModified;return b});b.wbg.__wbg_parentElement_45a9756dc74ff48b=(a=>{const b=c(a).parentElement;return p(b)?O:k(b)});b.wbg.__wbg_appendChild_4153ba1b5d54d73b=function(){return B(((a,b)=>{const d=c(a).appendChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_performance_eeefc685c9bc38b4=(a=>{const b=c(a).performance;return k(b)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===J;return b});b.wbg.__wbg_now_e0d8ec93dd25766a=(a=>{const b=c(a).now();return b});b.wbg.__wbg_get_0ee8ea3c7c984c45=((a,b)=>{const d=c(a)[b>>>O];return k(d)});b.wbg.__wbg_length_161c0d89c6535c1d=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_cfecb3965268594c=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_3f093dd26d5569f8=function(){return B(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_self_05040bd9523805b9=function(){return B((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_adc720039f2cb14f=function(){return B((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_622105db80c1457d=function(){return B((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_f56b013ed9bcf359=function(){return B((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbg_resolve_5da6faf2c96fd1d5=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_f9e58f5a50f43eae=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_then_20a5920e447d1cb1=((a,b,d)=>{const e=c(a).then(c(b),c(d));return k(e)});b.wbg.__wbg_buffer_b914fb8b50ebbc3e=(a=>{const b=c(a).buffer;return k(b)});b.wbg.__wbg_newwithbyteoffsetandlength_42904a72cefa1e00=((a,b,d)=>{const e=new Int8Array(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_newwithbyteoffsetandlength_0aafe9b39ed85f71=((a,b,d)=>{const e=new Int16Array(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_newwithbyteoffsetandlength_9ca2c1faeb430732=((a,b,d)=>{const e=new S(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_newwithbyteoffsetandlength_0de9ee56e9f6ee6e=((a,b,d)=>{const e=new P(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_new_b1f2d6842d615181=(a=>{const b=new P(c(a));return k(b)});b.wbg.__wbg_set_7d988c98e6ced92d=((a,b,d)=>{c(a).set(c(b),d>>>O)});b.wbg.__wbg_length_21c4b0ae73cba59d=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newwithbyteoffsetandlength_8c2171d5a9b7f791=((a,b,d)=>{const e=new Uint16Array(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_newwithbyteoffsetandlength_7f2d9491ea8c746e=((a,b,d)=>{const e=new Uint32Array(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbg_newwithbyteoffsetandlength_5fd0a60d38f47fa6=((a,b,d)=>{const e=new Float32Array(c(a),b>>>O,d>>>O);return k(e)});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/Z+ Q]=g;r()[b/Z+ O]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new N(j(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return k(b)});b.wbg.__wbindgen_closure_wrapper664=((a,b,c)=>{const d=w(a,b,_,x);return k(d)});b.wbg.__wbindgen_closure_wrapper666=((a,b,c)=>{const d=w(a,b,_,y);return k(d)});b.wbg.__wbindgen_closure_wrapper668=((a,b,c)=>{const d=w(a,b,_,z);return k(d)});b.wbg.__wbindgen_closure_wrapper739=((a,b,c)=>{const d=w(a,b,212,A);return k(d)});return b});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1ee4a085a06e8c25(b,c,k(d))});var t=(()=>{if(s===K||s.byteLength===O){s=new Float64Array(a.memory.buffer)};return s});var y=((b,c)=>{try{const g=a.__wbindgen_add_to_stack_pointer(-Y);a.wasm_bindgen__convert__closures__invoke0_mut__h05929b5d020aa697(g,b,c);var d=r()[g/Z+ O];var e=r()[g/Z+ Q];if(e){throw f(d)}}finally{a.__wbindgen_add_to_stack_pointer(Y)}});var e=(a=>{if(a<132)return;b[a]=d;d=a});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:Q,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=O;try{return e(c,f.b,...b)}finally{if(--f.cnt===O){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var o=((a,b,c)=>{if(c===J){const c=m.encode(a);const d=b(c.length,Q)>>>O;i().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,Q)>>>O;const f=i();let g=O;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==O){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,Q)>>>O;const b=i().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written;e=c(e,d,g,Q)>>>O};l=g;return e});var i=(()=>{if(h===K||h.byteLength===O){h=new P(a.memory.buffer)};return h});var j=((a,b)=>{a=a>>>O;return g.decode(i().subarray(a,a+ b))});var F=((b,c)=>{a=b.exports;H.__wbindgen_wasm_module=c;s=K;q=K;h=K;a.__wbindgen_start();return a});let a;const b=new I(128).fill(J);b.push(J,K,!0,!1);let d=b.length;const g=typeof TextDecoder!==L?new TextDecoder(M,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw N(`TextDecoder not available`)}};if(typeof TextDecoder!==L){g.decode()};let h=K;let l=O;const m=typeof TextEncoder!==L?new TextEncoder(M):{encode:()=>{throw N(`TextEncoder not available`)}};const n=typeof m.encodeInto===R?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=K;let s=K;const v=typeof X===L?{register:()=>{},unregister:()=>{}}:new X(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});export default H;export{G as initSync}