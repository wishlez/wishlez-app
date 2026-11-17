import {FC, useEffect, useState} from 'react';
import {invoke} from '@tauri-apps/api/core';
import {showError} from '@/components/showError.ts';
import type {Account} from '@/components/types/Account.ts';

export const Portfolio: FC = () => {
    const [accounts, setAccounts] = useState<Account[]>([]);

    useEffect(() => {
        invoke<Account[]>('read_accounts').then(setAccounts).catch(showError);
    }, []);

    return (
        <main className="min-h-svh p-4">
            <ul className="flex flex-col gap-1">
                {accounts.map(account => (
                    <li key={account.id} className="flex justify-between">
                        <span>{account.name}</span>
                        <span className="text-sm text-muted-foreground">{account.accountType}</span>
                    </li>
                ))}
            </ul>
        </main>
    );
};
