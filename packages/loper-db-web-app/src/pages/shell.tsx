import React from 'react';
import FontFaceObserver from 'fontfaceobserver';

import * as shell from '@ankoh/loper-db-web-shell/dist/shell';
import shell_wasm from '@ankoh/loper-db-web-shell/dist/shell_bg.wasm';

import styles from './shell.module.css';

const SHELL_FONT_FAMILY = 'Roboto Mono';

interface ShellProps {
    backgroundColor?: string;
    padding?: number[];
    borderRadius?: number[];
}

export const Shell: React.FC<ShellProps> = (props: ShellProps) => {
    const termContainer = React.useRef<HTMLDivElement | null>(null);

    // Embed the shell into the term container
    React.useEffect(() => {
        console.assert(termContainer.current != null);
        (async () => {
            const regular = new FontFaceObserver(SHELL_FONT_FAMILY).load();
            const bold = new FontFaceObserver(SHELL_FONT_FAMILY, { weight: 'bold' }).load();
            await Promise.all([regular, bold]);

            await shell.embed({
                shellModule: shell_wasm,
                container: termContainer.current!,
                fontFamily: SHELL_FONT_FAMILY,
                backgroundColor: props.backgroundColor,
            });
        })();
    }, []);

    const style: React.CSSProperties = {
        padding: props.padding ? `${props.padding.map(p => `${p}px`).join(' ')}` : '0px',
        borderRadius: props.borderRadius ? `${props.borderRadius.map(p => `${p}px`).join(' ')}` : '0px',
        backgroundColor: props.backgroundColor || 'transparent',
        width: '100%',
        height: '100%',
    };
    return (
        <div className={styles.root} style={style}>
            <div ref={termContainer} className={styles.term_container} />
        </div>
    );
};
