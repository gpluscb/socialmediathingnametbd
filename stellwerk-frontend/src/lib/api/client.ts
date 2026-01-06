import createClient, { type Client } from 'openapi-fetch';
import type { paths, components } from '$lib/generated/openapi-schema';
import { error } from '@sveltejs/kit';

// TODO: Configurable
const BASE_URL: string = 'http://localhost:8080/';
const CLIENT = createClient<paths>({
	baseUrl: BASE_URL,
});

export type Post = components['schemas']['Post'];

export async function getPost(id: string): Promise<Post | undefined> {
	const response = await CLIENT.GET('/posts/{id}', {
		params: { path: { id } },
	});

	return response.data;
}
