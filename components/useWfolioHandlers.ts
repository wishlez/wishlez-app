import {open, save} from '@tauri-apps/plugin-dialog';
import {invoke} from '@tauri-apps/api/core';
import {showError} from '@/components/showError.ts';
import {useTauriEvent} from '@/components/useTauriEvent.ts';
import {useCallback} from 'react';
import {useTranslation} from 'react-i18next';

const portfolioFilters = [
    {
        name: 'Wishlez Portfolio',
        extensions: ['wfolio']
    }
];

type UseWfolioHandlers = () => {
    handleCreate: () => Promise<void>;
    handleOpen: () => Promise<void>;
}

export const useWfolioHandlers: UseWfolioHandlers = () => {
    const {t} = useTranslation();

    const handleCreate = useCallback(async () => {
        try {
            const path = await save({
                title: t('launcher.create_portfolio'),
                filters: portfolioFilters,
                defaultPath: 'MyPortfolio.wfolio'
            });
            if (path) {
                await invoke('create_portfolio', {path});
            }
        } catch (error) {
            showError(error, 3);
        }
    }, [t]);
    const handleOpen = useCallback(async () => {
        try {
            const path = await open({
                title: t('launcher.open_portfolio'),
                filters: portfolioFilters
            });
            if (path) {
                await invoke('access_portfolio', {path});
            }
        } catch (error) {
            showError(error, 4);
        }
    }, [t]);

    useTauriEvent('menu:file_create', handleCreate);
    useTauriEvent('menu:file_open', handleOpen);

    return {
        handleCreate,
        handleOpen
    };
};
