import createClient from 'openapi-fetch';
import type { paths, components } from '$lib/generated/openapi-schema';
import { error } from '@sveltejs/kit';

// TODO: Configurable
const BASE_URL = 'http://localhost:8080/';
const CLIENT = createClient<paths>({
	baseUrl: BASE_URL,
	querySerializer: {
		object: {
			style: 'form',
			explode: true,
		},
	},
});

export type Post = components['schemas']['Post'];
export type PaginationReference = components['schemas']['PaginationReference'];

export async function getPost(id: string): Promise<Post | undefined> {
	const response = await CLIENT.GET('/posts/{id}', {
		params: { path: { id } },
	});

	if (!response.data) {
		if (response.response.status !== 404) {
			error(500);
		}
	}

	return response.data;
}

export async function getRecentPosts(
	perPage: number,
	pagination_reference: PaginationReference,
): Promise<Post[]> {
	const response = await CLIENT.GET('/posts/recent', {
		params: {
			query: { per_page: perPage, pagination_reference },
		},
	});

	if (!response.data) {
		error(500);
	}

	return response.data;
}
