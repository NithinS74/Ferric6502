/* @ts-self-types="./nesoxide.d.ts" */

export class CPU {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CPUFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cpu_free(ptr, 0);
    }
    /**
     * @param {number} start
     * @param {number} length
     * @returns {Uint8Array}
     */
    get_memory_slice(start, length) {
        const ret = wasm.cpu_get_memory_slice(this.__wbg_ptr, start, length);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {number}
     */
    get_program_counter() {
        const ret = wasm.cpu_get_program_counter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_register_a() {
        const ret = wasm.cpu_get_register_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_register_x() {
        const ret = wasm.cpu_get_register_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_register_y() {
        const ret = wasm.cpu_get_register_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_stack_pointer() {
        const ret = wasm.cpu_get_stack_pointer(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get_status_register() {
        const ret = wasm.cpu_get_status_register(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {Uint8Array} program
     */
    load_program_from_js(program) {
        const ptr0 = passArray8ToWasm0(program, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.cpu_load_program_from_js(this.__wbg_ptr, ptr0, len0);
    }
    constructor() {
        const ret = wasm.cpu_new();
        this.__wbg_ptr = ret;
        CPUFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    reset() {
        wasm.cpu_reset(this.__wbg_ptr);
    }
    /**
     * @returns {boolean}
     */
    step() {
        const ret = wasm.cpu_step(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get program_counter() {
        const ret = wasm.__wbg_get_cpu_program_counter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get register_a() {
        const ret = wasm.__wbg_get_cpu_register_a(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get register_x() {
        const ret = wasm.__wbg_get_cpu_register_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get register_y() {
        const ret = wasm.__wbg_get_cpu_register_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get stack_pointer() {
        const ret = wasm.__wbg_get_cpu_stack_pointer(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get status_register() {
        const ret = wasm.__wbg_get_cpu_status_register(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set program_counter(arg0) {
        wasm.__wbg_set_cpu_program_counter(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set register_a(arg0) {
        wasm.__wbg_set_cpu_register_a(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set register_x(arg0) {
        wasm.__wbg_set_cpu_register_x(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set register_y(arg0) {
        wasm.__wbg_set_cpu_register_y(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set stack_pointer(arg0) {
        wasm.__wbg_set_cpu_stack_pointer(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set status_register(arg0) {
        wasm.__wbg_set_cpu_status_register(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) CPU.prototype[Symbol.dispose] = CPU.prototype.free;
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_9c75d47bf9e7731e: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./nesoxide_bg.js": import0,
    };
}

const CPUFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cpu_free(ptr, 1));

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('nesoxide_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
