import copy from 'rollup-plugin-copy'
import replace from '@rollup/plugin-replace'
import { nodeResolve } from '@rollup/plugin-node-resolve'
import terser from '@rollup/plugin-terser'
import { ZBAR_WASM_REPOSITORY } from '@undecaf/barcode-detector-polyfill/zbar-wasm'

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
            nodeResolve(),
            replace({
                values: {
                    // Replaces the repository URL with a local reference
                    [ZBAR_WASM_REPOSITORY]: '@undecaf/zbar-wasm',
                    '/dist/main.js': '',
                    '/dist/index.js': '',
                    'zbar.wasm': 'zbar-6a921bd2bb9606f1.wasm'
                },
                preventAssignment: true,
            }),
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
