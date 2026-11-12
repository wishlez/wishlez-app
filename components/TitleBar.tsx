import {FC} from 'react';
import {getCurrentWindow} from '@tauri-apps/api/window';
import {TitleBarButton} from '@/components/TitleBarButton.tsx';
import {cn} from 'cn';

const iconSize = 16;

export const TitleBar: FC = () => {
    const currentWindow = getCurrentWindow();

    return (
        <header
            data-tauri-drag-region
            className={cn(
                'flex items-center px-2',
                'select-none'
            )}
        >
            <div
                data-tauri-drag-region
                className={cn(
                    'grow'
                )}
            />
            <div className="title-bar-controls">
                <TitleBarButton aria-label={'Minimize'} onClick={() => currentWindow.minimize()}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={iconSize}
                        height={iconSize}
                        viewBox="0 0 24 24"
                    >
                        <path fill="currentColor" d="M19 13H5v-2h14z"/>
                    </svg>
                </TitleBarButton>
                <TitleBarButton aria-label={'Maximize'} onClick={() => currentWindow.toggleMaximize()}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={iconSize}
                        height={iconSize}
                        viewBox="0 0 24 24"
                    >
                        <path fill="currentColor" d="M4 4h16v16H4zm2 4v10h12V8z"/>
                    </svg>
                </TitleBarButton>
                <TitleBarButton aria-label={'Close'} onClick={() => currentWindow.close()} variant={'danger'}>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={iconSize}
                        height={iconSize}
                        viewBox="0 0 24 24"
                    >
                        <path
                            fill="currentColor"
                            d="M13.46 12L19 17.54V19h-1.46L12 13.46L6.46 19H5v-1.46L10.54 12L5 6.46V5h1.46L12 10.54L17.54 5H19v1.46z"
                        />
                    </svg>
                </TitleBarButton>
            </div>
        </header>
    );
};
