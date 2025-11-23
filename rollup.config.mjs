import copy from 'rollup-plugin-copy'
import { nodeResolve } from '@rollup/plugin-node-resolve'
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
