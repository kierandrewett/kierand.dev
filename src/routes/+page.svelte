<script lang="ts">
	import Clock from "../components/Clock.svelte";
	import Contact from "../components/Contact.svelte";
	import Interests from "../components/Interests.svelte";
	import LastFm from "../components/LastFM.svelte";
</script>

<h2>
	<div class="icon-container">
		<img class="profile-picture" src="/favicon.png" alt="Kieran Drewett Profile" />
	</div>
	Kieran Drewett
</h2>
<p>
	<span>Software Engineer based in the UK.</span>
</p>

<Clock />
<LastFm />
<Interests />
<Contact />

<style>
	.profile-picture {
		width: 1.75rem;
		height: 1.75rem;
		border-radius: 5rem;
	}
</style>
