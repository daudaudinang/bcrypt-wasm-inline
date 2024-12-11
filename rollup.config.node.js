// rollup.config.node.js
import commonjs from '@rollup/plugin-commonjs';
import path from 'path';
import fs from 'fs';

function inlineWasmPlugin() {
    return {
        name: 'inline',
        transform(code, id) {

            // Chỉ transform file index.js
            if (!id.endsWith('index.js')) return null;

            // Đọc file WASM
            const wasmPath = path.join(path.dirname(id), 'index_bg.wasm');

            if (!fs.existsSync(wasmPath)) {
                console.warn(`WARNING: WASM file not found at ${wasmPath}`);
                return null;
            }

            // Đọc và encode WASM thành base64
            const wasmBuffer = fs.readFileSync(wasmPath);
            const wasmBase64 = wasmBuffer.toString('base64');

            const transformedCode = code.replace(
                /const path = require\('path'\)\.join\(__dirname,\s*['"]index_bg\.wasm['"]\);[\s\S]*?const bytes = require\('fs'\)\.readFileSync\(path\);/,
                `const bytes = Buffer.from("${wasmBase64}", 'base64');`
            );

            return {
                code: transformedCode,
                map: {mappings: ''} // Empty sourcemap
            };
        }
    };
}

export default {
    input: 'dist/raw/index.js',
    output: [
        {
            dir: 'dist',
            format: 'es',
            exports: 'named',
            entryFileNames: 'esm/[name].js',
        },
        {
            dir: 'dist',
            format: 'cjs',
            exports: 'named',
            entryFileNames: 'cjs/[name].js',
        }
    ],
    plugins: [
        inlineWasmPlugin(),
        commonjs(),
    ],
    external: ['path', 'fs', 'util']
};