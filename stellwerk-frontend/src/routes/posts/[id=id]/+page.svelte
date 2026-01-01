<script lang="ts">
	import Post from '$lib/components/Post.svelte';
	import type { PageProps } from './$types';

	const { data }: PageProps = $props();
	const { post } = $derived(data);
</script>

<Post {post} />
