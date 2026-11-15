import {useEffect} from 'react';
import {type Event, listen, type UnlistenFn} from '@tauri-apps/api/event';

export function useTauriEvent<T = unknown>(
    eventName: string,
    handler: (event: Event<T>) => unknown | Promise<unknown>
) {
    useEffect(() => {
        let disposed = false;
        let unlisten: UnlistenFn | undefined;

        void listen<T>(eventName, async event => {
            if (disposed) return;

            try {
                await handler(event);
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
}
