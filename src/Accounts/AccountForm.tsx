import {accountTypes, accountTypeSchema} from '@/components/accountTypes.ts';
import {showError} from '@/components/showError.ts';
import type {AccountType} from '@/components/types/AccountType.ts';
import {Button} from '@/components/ui/button.tsx';
import {Field, FieldError, FieldGroup, FieldLabel, FieldLegend, FieldSet} from '@/components/ui/field.tsx';
import {Input} from '@/components/ui/input.tsx';
import {Label} from '@/components/ui/label.tsx';
import {RadioGroup, RadioGroupItem} from '@/components/ui/radio-group.tsx';
import {Separator} from '@/components/ui/separator.tsx';
import {zodResolver} from '@hookform/resolvers/zod';
import {FC} from 'react';
import {Controller, useForm} from 'react-hook-form';
import {useTranslation} from 'react-i18next';
import {z} from 'zod';

const schema = z.object({
    name: z.string().trim().min(1).max(100),
    accountType: accountTypeSchema
});

type FormValues = z.infer<typeof schema>;
type AccountFormProps = {
    onSubmit: (values: FormValues) => Promise<void>;
    defaultValues?: FormValues;
    submitButtonText: string;
};

export const AccountForm: FC<AccountFormProps> = (props) => {
    const {t} = useTranslation();
    const {
        register,
        handleSubmit,
        control,
        reset,
        formState: {errors, isSubmitting}
    } = useForm<FormValues>({
        resolver: zodResolver(schema),
        defaultValues: props.defaultValues ?? {name: '', accountType: 'Asset'}
    });

    const onSubmit = handleSubmit(async (values) => {
        try {
            await props.onSubmit(values);
        } catch (e) {
            showError(e);
        }
    });

    return (
        <form onSubmit={onSubmit}>
            <FieldGroup className={'max-w-md m-auto'}>
                <Field data-invalid={!!errors.name}>
                    <FieldLabel htmlFor="name">{t('accounts.form.name')}</FieldLabel>
                    <Input id="name" autoFocus {...register('name')}/>
                    {errors.name && <FieldError>{t('accounts.form.name_error')}</FieldError>}
                </Field>

                <FieldSet>
                    <FieldLegend variant={'label'}>{t('accounts.form.account_type')}</FieldLegend>
                    <Controller
                        control={control}
                        name={'accountType'}
                        render={({field}) => (
                            <RadioGroup value={field.value} onValueChange={field.onChange}>
                                {accountTypes.map(label => (
                                    <div key={label} className="flex items-center gap-2">
                                        <RadioGroupItem value={label} id={`accountType-${label}`}/>
                                        <Label htmlFor={`accountType-${label}`}>{t('accounts.form.account_type', {context: label.toLowerCase() as Lowercase<AccountType>})}</Label>
                                    </div>
                                ))}
                            </RadioGroup>
                        )}
                    />
                </FieldSet>

                <Separator/>

                <Field orientation={'horizontal'} className={'justify-between'}>
                    <Button type="button" disabled={isSubmitting} variant={'secondary'} onClick={() => reset()}>{t('common.reset')}</Button>
                    <Button type="submit" disabled={isSubmitting}>{props.submitButtonText}</Button>
                </Field>
            </FieldGroup>
        </form>
    );
};
