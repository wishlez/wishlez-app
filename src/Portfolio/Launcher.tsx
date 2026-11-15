import {FC, useCallback} from 'react';
import {Button} from '@/components/ui/button.tsx';
import {IconFolderOpen, IconPlus} from '@tabler/icons-react';
import {useTranslation} from 'react-i18next';
import {open, save} from '@tauri-apps/plugin-dialog';
import {useTauriEvent} from '@/components/useTauriEvent.ts';
import {invoke} from '@tauri-apps/api/core';

const portfolioFilters = [
    {
        name: 'Wishlez Portfolio',
        extensions: ['wfolio']
    }
];

export const Launcher: FC = () => {
    const {t} = useTranslation();

    const handleCreate = useCallback(async () => {
        const path = await save({
            title: t('launcher.create_portfolio'),
            filters: portfolioFilters,
            defaultPath: 'MyPortfolio.wfolio'
        });
        if (path) {
            await invoke('create_portfolio', {path});
            console.log('Portfolio created at:', path);
        }
    }, [t]);
    const handleOpen = useCallback(async () => {
        const path = await open({
            title: t('launcher.open_portfolio'),
            filters: portfolioFilters
        });
        if (path) {
            await invoke('access_portfolio', {path});
            console.log('Portfolio opened at:', path);
        }
    }, [t]);

    useTauriEvent('menu:file_create', handleCreate);
    useTauriEvent('menu:file_open', handleOpen);

    return (
        <div className={'grid items-center content-center justify-center min-h-svh gap-4'}>
            <div className="flex gap-4 justify-center">
                <Button size={'lg'} onClick={handleCreate}>
                    <IconPlus stroke={2}/>
                    {t('launcher.create')}
                </Button>
                <Button size={'lg'} onClick={handleOpen}>
                    <IconFolderOpen stroke={2}/>
                    {t('launcher.open')}
                </Button>
            </div>
            {t('launcher.introduction')}
        </div>
    );
};
