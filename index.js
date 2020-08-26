import * as wasm from "./wasm/pkg/wasm_aes";

console.log(wasm.zx_encrypt('abc', 'axb2c3e4f5$6e7%8', 'a1b2c3d4e5f6g7h8'));
console.log(wasm.zx_decrypt('bbWrOxhQQsC13jv3OKRF+Q==', 'axb2c3e4f5$6e7%8', 'a1b2c3d4e5f6g7h8'));