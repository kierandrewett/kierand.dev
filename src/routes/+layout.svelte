<script lang="ts">
	import "../styles/index.scss";
</script>

<svelte:head>
	<title>Kieran Drewett</title>
	<meta name="description" content="Kieran Drewett" />
</svelte:head>

<main>
	<slot />
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		gap: 1.5rem;
		max-width: 650px;
		margin: 0 auto;
		padding-block: 10rem;
		padding-inline-start: calc(var(--test) + 1rem);
		padding-inline-end: 2rem;
	}
</style>
