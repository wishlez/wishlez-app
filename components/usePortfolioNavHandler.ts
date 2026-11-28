import {MenuId} from '@/components/types/MenuId.ts';
import {useCallback} from 'react';
import {useNavigate} from 'react-router';

export const usePortfolioNavHandler = (id: MenuId) => {
    const navigate = useNavigate();

    return useCallback(() => {
        navigate(`/${id}`);
    }, [navigate, id]);
};
