import { getToken } from '$lib/api/client';
import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { deleteTempAuthInfo, getTempAuthInfo, setApiToken } from '$lib/cookies';

export const load: PageServerLoad = async ({ cookies, url }) => {
	let { temp_sess_id, redirect: redirect_path } = getTempAuthInfo(cookies) ?? error(500);
	deleteTempAuthInfo(cookies);

	let code = url.searchParams.get('code');
	let csrf_token = url.searchParams.get('state');

	if (!temp_sess_id || !code || !csrf_token) {
		error(500);
	}

	let token_response = await getToken(code, csrf_token, temp_sess_id, true);

	let expires_at = token_response.expires_at ? new Date(token_response.expires_at) : undefined;
	setApiToken(cookies, token_response.token, expires_at);

	let redirect_url = new URL(redirect_path ?? '/', url);
	redirect(303, redirect_url);
};
