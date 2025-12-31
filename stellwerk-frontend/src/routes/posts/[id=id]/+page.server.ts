import { error } from '@sveltejs/kit';
import { apiClient, type Post } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad<{ post: Post }> = async ({ params, cookies }) => {
	const client = apiClient(cookies.get('token')!);

	const post = (
		await client.GET('/posts/{id}', {
			params: {
				path: {
					id: params.id,
				},
			},
		})
	).data;

	if (!post) error(404);

	return {
		post: post,
	};
};
