import {Layout} from '@/components/Layout.tsx';
import type {CreateOrUpdateAccountRequest} from '@/components/types/CreateOrUpdateAccountRequest.ts';
import {PortfolioMenuEvent} from '@/components/usePortfolioMenuEvent.ts';
import {AccountForm} from '@/src/Accounts/AccountForm.tsx';
import {invoke} from '@tauri-apps/api/core';
import {FC} from 'react';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router';

export const AccountCreate: FC = () => {
    const {t} = useTranslation();
    const navigate = useNavigate();

    return (
        <Layout parent={PortfolioMenuEvent.account_list} title={t('accounts.create_account')}>
            <AccountForm
                onSubmit={async (values) => {
                    const request: CreateOrUpdateAccountRequest = {id: null, isBuiltIn: false, ...values};
                    await invoke('create_account', {request});
                    navigate(`/${PortfolioMenuEvent.account_list}`);
                }}
                submitButtonText={t('accounts.create_account')}
            />
        </Layout>
    );
};
