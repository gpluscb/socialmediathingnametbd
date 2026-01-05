import { error } from '@sveltejs/kit';
import { getPost, type Post } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad<{ post: Post }> = async ({ params }) => {
	const post = await getPost(params.id);

	if (!post) error(404);

	return {
		post: post,
	};
};
