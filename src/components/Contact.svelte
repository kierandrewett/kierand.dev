<script>
	import { Contact } from "lucide-svelte";
	import Link from "./Link.svelte";
</script>

<section class="contact">
	<h4>
		<div class="icon-container">
			<Contact stroke="currentColor" />
		</div>
		Get in touch
	</h4>

	<ul>
		<li>
			<Link target="_blank" href="https://twitter.com/EnderDev_" title="@EnderDev_"
				>Twitter</Link
			>
		</li>
		<li>
			<Link target="_blank" href="https://github.com/kierandrewett" title="@kierandrewett"
				>GitHub</Link
			>
		</li>

		<li>
			<Link target="_blank" href="https://www.linkedin.com/in/kieran-drewett" title="LinkedIn"
				>LinkedIn</Link
			>
		</li>

		<li>
			<Link target="_blank" href="mailto:kieran@drewett.dev" title="kieran@drewett.dev"
				>Email (Personal)</Link
			>
		</li>

		<li>
			<Link target="_blank" href="mailto:kieran@dothq.org" title="kieran@dothq.org"
				>Email (Dot HQ)</Link
			>
		</li>

		<li>
			<Link
				target="_blank"
				href="https://discord.com/users/217562587938816000"
				title="@217562587938816000">Discord</Link
			>
		</li>

		<li>
			<Link
				target="_blank"
				href="https://mastodon.social/@enderdev@mastodon.social"
				title="@enderdev@mastodon.social">Mastodon</Link
			>
		</li>

		<li>
			<Link target="_blank" href="https://last.fm/user/EnderDev" title="@EnderDev"
				>Last.fm</Link
			>
		</li>
	</ul>
</section>

<style>
	ul {
		list-style-type: none;
	}
</style>
