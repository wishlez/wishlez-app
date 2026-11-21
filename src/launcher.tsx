import {Toaster} from '@/components/ui/toast.tsx';
import {Launcher} from '@/src/Portfolio/Launcher.tsx';
import '@/i18n/i18n.ts';
import '@/src/App.css';
import React from 'react';
import ReactDOM from 'react-dom/client';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <Launcher/>
        <Toaster/>
    </React.StrictMode>
);
