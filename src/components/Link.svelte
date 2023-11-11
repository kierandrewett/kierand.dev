<script>
	export let href = "";
	export let target = "";
	export let title = "";
</script>

<a class="link" {href} {target} {title}>
	<slot />
	<svg width="20" height="20" viewBox="0 0 24 24" fill="none"
		><path
			d="M18.3787 3.5H2.00001V0.5H23.5V22H20.5V5.62132L2.06067 24.0607L-0.0606537 21.9393L18.3787 3.5Z"
			fill="currentColor"
		/></svg
	>
</a>

<style>
	.link {
		font-weight: 600;
		color: var(--color-lighter);
		max-width: fit-content;
		padding: 0 1px;
		transition: 0.2s all;
	}

	.link:hover {
		background-color: var(--color);
		color: var(--background);
	}

	.link:not([href^="http"]) svg {
		display: none;
	}
</style>
