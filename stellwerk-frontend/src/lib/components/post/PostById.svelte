<script lang="ts">
	import { getPost } from '$lib/api/client';
	import Post from './Post.svelte';
	import PostContainer from './PostContainer.svelte';

	interface Props {
		id: string;
	}

	const { id }: Props = $props();
</script>

{#snippet pending()}
	<PostContainer>Loading...</PostContainer>
{/snippet}

<!--TODO: I guess log this error somewhere somehow-->
{#snippet failed(_error: unknown, reset: () => void)}
	<PostContainer>
		<p>Failed to load.</p>
		<button onclick={reset}>Retry?</button>
	</PostContainer>
{/snippet}

<svelte:boundary {pending} {failed}><Post post={await getPost(id)} /></svelte:boundary>
