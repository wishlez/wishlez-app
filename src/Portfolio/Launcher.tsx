import {FC} from 'react';
import {Button} from '@/components/ui/button.tsx';
import {IconFolderOpen, IconPlus} from '@tabler/icons-react';
import {useTranslation} from 'react-i18next';
import {useWfolioHandlers} from '@/components/useWfolioHandlers.ts';

export const Launcher: FC = () => {
    const {t} = useTranslation();
    const {handleCreate, handleOpen} = useWfolioHandlers();

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
