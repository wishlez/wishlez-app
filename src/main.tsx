import {Toaster} from '@/components/ui/toast.tsx';
import {App} from '@/src/App.tsx';
import React from 'react';
import ReactDOM from 'react-dom/client';
import {BrowserRouter} from 'react-router';
import '@/i18n/i18n.ts';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter>
            <App/>
            <Toaster/>
        </BrowserRouter>
    </React.StrictMode>
);
