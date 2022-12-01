import * as shell from '../crate/pkg';
import { HistoryStore } from './utils/history_store';
import { LoperServiceClient } from './database';

export const isNode = () => (typeof navigator === 'undefined' ? true : false);
const userAgent = () => (isNode() ? 'node' : navigator.userAgent);
export const isSafari = () => /^((?!chrome|android).)*safari/i.test(userAgent());

const hasWebGL = (): boolean => {
    if (isSafari()) {
        return false;
    }
    const canvas = document.createElement('canvas') as any;
    const supports = 'probablySupportsContext' in canvas ? 'probablySupportsContext' : 'supportsContext';
    if (supports in canvas) {
        return canvas[supports]('webgl2');
    }
    return 'WebGL2RenderingContext' in window;
};

class ShellRuntime {
    history: HistoryStore;
    service: LoperServiceClient | null;
    resizeHandler: (_event: UIEvent) => void;

    constructor(protected container: HTMLDivElement) {
        this.history = new HistoryStore();
        this.service = null;
        this.resizeHandler = (_event: UIEvent) => {
            const rect = container.getBoundingClientRect();
            shell.resize(rect.width, rect.height);
        };
    }

    public async configureClient(this: ShellRuntime, url: string): Promise<LoperServiceClient> {
        if (this.service) {
            if (this.service?.url == url) {
                return this.service;
            }
        }
        this.service = new LoperServiceClient(url);
        return this.service;
    }
    public async readClipboardText(this: ShellRuntime): Promise<string> {
        return await navigator.clipboard.readText();
    }
    public async writeClipboardText(this: ShellRuntime, value: string) {
        return await navigator.clipboard.writeText(value);
    }
    public async pushInputToHistory(this: ShellRuntime, value: string) {
        this.history.push(value);
    }
}

export interface InstantiationProgress {
    startedAt: Date;
    updatedAt: Date;
    bytesTotal: number;
    bytesLoaded: number;
}
export type InstantiationProgressHandler = (p: InstantiationProgress) => void;

interface ShellProps {
    shellModule: RequestInfo | URL | Response | BufferSource | WebAssembly.Module;
    container: HTMLDivElement;
    backgroundColor?: string;
    fontFamily?: string;
}

export async function embed(props: ShellProps) {
    // Initialize the shell
    await shell.default(props.shellModule);

    // Embed into container
    const runtime = new ShellRuntime(props.container);
    shell.embed(props.container!, runtime, {
        ...props,
        fontFamily: props.fontFamily ?? 'monospace',
        backgroundColor: props.backgroundColor ?? '#333',
        withWebGL: hasWebGL(),
    });
    props.container.onresize = runtime.resizeHandler;

    const TERM_BOLD = '\x1b[1m';
    const TERM_NORMAL = '\x1b[m';
    // const TERM_CLEAR = '\x1b[2K\r';

    // Additional steps
    const step = async (label: string, work: () => Promise<void>) => {
        shell.writeln(`${TERM_BOLD}[ RUN ]${TERM_NORMAL} ${label}`);
        await work();
        shell.writeln(`${TERM_BOLD}[ OK  ]${TERM_NORMAL} ${label}`);
    };
    await step('Loading Shell History', async () => {
        await runtime.history.open();
        const [hist, histCursor] = await runtime.history.load();
        shell.loadHistory(hist, histCursor);
    });
    shell.writeln(`${TERM_BOLD}[ RUN ]${TERM_NORMAL} Setup Loper Service Client`);
    await shell.initialSetup();
}
