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
	Full-stack developer based in the UK currently building Dot Browser at
	<Link href="https://www.dothq.org/">Dot HQ</Link>.
</p>

<Link href={"/api/cv"}>CV / Resume</Link>

<Clock />
<LastFm />
<Interests />
<Contact />

<style>
	.profile-picture {
		width: 3rem;
		height: 3rem;
		border-radius: 3rem;
	}
</style>
