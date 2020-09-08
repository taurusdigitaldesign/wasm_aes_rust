import * as wasm from "./wasm/pkg/wasm_aes";

console.log(wasm.zx_encrypt('eyJhIjoiYiIsImIiOiJjIn0=', 'axb2c3e4f5$6e7%8', 'a1b2c3d4e5f6g7h8'));
// console.log(wasm.zx_decrypt('8r4Glof5YVtt96nKrPSprw==', 'axb2c3e4f5$6e7%8', 'a1b2c3d4e5f6g7h8'));

console.log(wasm.zx_encrypt('eyJwaG9uZSI6IjE4MDEyMzQ2NjY2IiwicGFzc3dvcmQiOiJhYWExMTEifQ==', '95796646b46fca55', '5796646b46fca551'));