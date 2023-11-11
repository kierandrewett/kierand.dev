<script lang="ts">
	import "../styles/index.scss";
</script>

<main>
	<slot />
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		gap: 2rem;
		max-width: 650px;
		margin: 0 auto;
		padding: 12rem 2rem;
	}
</style>
