<script lang="ts">
	import type { PageProps } from "./$types";

    const { data }: PageProps = $props();
</script>

<div class="border-4 border-red-400">
    <p>Post by {data.post.author.handle}</p>
    <p>{data.post.content.content}</p>
</div>
