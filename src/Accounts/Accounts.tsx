import {Layout} from '@/components/Layout.tsx';
import {showError} from '@/components/showError.ts';
import type {Account} from '@/components/types/Account.ts';
import {Button} from '@/components/ui/button.tsx';
import {Tabs, TabsList, TabsTrigger} from '@/components/ui/tabs.tsx';
import {PortfolioMenuEvent} from '@/components/usePortfolioMenuEvent.ts';
import {usePortfolioNavHandler} from '@/components/usePortfolioNavHandler.ts';
import {IconPlus} from '@tabler/icons-react';
import {invoke} from '@tauri-apps/api/core';
import {FC, useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';

export const Accounts: FC = () => {
    const {t} = useTranslation();
    const [accounts, setAccounts] = useState<Account[]>([]);
    const [activeTab, setActiveTab] = useState<'user' | 'builtin' | 'all'>('user');

    const createAccount = usePortfolioNavHandler(PortfolioMenuEvent.account_create);

    useEffect(() => {
        invoke<Account[]>('read_accounts').then(setAccounts).catch(showError);
    }, []);

    const showBuiltin = activeTab === 'builtin';
    const filteredAccounts = activeTab === 'all' ?
        accounts :
        accounts.filter(account => account.isBuiltIn === showBuiltin);

    return (
        <Tabs defaultValue={activeTab} onValueChange={(value) => setActiveTab(value)}>
            <Layout
                title={t('accounts.list_title')}
                views={
                    <TabsList>
                        <TabsTrigger value={'user'}>{t('accounts.user_accounts')}</TabsTrigger>
                        <TabsTrigger value={'builtin'}>{t('accounts.builtin_accounts')}</TabsTrigger>
                        <TabsTrigger value={'all'}>{t('accounts.all_accounts')}</TabsTrigger>
                    </TabsList>
                }
                actions={
                    <Button variant={'link'} onClick={createAccount}>
                        <IconPlus stroke={2}/>
                        {t('accounts.create_account')}
                    </Button>
                }
            >
                <ul className="flex flex-col gap-1">
                    {filteredAccounts.map(account => (
                        <li key={account.id} className="flex justify-between">
                            <span>{account.name}</span>
                            <span className="text-sm text-muted-foreground">{account.accountType}</span>
                        </li>
                    ))}
                </ul>
            </Layout>
        </Tabs>
    );
};
