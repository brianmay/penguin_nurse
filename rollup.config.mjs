import copy from 'rollup-plugin-copy'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import replace from '@rollup/plugin-replace'
import terser from '@rollup/plugin-terser'

export default [
    {
        input: 'barcode.js',
        output: {
            file: './assets/bundle.js',
            format: 'esm',
            generatedCode: 'es2015',
            sourcemap: false,
        },
        plugins: [
            // Replace Node.js detection with `false` so the `if(f){await import("module")...}`
            // dead-code block is tree-shaken out, avoiding esbuild errors in dx serve.
            replace({
                preventAssignment: true,
                values: {
                    '"object"==typeof process&&"object"==typeof process.versions&&"string"==typeof process.versions.node': 'false',
                },
            }),
            nodeResolve(),
            copy({
                targets: [
                    {
                        src: 'node_modules/@undecaf/zbar-wasm/dist/zbar.wasm',
                        dest: './assets/'
                    }
                ],
            }),
            terser(),
        ],
    },
]
