import * as Comlink from "comlink";
import init, { encode as enc, PartialFgbWriterOptions } from "flatgeobuf-wasm";

init().then(() => {
  console.log("WASM initialized");
}).catch((err) => {
  console.error("WASM initialization failed:", err);
});

export class API {
  public encode(bytes: Uint8Array, opts: PartialFgbWriterOptions): Uint8Array {
    return enc(bytes, opts);
  }
}

// Expose the API
Comlink.expose(new API());