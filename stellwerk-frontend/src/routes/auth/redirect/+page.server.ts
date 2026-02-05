import { getToken } from '$lib/api/client';
import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, url }) => {
    let session_id = cookies.get('temp_sess_id');
    cookies.delete('temp_sess_id', { path: '/auth' });
    let redirect_path = cookies.get('redirect');
    cookies.delete('redirect', { path: '/auth' });

    let code = url.searchParams.get('code');
    let csrf_token = url.searchParams.get('state');

    if (!session_id || !code || !csrf_token) {
        error(500);
    }

    let token_response = await getToken(code, csrf_token, session_id, true);

    let expires_at = token_response.expires_at ? new Date(token_response.expires_at) : undefined;
    cookies.set('api_token', token_response.token, {
        path: '/',
        expires: expires_at,
        httpOnly: true,
        sameSite: true,
        // TODO: Make secure once tls is set up
        // secure: true,
    });

    let redirect_url = new URL(redirect_path ?? '/', url);
    redirect(303, redirect_url);
};
