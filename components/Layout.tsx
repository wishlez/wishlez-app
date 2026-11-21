import {Button} from '@/components/ui/button.tsx';
import {ButtonProps} from '@base-ui/react/button';
import {IconChevronLeft} from '@tabler/icons-react';
import {cn} from 'cn';
import {FC, PropsWithChildren} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router';

type LayoutProps = PropsWithChildren<{
    title: string;
    hideBackButton?: boolean;
    slotProps?: {
        backButton?: ButtonProps;
    }
}>;

export const Layout: FC<LayoutProps> = (props) => {
    const navigate = useNavigate();
    const {t} = useTranslation();

    return (
        <main className="min-h-svh p-4">
            <header className={'flex mbe-4'}>
                <Button
                    aria-label={t('portfolio.go_to_dashboard')}
                    onClick={() => navigate('/')}
                    {...props.slotProps?.backButton}
                    size={'icon-lg'}
                    className={cn(
                        'me-2',
                        {'hidden': props.hideBackButton},
                        props.slotProps?.backButton?.className
                    )}
                >
                    <IconChevronLeft/>
                </Button>
                <h1>{props.title}</h1>
            </header>
            <article>
                {props.children}
            </article>
        </main>
    );
};
