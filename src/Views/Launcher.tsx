import {Button} from '@/components/ui/button.tsx';
import {usePortfolioFileNav} from '@/components/usePortfolioFileNav.ts';
import {IconFolderOpen, IconPlus} from '@tabler/icons-react';
import {FC} from 'react';
import {useTranslation} from 'react-i18next';

export const Launcher: FC = () => {
    const {t} = useTranslation();
    const {handleCreate, handleOpen} = usePortfolioFileNav();

    return (
        <div className={'grid items-center content-center text-center justify-center min-h-[calc(100vh-2rem)] gap-4'}>
            <div className="flex gap-2 justify-center">
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
