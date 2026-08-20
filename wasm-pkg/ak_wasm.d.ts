/* tslint:disable */
/* eslint-disable */

export class WasmViewer {
    free(): void;
    [Symbol.dispose](): void;
    append_frame(positions_xyz: Float64Array, numbers: Int32Array, cell_3x3: Float64Array): void;
    load_xyz(xyz: string): void;
    constructor(canvas?: HTMLCanvasElement | null);
    run(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_wasmviewer_free: (a: number, b: number) => void;
    readonly wasmviewer_append_frame: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly wasmviewer_load_xyz: (a: number, b: number, c: number) => [number, number];
    readonly wasmviewer_new: (a: number) => number;
    readonly wasmviewer_run: (a: number) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__hf497823071f04816: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen__convert__closures_____invoke__h8c599f349321c945: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h6c028a06bee46edd: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_5: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_7: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_8: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_9: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h0cb13890a2a1a30e_10: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hdfedbdb4ec1ed43e: (a: number, b: number, c: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hbbd341c82aa7fab0: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hfa58cebcc3d62d00: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
