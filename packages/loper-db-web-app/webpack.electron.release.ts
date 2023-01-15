import { configure } from './webpack.electron.common';
import path from 'path';

const base = configure({
    buildDir: path.resolve(__dirname, './build/electron/release'),
    tsLoaderOptions: {
        compilerOptions: {
            configFile: './tsconfig.json',
            sourceMap: false,
        },
    },
});

export default {
    ...base,
    mode: 'production',
    devtool: false,
};
