import {Toaster} from '@/components/ui/toast.tsx';
import {Portfolio} from '@/src/Views/Portfolio.tsx';
import React from 'react';
import ReactDOM from 'react-dom/client';
import {HashRouter} from 'react-router';
import '@/i18n/i18n.ts';
import '@/src/app.css';

if (import.meta.env.DEV && import.meta.env.VITE_INIT_ROUTE && !window.location.hash) {
    window.location.hash = import.meta.env.VITE_INIT_ROUTE;
}

if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (event) => {
        event.preventDefault();
    }, {capture: true});
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <HashRouter>
            <Portfolio/>
            <Toaster/>
        </HashRouter>
    </React.StrictMode>
);
