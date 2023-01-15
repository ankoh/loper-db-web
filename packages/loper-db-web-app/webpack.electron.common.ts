export function configure(params: any) {
    return {
        target: 'node',
        entry: './src/electron/main.ts',
        resolve: {
            extensions: ['.ts', '.js', '.mjs'],
        },
        module: {
            rules: [
                {
                    test: /\.m?js/,
                    resolve: {
                        fullySpecified: false,
                    },
                },
                {
                    test: /\.tsx?$/,
                    loader: 'ts-loader',
                    exclude: /node_modules/,
                    options: params.tsLoaderOptions,
                },
            ],
        },
    };
}
