/* tslint:disable */
/* eslint-disable */

export class CPU {
    free(): void;
    [Symbol.dispose](): void;
    get_memory_slice(start: number, length: number): Uint8Array;
    get_program_counter(): number;
    get_register_a(): number;
    get_register_x(): number;
    get_register_y(): number;
    get_stack_pointer(): number;
    get_status_register(): number;
    load_program_from_js(program: Uint8Array): void;
    constructor();
    reset(): void;
    step(): boolean;
    program_counter: number;
    register_a: number;
    register_x: number;
    register_y: number;
    stack_pointer: number;
    status_register: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_cpu_free: (a: number, b: number) => void;
    readonly __wbg_get_cpu_program_counter: (a: number) => number;
    readonly __wbg_get_cpu_register_a: (a: number) => number;
    readonly __wbg_get_cpu_register_x: (a: number) => number;
    readonly __wbg_get_cpu_register_y: (a: number) => number;
    readonly __wbg_get_cpu_stack_pointer: (a: number) => number;
    readonly __wbg_get_cpu_status_register: (a: number) => number;
    readonly __wbg_set_cpu_program_counter: (a: number, b: number) => void;
    readonly __wbg_set_cpu_register_a: (a: number, b: number) => void;
    readonly __wbg_set_cpu_register_x: (a: number, b: number) => void;
    readonly __wbg_set_cpu_register_y: (a: number, b: number) => void;
    readonly __wbg_set_cpu_stack_pointer: (a: number, b: number) => void;
    readonly __wbg_set_cpu_status_register: (a: number, b: number) => void;
    readonly cpu_get_memory_slice: (a: number, b: number, c: number) => [number, number];
    readonly cpu_get_program_counter: (a: number) => number;
    readonly cpu_get_register_a: (a: number) => number;
    readonly cpu_get_register_x: (a: number) => number;
    readonly cpu_get_register_y: (a: number) => number;
    readonly cpu_get_stack_pointer: (a: number) => number;
    readonly cpu_get_status_register: (a: number) => number;
    readonly cpu_load_program_from_js: (a: number, b: number, c: number) => void;
    readonly cpu_new: () => number;
    readonly cpu_reset: (a: number) => void;
    readonly cpu_step: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
