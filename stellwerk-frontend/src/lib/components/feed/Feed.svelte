<script lang="ts">
	import { getRecentPosts, type PaginationReference } from '$lib/api/client';
	import Post from '../post/Post.svelte';

	interface Props {
		pagination: PaginationReference;
	}
	
	let { pagination }: Props = $props();
</script>

<svelte:boundary>
	{@const posts = await getRecentPosts(10, pagination)}
	
	<div>
		{#each posts as post}
			<Post {post} />
		{/each}

		<button onclick={() => (pagination = { pagination_reference: 'latest' })}>Newest</button>
		<button
			onclick={() => {
				const first = posts.at(0)?.id;
				pagination = first
					? { after: first }
					: { pagination_reference: 'latest' };
			}}
		>
			Previous
		</button>
		<button
			onclick={() => {
				const last = posts.at(-1)?.id;
				pagination = last
					? { before: last }
					: { pagination_reference: 'latest' };
			}}
		>
			Next
		</button>
	</div>
</svelte:boundary>
