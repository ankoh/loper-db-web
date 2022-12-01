import * as React from 'react';
import { createRoot } from 'react-dom/client';
import { Shell } from './pages/shell';
import { Route, Routes, Navigate, BrowserRouter } from 'react-router-dom';
import { NavBarContainer } from './components/navbar';

import '../static/fonts/fonts.module.css';
import './globals.css';
import 'bootstrap/dist/css/bootstrap.min.css';
import 'xterm/css/xterm.css';
import 'react-popper-tooltip/dist/styles.css';

const element = document.getElementById('root');
const root = createRoot(element!);
root.render(
    <BrowserRouter>
        <Routes>
            <Route
                index
                element={
                    <NavBarContainer>
                        <Shell padding={[16, 0, 0, 20]} backgroundColor="#f0f0f0" />
                    </NavBarContainer>
                }
            />
            <Route path="*" element={<Navigate to="/" />} />
        </Routes>
    </BrowserRouter>
);
