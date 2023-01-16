import { configure } from './webpack.pwa.common';
import path from 'path';

const base = configure({
    buildDir: path.resolve(__dirname, './build/pwa/release'),
    tsLoaderOptions: {
        configFile: 'tsconfig.pwa.json',
        compilerOptions: {
            sourceMap: false,
        }
    },
    extractCss: true,
    cssIdentifier: '[hash:base64]',
});

export default {
    ...base,
    mode: 'production',
    devtool: false,
};
