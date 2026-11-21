import {type UnlistenFn} from '@tauri-apps/api/event';
import {getCurrentWindow} from '@tauri-apps/api/window';
import {useEffect} from 'react';

export type TauriEventHandler<T = unknown> = (event: string, payload: T) => unknown | Promise<unknown>;

export const useTauriEvent = <T = unknown>(
    eventName: string,
    handler: TauriEventHandler<T>
) => {
    useEffect(() => {
        let disposed = false;
        let unlisten: UnlistenFn | undefined;

        void getCurrentWindow().listen<T>(eventName, async event => {
            if (disposed) return;

            try {
                await handler(event.event, event.payload);
            } catch (error) {
                console.error(`Failed to handle ${eventName}:`, error);
            }
        }).then(cleanup => {
            if (disposed) {
                cleanup();
            } else {
                unlisten = cleanup;
            }
        }).catch(error => {
            console.error(`Failed to subscribe to ${eventName}:`, error);
        });

        return () => {
            disposed = true;
            unlisten?.();
        };
    }, [eventName, handler]);
};
