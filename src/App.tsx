import {FC} from 'react';
import './App.css';
import {TitleBar} from '@/components/TitleBar.tsx';

export const App: FC = () => (
    <>
        <TitleBar/>
        <main className="p-2">
            <h1>Welcome to Tauri + React</h1>
        </main>
    </>
);
