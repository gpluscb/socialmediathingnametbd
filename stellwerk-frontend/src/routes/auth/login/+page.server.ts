import { randomBytes } from 'crypto';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { getAuthUrl } from '$lib/api/client';
import { setTempAuthInfo } from '$lib/cookies';

export const load: PageServerLoad = async ({ cookies, url }) => {
	let random_session_id = randomBytes(16).toString('base64');
	let redirect_param = url.searchParams.get('redirect') ?? undefined;

	setTempAuthInfo(cookies, random_session_id, redirect_param);

	let { url: redirect_url } = await getAuthUrl(
		'Discord',
		new URL('/auth/redirect', url),
		random_session_id,
	);

	redirect(303, redirect_url);
};
