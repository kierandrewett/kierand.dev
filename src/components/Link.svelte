<script>
	import { ArrowUpRight } from "lucide-svelte";

	export let href = "";
	export let target = "";
	export let title = "";
</script>

<a class="link" {href} {target} {title}>
	<slot />
	{#if href.startsWith("http")}
		<ArrowUpRight />
	{/if}
</a>
