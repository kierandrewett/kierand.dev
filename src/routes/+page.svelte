<script lang="ts">
	import Clock from "../components/Clock.svelte";
	import Contact from "../components/Contact.svelte";
	import Interests from "../components/Interests.svelte";
	import LastFm from "../components/LastFM.svelte";
	import Link from "../components/Link.svelte";
</script>

<svelte:head>
	<title>Kieran Drewett</title>
	<meta name="description" content="Kieran Drewett" />
</svelte:head>

<img class="profile-picture" src="/favicon.png" alt="Kieran Drewett Profile" />
<h2>Kieran Drewett</h2>
<p>
	Software engineer based in the UK currently working on Dot Browser at
	<Link href="https://www.dothq.org/">Dot HQ</Link>.
</p>

<Clock />
<LastFm />
<Interests />
<Contact />

<style>
	.profile-picture {
		width: 5rem;
		height: 5rem;
		border-radius: 5rem;
	}
</style>
