import rust from "@wasm-tool/rollup-plugin-rust";
import { terser } from 'rollup-plugin-terser';

import pkg from './package.json'
const copyright = `// ${pkg.homepage} v${pkg.version} Copyright ${(new Date).getFullYear()} ${pkg.author.name}`;

export default [
    {
        input: 'index.js',
        output: {
            file: "dist/mbd-wasm.js",
            format: "umd",
            sourcemap: true,
            name: "mbd-wasm",
        },
        plugins: [
            rust(),
        ],
    },
    {
        input: 'index.js',
        output: {
            file: "dist/mbd-wasm.cjs.js",
            format: "cjs",
            sourcemap: true,
            name: "mbd-wasm.cjs",
            exports: 'default',
        },
        plugins: [
            rust({
                nodejs: true,
            }),
        ],
    },
    {
        input: 'index.js',
        output: {
            file: "dist/mbd-wasm.min.js",
            format: "umd",
            sourcemap: true,
            name: "mbd-wasm.min",
        },
        plugins: [
            rust({
                inlineWasm: true,
            }),
            terser({output: {preamble: copyright}}),
        ],
    }
];