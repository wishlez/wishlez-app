import {Layout} from '@/components/Layout.tsx';
import {showError} from '@/components/showError.ts';
import type {Account} from '@/components/types/Account.ts';
import {invoke} from '@tauri-apps/api/core';
import {FC, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';

export const Accounts: FC = () => {
    const {t} = useTranslation();
    const [accounts, setAccounts] = useState<Account[]>([]);

    useEffect(() => {
        invoke<Account[]>('read_accounts').then(setAccounts).catch(showError);
    }, []);

    return (
        <Layout title={t('accounts.list_title')}>
            <ul className="flex flex-col gap-1">
                {accounts.map(account => (
                    <li key={account.id} className="flex justify-between">
                        <span>{account.name}</span>
                        <span className="text-sm text-muted-foreground">{account.accountType}</span>
                    </li>
                ))}
            </ul>
        </Layout>
    );
};
