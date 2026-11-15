import React from 'react';
import ReactDOM from 'react-dom/client';
import {App} from '@/src/App.tsx';
import {BrowserRouter} from 'react-router';
import '@/i18n/i18n.ts';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter>
            <App/>
        </BrowserRouter>
    </React.StrictMode>
);
