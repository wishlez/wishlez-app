import {FC} from 'react';
import {Route, Routes} from 'react-router';
import {Toaster} from '@/components/ui/toast.tsx';
import {Portfolio} from '@/src/Portfolio/Portfolio.tsx';
import '@/src/App.css';

export const App: FC = () => (
    <>
        <Routes>
            <Route path={'/'} element={<Portfolio/>}/>
        </Routes>
        <Toaster/>
    </>
);
