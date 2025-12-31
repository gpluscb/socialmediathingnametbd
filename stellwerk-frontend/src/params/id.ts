import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param: string): boolean => {
    return /\d{1,20}/.test(param);
}) satisfies ParamMatcher;
