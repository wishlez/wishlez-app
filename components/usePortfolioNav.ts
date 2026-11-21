import {usePortfolioFileNav} from '@/components/usePortfolioFileNav.ts';
import {PortfolioMenuEvent, usePortfolioMenuEvent} from '@/components/usePortfolioMenuEvent.ts';
import {TauriEventHandler} from '@/components/useTauriEvent.ts';
import {useCallback} from 'react';
import {useNavigate} from 'react-router';

export const usePortfolioNav = (): void => {
    usePortfolioFileNav();

    const navigate = useNavigate();

    const handleEvent: TauriEventHandler = useCallback((id) => {
        navigate(`/${id}`);
    }, [navigate]);

    usePortfolioMenuEvent(PortfolioMenuEvent.account_create, handleEvent);
    usePortfolioMenuEvent(PortfolioMenuEvent.account_list, handleEvent);
};
