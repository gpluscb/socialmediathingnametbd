<script lang="ts">
	import type { Post } from '$lib/api/client';

	interface Props {
		post: Post;
	}

	const { post }: Props = $props();
</script>

<div class="border-4 border-red-400">
	<p>Post by {post.author.handle}</p>
	<p>{post.content.content}</p>
</div>
