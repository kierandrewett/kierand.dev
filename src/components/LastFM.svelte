<script lang="ts">
	import { browser } from "$app/environment";
	import axios from "axios";

	let data: Awaited<ReturnType<typeof import("$lib/lastfm").getCurrentlyScrobbling>>;

	async function init() {
		if (browser) {
			if (document.visibilityState == "hidden") return;

			const res = await axios.get("/api/lastfm");

			data = res.data;

			if (data.album_art.includes("2a96cbd8b46e442fc41c2b86b821562f")) {
				data.album_art = null;
			}
		}
	}

	init();
	setInterval(() => init(), 30000);
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

	<div class="lastfm-player">
		<img
			src={data
				? data?.album_art || "/images/album_art.svg"
				: "/images/album_art_loading.svg"}
			alt="Album Art"
		/>
		<div class="info">
			<strong>{data ? data?.track_name || "Nothing playing" : "Connecting..."}</strong>
			<span>{data?.artist_name || ""}</span>
		</div>
	</div>
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
</style>
