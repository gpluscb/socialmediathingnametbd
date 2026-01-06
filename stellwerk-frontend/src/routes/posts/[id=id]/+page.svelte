<script lang="ts">
	import PostById from '$lib/components/post/PostById.svelte';
	import type { PageProps } from './$types';

	const { params }: PageProps = $props();
	const { id: postId } = $derived(params);
</script>

<PostById id={postId} />
