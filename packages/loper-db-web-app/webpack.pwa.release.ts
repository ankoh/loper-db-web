import { configure } from './webpack.pwa.common';
import path from 'path';

const base = configure({
    buildDir: path.resolve(__dirname, './build/pwa/release'),
    tsLoaderOptions: {
        compilerOptions: {
            configFile: './tsconfig.json',
            sourceMap: false,
            target: "ES5",
            module: "es2020",
        },
    },
    extractCss: true,
    cssIdentifier: '[hash:base64]',
});

export default {
    ...base,
    mode: 'production',
    devtool: false,
};
