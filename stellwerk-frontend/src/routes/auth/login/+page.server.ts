import { randomBytes } from 'crypto';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { getOAuth2Url } from '$lib/api/client';

export const load: PageServerLoad = async ({ cookies, url }) => {
	let random_session_id = randomBytes(16).toString('base64');
	let redirect_param = url.searchParams.get('redirect');

	cookies.set('temp_sess_id', random_session_id, {
		path: '/auth',
		maxAge: 30 * 60,
		httpOnly: true,
		sameSite: 'lax',
		// TODO: Make secure once tls is set up
		// secure: true,
	});

	if (redirect_param) {
		cookies.set('redirect', redirect_param, {
			path: '/auth',
			// TODO: Configurable for the server
			maxAge: 30 * 60,
			httpOnly: true,
			sameSite: 'lax',
			// TODO: Make secure once tls is set up
			// secure: true,
		});
	}

	let { url: redirect_url } = await getOAuth2Url(
		'Discord',
		new URL('/auth/redirect', url),
		random_session_id,
	);

	redirect(303, redirect_url);
};
