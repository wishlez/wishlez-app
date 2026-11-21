import {MenuId} from '@/components/types/MenuId.ts';
import {TauriEventHandler, useTauriEvent} from '@/components/useTauriEvent.ts';
import {useCallback} from 'react';

export const MenuEventPrefix = 'menu:';

export const PortfolioMenuEvent: {
    [key in MenuId]: key
} = {
    file_create: 'file_create',
    file_open: 'file_open',
    file_open_launcher: 'file_open_launcher',
    account_list: 'account_list',
    account_create: 'account_create'
};

export const usePortfolioMenuEvent = <T>(id: MenuId, handler: TauriEventHandler<T>): void => {
    const handleMenuEvent: TauriEventHandler<T> = useCallback(async (name, payload) => {
        if (name.startsWith(MenuEventPrefix)) {
            const menuId = name.substring(MenuEventPrefix.length);

            if (menuId === id) {
                await handler(menuId, payload);
            }
        }
    }, [id, handler]);

    return useTauriEvent(`${MenuEventPrefix}${id}`, handleMenuEvent);
};
