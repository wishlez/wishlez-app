import {ComponentProps, FC} from 'react';
import {cn} from 'cn';

type TitleBarButtonProps = ComponentProps<'button'> & {
    variant?: 'default' | 'danger';
};

export const TitleBarButton: FC<TitleBarButtonProps> = ({variant = 'default', className, type, ...props}) => (
    <button
        {...props}
        tabIndex={-1}
        type={'button'}
        className={cn(
            'inline-flex h-10 w-12 items-center justify-center',
            'focus-visible:outline-none',
            'disabled:pointer-events-none disabled:opacity-50',
            variant === 'danger' ?
                'hover:bg-red-700 hover:text-white active:bg-red-800 active:text-white' :
                'hover:bg-accent hover:text-accent-foreground active:bg-accent/80',
            className
        )}
    />
);
