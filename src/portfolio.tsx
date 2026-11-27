import {Toaster} from '@/components/ui/toast.tsx';
import {Portfolio} from '@/src/Views/Portfolio.tsx';
import React from 'react';
import ReactDOM from 'react-dom/client';
import {HashRouter} from 'react-router';
import '@/i18n/i18n.ts';
import '@/src/app.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <HashRouter>
            <Portfolio/>
            <Toaster/>
        </HashRouter>
    </React.StrictMode>
);
