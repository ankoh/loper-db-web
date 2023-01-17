import HtmlWebpackPlugin from 'html-webpack-plugin';
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import ForkTsCheckerWebpackPlugin from 'fork-ts-checker-webpack-plugin';
import path from 'path';

export default {
    mode: 'production',
    devtool: false,
    output: {
        chunkFilename: 'static/js/[name].[contenthash].js',
        assetModuleFilename: 'static/assets/[name].[contenthash][ext]',
        webassemblyModuleFilename: 'static/wasm/[hash].wasm',
        clean: true,
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js', '.mjs', '.jsx', '.css', '.wasm'],
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
                options: {
                    configFile: 'tsconfig.pwa.json',
                    compilerOptions: {
                        sourceMap: false,
                    }
                }
            },
            {
                test: /\.css$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    {
                        loader: 'css-loader',
                        options: {
                            modules: {
                                mode: 'local',
                                auto: true,
                                exportGlobals: true,
                                localIdentName: '[hash:base64]',
                                localIdentContext: path.resolve(__dirname, 'src'),
                            },
                        },
                    },
                ],
            },
            {
                test: /.*\.wasm$/,
                type: 'asset/resource',
                generator: {
                    filename: 'static/wasm/[name].[contenthash][ext]',
                },
            },
            {
                test: /\.(png|jpe?g|gif|svg|ico)$/i,
                type: 'asset/resource',
                generator: {
                    filename: 'static/img/[name].[contenthash][ext]',
                },
            },
        ],
    },
    performance: {
        assetFilter: (file: string) => {
            return file.endsWith('.js');
        },
        maxEntrypointSize: 1000000,
        maxAssetSize: 1000000,
    },
    optimization: {
        usedExports: 'global',
        chunkIds: 'deterministic',
        moduleIds: 'deterministic',
        splitChunks: {
            chunks: 'all',
            cacheGroups: {
                vendors: {
                    test: /[\\/]node_modules[\\/]/,
                    priority: -10,
                },
                default: {
                    priority: -20,
                    reuseExistingChunk: true,
                },
            },
        },
    },
    plugins: [
        new ForkTsCheckerWebpackPlugin(),
        new HtmlWebpackPlugin({
            template: './static/index.html',
            filename: './index.html',
        }),
        new MiniCssExtractPlugin({
            filename: './static/css/[id].[contenthash].css',
            chunkFilename: './static/css/[id].[contenthash].css',
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
};