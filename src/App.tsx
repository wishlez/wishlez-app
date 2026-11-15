import {FC} from 'react';
import '@/src/App.css';
import {Route, Routes} from 'react-router';
import {Launcher} from '@/src/Portfolio/Launcher.tsx';
import {Toaster} from '@/components/ui/toast.tsx';

export const App: FC = () => (
    <>
        <Routes>
            <Route path={'/'} element={<Launcher/>}/>
        </Routes>
        <Toaster/>
    </>
);
