import type {AccountType} from '@/components/types/AccountType.ts';
import {z} from 'zod';

const all = {
    Asset: true, Liability: true, Equity: true, Income: true, Expense: true,
} satisfies Record<AccountType, true>;

export const accountTypes = Object.keys(all) as AccountType[];

export const accountTypeSchema = z.enum(accountTypes)
