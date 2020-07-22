import * as Comlink from 'comlink';

const wasmImport = import('./crate/pkg');

const obj = {
  async initialize() {
    const wasm = await wasmImport;
    Object.assign(obj, wasm);
  },
};

Comlink.expose(obj, self);
