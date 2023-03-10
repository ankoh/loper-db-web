import { MakerSquirrel } from "@electron-forge/maker-squirrel";
import { MakerZIP } from "@electron-forge/maker-zip";
import { WebpackPlugin } from "@electron-forge/plugin-webpack";

import mainConfig from "./webpack.electron.main";
import rendererConfig from "./webpack.electron.renderer";

export default {
    packagerConfig: {},
    rebuildConfig: {},
    makers: [
        new MakerSquirrel({}),
        new MakerZIP({}, ["darwin"]),
    ],
    plugins: [
        new WebpackPlugin({
            mainConfig: mainConfig as any,
            renderer: {
                config: rendererConfig as any,
                entryPoints: [{
                    name: "main_window",
                    html: "./static/index.html",
                    js: "./src/app.tsx",
                    preload: {
                        js: "./src/electron/preload.ts",
                    },
                }],
            },
        }),
    ],
};
