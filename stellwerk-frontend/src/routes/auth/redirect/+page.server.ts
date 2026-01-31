import { getOAuth2Authentication } from '$lib/api/client';
import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, url }) => {
    let session_id = cookies.get('temp_sess_id');
    cookies.delete('temp_sess_id', { path: '/auth/redirect' });
    let redirect_path = cookies.get('redirect');
    cookies.delete('redirect', { path: '/auth/redirect' });

    let code = url.searchParams.get('code');
    let csrf_token = url.searchParams.get('state');

    if (!session_id || !code || !csrf_token) {
        console.log(`sessid: ${session_id}`);
        console.log(`code: ${code}`);
        console.log(`state: ${csrf_token}`);
        error(500);
    }

    let { token } = await getOAuth2Authentication(code, csrf_token, session_id, true);

    cookies.set('api_token', token, {
        path: '/',
        maxAge: 60 * 60 * 48,
        httpOnly: true,
        sameSite: true,
        // secure: true,
    });

    let redirect_url = new URL(redirect_path ?? '/', url);
    redirect(303, redirect_url);
};
