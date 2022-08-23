/**
 * Basic Wasm bindings for [resvg](https://github.com/RazrFalcon/resvg) for
 * Deno.
 *
 * Currently there are three fonts built into the Wasm library, used for
 * rendering text.
 *
 * - `Bitter` is the default and the _serif_ font.
 * - `Inter` is the _sans-serif_ font.
 * - `JetBrains Mono` is the _monospace_ font.
 *
 * @module
 */

import { instantiate } from "./lib/resvg_wasm.generated.js";

let wasm: Awaited<ReturnType<typeof instantiate>> | undefined;

/** Render an SVG string to a typed array encoded as a PNG. */
export async function render(svg: string): Promise<Uint8Array> {
  wasm = wasm ?? await instantiate();
  return wasm.render(svg);
}
