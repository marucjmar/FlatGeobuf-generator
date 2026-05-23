import * as Comlink from "comlink";
import init, { encode as enc } from "flatgeobuf-wasm";

init().then(() => {
  console.log("WASM initialized");
}).catch((err) => {
  console.error("WASM initialization failed:", err);
});

export class API {
  public encode(bytes: Uint8Array): Uint8Array {
    return enc(bytes);
  }
}

// Expose the API
Comlink.expose(new API());