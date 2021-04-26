import rust from "@wasm-tool/rollup-plugin-rust";
import { terser } from 'rollup-plugin-terser';

export default [
    {
        input: {
            "mbd-wasm": "Cargo.toml",
        },
        output: {
            dir: "dist/",
            format: "umd",
            sourcemap: true,
            name: "mbd-wasm",
        },
        plugins: [
            rust(),
        ],
    },
    {
        input: {
            "mbd-wasm.cjs": "Cargo.toml",
        },
        output: {
            dir: "dist/",
            format: "cjs",
            sourcemap: true,
            name: "mbd-wasm.cjs",
        },
        plugins: [
            rust({
                nodejs: true,
            }),
        ],
    },
    {
        input: {
            "mbd-wasm.min": "Cargo.toml",
        },
        output: {
            dir: "dist/",
            format: "umd",
            sourcemap: true,
            name: "mbd-wasm.min",
        },
        plugins: [
            rust({
                inlineWasm: true,
            }),
            terser(),
        ],
    }
];