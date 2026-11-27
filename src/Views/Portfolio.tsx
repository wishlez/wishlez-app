import {Layout} from '@/components/Layout.tsx';
import {Toaster} from '@/components/ui/toast.tsx';
import {PortfolioMenuEvent} from '@/components/usePortfolioMenuEvent.ts';
import {usePortfolioNav} from '@/components/usePortfolioNav.ts';
import {Accounts} from '@/src/Accounts/Accounts.tsx';
import {FC} from 'react';
import {Route, Routes} from 'react-router';
import '@/src/app.css';

export const Portfolio: FC = () => {
    usePortfolioNav();

    return (
        <>
            <Routes>
                <Route path={'/'} element={<Layout hideBackButton title="My Portfolio"/>}/>
                <Route path={`/${PortfolioMenuEvent.account_list}`} element={<Accounts/>}/>
            </Routes>
            <Toaster/>
        </>
    );
};
