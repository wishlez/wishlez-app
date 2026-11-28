import {MenuId} from '@/components/types/MenuId.ts';
import {Button} from '@/components/ui/button.tsx';
import {IconChevronLeft} from '@tabler/icons-react';
import {cn} from 'cn';
import {FC, PropsWithChildren, ReactNode} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router';

type LayoutProps = PropsWithChildren<{
    title: string;
    hideBackButton?: boolean;
    views?: ReactNode;
    actions?: ReactNode;
    parent?: MenuId;
}>;

export const Layout: FC<LayoutProps> = (props) => {
    const navigate = useNavigate();
    const {t} = useTranslation();
    const backPath = `/${props.parent ?? ''}`;

    return (
        <>
            <header
                className={cn(
                    'grid grid-cols-3 items-center',
                    'mbe-4'
                )}
            >
                <div className={'flex justify-self-start'}>
                    <Button
                        aria-label={t('portfolio.go_to_dashboard')}
                        onClick={() => navigate(backPath)}
                        size={'icon-lg'}
                        className={cn(
                            'me-2',
                            {'hidden': props.hideBackButton}
                        )}
                        variant={'outline'}
                    >
                        <IconChevronLeft/>
                    </Button>
                    <h1>{props.title}</h1>
                </div>
                <div className={'justify-self-center-safe'}>{props.views}</div>
                <div className={'justify-self-end-safe'}>{props.actions}</div>
            </header>
            <article>
                {props.children}
            </article>
        </>
    );
};
