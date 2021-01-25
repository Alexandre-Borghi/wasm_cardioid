/* tslint:disable */
/* eslint-disable */
/**
*/
export class AppClient {
  free(): void;
/**
*/
  constructor();
/**
*/
  setup(): void;
/**
* @param {number} dt
* @param {number} canvas_width
* @param {number} canvas_height
*/
  update(dt: number, canvas_width: number, canvas_height: number): void;
/**
*/
  render(): void;
/**
* @param {string} color
*/
  background(color: string): void;
/**
* @param {number} cx
* @param {number} cy
* @param {number} r
*/
  circle_stroke(cx: number, cy: number, r: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_appclient_free: (a: number) => void;
  readonly appclient_new: () => number;
  readonly appclient_setup: (a: number) => void;
  readonly appclient_update: (a: number, b: number, c: number, d: number) => void;
  readonly appclient_render: (a: number) => void;
  readonly appclient_background: (a: number, b: number, c: number) => void;
  readonly appclient_circle_stroke: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        