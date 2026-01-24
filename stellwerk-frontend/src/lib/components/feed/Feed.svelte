<script lang="ts">
	import { getRecentPosts, type PaginationReference } from '$lib/api/client';
	import Post from '../post/Post.svelte';
	import { useSearchParams } from 'runed/kit';
	import z from 'zod';

	const schema = z.object({
		pagination_reference: z.literal('newest').nullable().default(null),
		newer_than: z.string().nullable().default(null),
		older_than: z.string().nullable().default(null),
	});
	const params = useSearchParams(schema);

	function paramsToPaginationReference({
		newer_than,
		older_than,
	}: z.output<typeof schema>): PaginationReference {
		if (newer_than) {
			return { newer_than };
		}
		if (older_than) {
			return { older_than };
		}
		return { pagination_reference: 'newest' };
	}

	const paginationReference = $derived(paramsToPaginationReference(params));
</script>

<div>
	<svelte:boundary>
		{@const posts = await getRecentPosts(10, paginationReference)}

		{#each posts as post}
			<a href="/posts/{post.id}">
				<Post {post} />
			</a>
		{/each}

		<button
			onclick={() =>
				params.update({
					older_than: null,
					newer_than: null,
					pagination_reference: 'newest',
				})}
		>
			Newest
		</button>
		<button
			onclick={() => {
				const first = posts.at(0)?.id;
				params.update({
					older_than: null,
					newer_than: first ?? null,
					pagination_reference: first ? null : 'newest',
				});
			}}
		>
			Newer
		</button>
		<button
			onclick={() => {
				const last = posts.at(-1)?.id;
				params.update({
					older_than: last ?? null,
					newer_than: null,
					pagination_reference: last ? null : 'newest',
				});
			}}
		>
			Older
		</button>

		{#snippet pending()}
			Loading...
		{/snippet}

		<!--TODO: I guess log this error somewhere somehow-->
		{#snippet failed(_error, reset)}
			<p>Failed to load.</p>
			<button onclick={reset}>Retry?</button>
		{/snippet}
	</svelte:boundary>
</div>
