import i18n from '@/i18n/i18n.ts';
import {toast} from '@/components/ui/toast.tsx';

type TauriError = {
    code: string;
    message: string;
}

type ShowError = (error: unknown, code?: number) => void;

const isTauriError = (error: unknown): error is TauriError => {
    return typeof error === 'object' && error !== null && 'code' in error && 'message' in error;
};

export const showError: ShowError = (error, code = 0) => {
    if (isTauriError(error)) {
        console.error(`Tauri Error [${error.code}]: ${error.message}`);

        toast.add({
            title: i18n.t('errors.code', {context: error.code}),
            type: 'error'
        });
    } else {
        console.error('An error occurred', error);

        toast.add({
            title: i18n.t('errors.code', {context: code.toString().padStart(4, '0')}),
            type: 'error'
        });
    }
};
