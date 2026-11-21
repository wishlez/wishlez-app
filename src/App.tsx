import {Toaster} from '@/components/ui/toast.tsx';
import {PortfolioMenuEvent} from '@/components/usePortfolioMenuEvent.ts';
import {usePortfolioNav} from '@/components/usePortfolioNav.ts';
import {Accounts} from '@/src/Accounts/Accounts.tsx';
import {Portfolio} from '@/src/Portfolio/Portfolio.tsx';
import {FC} from 'react';
import {Route, Routes} from 'react-router';
import '@/src/App.css';

export const App: FC = () => {
    usePortfolioNav();

    return (
        <>
            <Routes>
                <Route path={'/'} element={<Portfolio/>}/>
                <Route path={`/${PortfolioMenuEvent.account_list}`} element={<Accounts/>}/>
            </Routes>
            <Toaster/>
        </>
    );
};
