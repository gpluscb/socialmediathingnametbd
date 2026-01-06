<script lang="ts">
	import type { Post } from '$lib/api/client';
	import PostContainer from './PostContainer.svelte';

	interface Props {
		post?: Post;
	}

	const { post }: Props = $props();
</script>

<PostContainer>
	{#if post}
		<p>Post by {post.author.handle}</p>
		<p>{post.content.content}</p>
	{:else}
		<p>Not found.</p>
	{/if}
</PostContainer>
