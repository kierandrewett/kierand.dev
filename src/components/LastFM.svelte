<script lang="ts">
	import { browser } from "$app/environment";
	import axios from "axios";
	import title from "title";

	let data: Awaited<ReturnType<typeof import("$lib/lastfm").getCurrentlyScrobbling>>;

	let tags: string[] = [""];

	async function init() {
		if (browser) {
			if (document.visibilityState == "hidden") return;

			const res = await axios.get("/api/lastfm");

			data = res.data;

			if (data && data.ok) {
				if (data.album_art.includes("2a96cbd8b46e442fc41c2b86b821562f")) {
					data.album_art = null;
				}
			}
		}
	}

	init();
	setInterval(() => init(), 30000);

	async function getTopTags() {
		if (browser) {
			let tagsData = [];

			try {
				const res = await axios.get("/api/lastfm/tags");

				tagsData = (Object.keys(res.data.tags || {}) || [])
					.map((t) => title(t))
					.slice(0, 5);

				tags = tagsData;
			} catch (e) {}
		}
	}

	getTopTags();
</script>

<noscript>
	<style>
		.lastfm {
			display: none !important;
		}
	</style>
</noscript>

<section class="lastfm">
	<h4>Currently listening to</h4>

	<a class="lastfm-player" href={"https://last.fm/user/EnderDev"} target="_blank">
		<img
			src={data && data.ok
				? data?.album_art || "/images/album_art.svg"
				: "/images/album_art_loading.svg"}
			alt="Album Art"
		/>
		<div class="info">
			<strong
				>{data && data.ok ? data?.track_name || "Nothing playing" : "Connecting..."}</strong
			>
			<span>{data && data.ok ? data?.artist_name : ""}</span>
		</div>
	</a>

	<h5>My top tags</h5>

	<ul class="lastfm-tags">
		{#each tags as tag}
			<li class="lastfm-tag">{tag}</li>
		{/each}
	</ul>
</section>

<style>
	.lastfm-player {
		display: flex;
		width: 100%;
		max-width: 375px;
		color: var(--pink-lighter);
		background-color: rgba(246, 226, 252, 0.1);
		border: 1px solid rgba(246, 226, 252, 0.075);
		border-radius: 0.5rem;
		padding: 0.5rem;
		gap: 1rem;
		box-shadow: 0 1.6px 3.6px 0 rgba(0, 0, 0, 0.132), 0 0.3px 0.9px 0 rgba(0, 0, 0, 0.108);
		transition: 0.2s all;

		&:hover {
			border-color: rgba(246, 226, 252, 0.2);
		}
	}

	.lastfm-player > img {
		display: flex;
		width: 75px;
		height: 75px;
		max-width: 75px;
		aspect-ratio: 1;
		border-radius: 4px;
		background-color: rgba(255, 255, 255, 0.05);
		background-size: cover;
	}

	.lastfm-player .info {
		display: flex;
		flex-direction: column;
		justify-content: center;
		font-size: 1rem;
		line-height: 1.75rem;
		flex: 1;
	}

	.lastfm-player .info strong {
		line-height: 1.25rem;
	}

	.lastfm-player .info span {
		font-weight: 700;
		color: var(--color-light);
	}

	.lastfm-tags {
		display: flex;
		gap: 0.25rem;
		list-style-type: none;

		& li {
			background-color: rgba(246, 226, 252, 0.1);
			border: 1px solid rgba(246, 226, 252, 0.075);
			box-shadow: 0 1.6px 3.6px 0 rgba(0, 0, 0, 0.132), 0 0.3px 0.9px 0 rgba(0, 0, 0, 0.108);
			padding: 0px 10px;
			font-size: 1rem;
			font-weight: 600;
			border-radius: 4px;
		}
	}
</style>
