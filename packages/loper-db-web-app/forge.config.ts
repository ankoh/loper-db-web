import { MakerSquirrel } from "@electron-forge/maker-squirrel";
import { MakerZIP } from "@electron-forge/maker-zip";
import { MakerDeb } from "@electron-forge/maker-deb";
import { WebpackPlugin } from "@electron-forge/plugin-webpack";

import mainConfig from "./webpack.electron.release";
import rendererConfig from "./webpack.pwa.release";

export default {
    packagerConfig: {},
    rebuildConfig: {},
    makers: [
        new MakerSquirrel({}),
        new MakerZIP({}, ["darwin"]),
        new MakerDeb({}),
    ],
    plugins: [
        new WebpackPlugin({
            mainConfig: mainConfig as any,
            renderer: {
                config: rendererConfig as any,
                entryPoints: [
                {
                    html: "./static/index.html",
                    js: "./src/app.tsx",
                    name: "main_window",
                    preload: {
                        js: "./src/electron/preload.ts",
                    },
                },
                ],
            },
        }),
    ],
};
