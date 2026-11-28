import {Toaster} from '@/components/ui/toast.tsx';
import {Launcher} from '@/src/Views/Launcher.tsx';
import '@/i18n/i18n.ts';
import '@/src/app.css';
import React from 'react';
import ReactDOM from 'react-dom/client';

if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (event) => {
        event.preventDefault();
    }, {capture: true});
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <Launcher/>
        <Toaster/>
    </React.StrictMode>
);
