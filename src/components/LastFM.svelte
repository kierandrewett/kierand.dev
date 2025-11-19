<script lang="ts">
	import { browser } from "$app/environment";
	import axios from "axios";
	import { LoaderCircle, Music4 } from "lucide-svelte";
	import { onMount } from "svelte";
	import title from "title";

	let data: any = null;
	let tags: string[] = [""];

	let titleEl: HTMLElement | null = null;
	let albumEl: HTMLElement | null = null;

	let overflow = false;
	let raf: number | null = null;

	let isLoadingAlbumArt = true;
	let currentAlbumArtURL: string | null = null;

	let ws: WebSocket | null = null;
	let wsConnected = false;

	const MASK_NONE =
		"linear-gradient(to right, transparent 0%, black 10%, black 90%, transparent 100%)";
	const MASK_RIGHT = "linear-gradient(to right, black 0%, black 90%, transparent 100%)";
	const MASK_LEFT = "linear-gradient(to right, transparent 0%, black 10%, black 100%)";

	function hardReset() {
		if (!titleEl) return;
		titleEl.style.animation = "none";
		titleEl.style.transform = "translateX(0)";
		titleEl.classList.remove("marquee");
		void titleEl.offsetWidth;
		titleEl.style.animation = "";
	}

	function loadAlbumArt(url: string | null) {
		if (!albumEl) return;

		if (url === currentAlbumArtURL && url !== null) return;

		isLoadingAlbumArt = url !== null;

		albumEl.classList.remove("loaded");
		albumEl.style.backgroundImage = "";

		if (!url) return;

		const img = new Image();
		img.src = url;
		img.style.display = "none";

		document.body.appendChild(img);

		img.onload = () => {
			currentAlbumArtURL = url;
			isLoadingAlbumArt = false;

			img.remove();
			if (!albumEl) return;
			albumEl.style.backgroundImage = `url("${url}")`;
			albumEl.classList.add("loaded");
		};

		img.onerror = () => {
			img.remove();
			console.log("Failed to load album art", url);
		};
	}

	function checkOverflow() {
		if (!titleEl) return;

		const parent = titleEl.parentElement!;
		const containerWidth = parent.clientWidth;
		const contentWidth = titleEl.scrollWidth;

		parent.style.setProperty("--visible-width", `${containerWidth}px`);
		const newOverflow = contentWidth > containerWidth + 1;

		if (newOverflow !== overflow) {
			overflow = newOverflow;
			if (!overflow) hardReset();
		}

		if (overflow) updateMarqueeSpeed();
	}

	function updateMarqueeSpeed() {
		if (!titleEl) return;

		const contentWidth = titleEl.scrollWidth;
		const containerWidth = titleEl.parentElement!.clientWidth;
		const distance = contentWidth - containerWidth;

		if (distance <= 0) {
			titleEl.style.setProperty("--marquee-time", "0s");
			return;
		}

		const SPEED = 0.25;
		const duration = distance * SPEED;
		titleEl.style.setProperty("--marquee-time", `${duration}s`);
	}

	function updateMask() {
		if (!titleEl) return;

		const parent = titleEl.parentElement!;
		const containerWidth = parent.clientWidth;
		const contentWidth = titleEl.scrollWidth;

		if (contentWidth <= containerWidth) {
			parent.style.removeProperty("--mask");
			return;
		}

		let x = 0;
		const matrix = getComputedStyle(titleEl).transform;

		if (matrix && matrix !== "none") {
			const match = matrix.match(/matrix.*\((.+)\)/);
			if (match) {
				const parts = match[1].split(",").map((p) => p.trim());
				if (parts.length >= 6) x = parseFloat(parts[4]);
			}
		}

		const end = -(contentWidth - containerWidth);

		if (x >= -1) parent.style.setProperty("--mask", MASK_RIGHT);
		else if (x <= end + 1) parent.style.setProperty("--mask", MASK_LEFT);
		else parent.style.setProperty("--mask", MASK_NONE);
	}

	function trackTransform() {
		updateMask();
		raf = requestAnimationFrame(trackTransform);
	}

	/* ---------------------------------------------------
	   🔥 WS HANDLING
	---------------------------------------------------- */
	function connectWS() {
		if (!browser) return;

		try {
			ws = new WebSocket("wss://lastfm.drewett.dev");

			ws.onopen = () => {
				wsConnected = true;
				console.log("WS connected");
			};

			ws.onmessage = (e) => {
				let msg = null;
				try {
					msg = JSON.parse(e.data);
				} catch {
					return;
				}

				if ("playing" in msg) {
					if (!msg.playing) {
						data = { ok: true };
						loadAlbumArt(null);
						checkOverflow();
					}

					return;
				}

				if ("track" in msg) {
					data = {
						ok: true,
						track_name: msg.track,
						artist_name: msg.artist,
						album: msg.album,
						album_art: msg.album_art
					};
					loadAlbumArt(msg.album_art);
					checkOverflow();
				}
			};

			ws.onerror = () => {
				console.log("WS error");
			};

			ws.onclose = () => {
				wsConnected = false;
				console.log("WS closed, retrying in 5s...");
				setTimeout(connectWS, 5000);
			};
		} catch {
			console.log("WS connection failed, fallbacking");
		}
	}

	async function fallbackAPI() {
		if (wsConnected) return;

		try {
			const res = await axios.get("/api/lastfm");
			data = res.data;

			if (data?.ok) {
				let art =
					data.album_art && !data.album_art.includes("2a96cbd8b46e442fc41c2b86b821562f")
						? data.album_art
						: null;

				loadAlbumArt(art);
				checkOverflow();
				updateMask();
			}
		} catch {}
	}

	async function getTopTags() {
		if (!browser) return;

		try {
			const res = await axios.get("/api/lastfm/tags");
			tags = ((res.data.tags || []) as string[]).map((t) => title(t)).slice(0, 7);
		} catch {}
	}

	$: if (titleEl) {
		if (raf) cancelAnimationFrame(raf);
		checkOverflow();
		updateMask();

		requestAnimationFrame(() => {
			raf = requestAnimationFrame(trackTransform);
		});
	}

	onMount(() => {
		connectWS();
		getTopTags();

		// fallback every 5s
		setInterval(fallbackAPI, 5000);
	});
</script>

<noscript>
	<style>
		.lastfm {
			display: none !important;
		}
	</style>
</noscript>

<section class="lastfm">
	<h4>
		<div class="icon-container">
			<Music4 stroke="currentColor" />
		</div>
		Currently listening to
	</h4>

	<a class="lastfm-player" href={"https://last.fm/user/EnderDev"} target="_blank">
		<div class="album-art" bind:this={albumEl}>
			<div class="art-icon" class:loading={isLoadingAlbumArt}>
				{#if isLoadingAlbumArt}
					<LoaderCircle stroke="currentColor" />
				{:else if data?.ok && data.album_art == null}
					<Music4 stroke="currentColor" />
				{/if}
			</div>
		</div>

		<div class="info">
			<strong>
				{#if data?.ok}
					{#if data.track_name}
						<div bind:this={titleEl} class:marquee={overflow}>
							{data.track_name}
						</div>
					{:else}
						Nothing playing
					{/if}
				{:else}
					Connecting...
				{/if}
			</strong>
			<span>{data?.ok && data.artist_name ? data.artist_name : ""}</span>
		</div>
	</a>

	<h5>
		<div class="icon-container" />
		My top tags
	</h5>

	<ul class="lastfm-tags">
		{#each tags as tag}
			<a href={"https://www.last.fm/tag/" + encodeURI(tag.toLowerCase())} target="_blank">
				<li class="lastfm-tag">{tag}</li>
			</a>
		{/each}
	</ul>
</section>

<style>
	.lastfm-player {
		display: flex;
		width: 100%;
		max-width: 375px;
		color: var(--pink-lighter);
		background-color: var(--card);
		border: 1px solid var(--border);
		border-radius: 0.25rem;
		padding: 0.5rem;
		gap: 1rem;
		box-shadow: 0 1.6px 3.6px 0 rgba(0, 0, 0, 0.132), 0 0.3px 0.9px 0 rgba(0, 0, 0, 0.108);
		transition: 0.2s all;
	}

	.lastfm-player:hover {
		border-color: var(--border-strong);
	}

	.lastfm-player > img {
		display: flex;
		width: 75px;
		height: 75px;
		max-width: 75px;
		aspect-ratio: 1;
		border-radius: 4px;
		background-color: var(--card);
		border: 1px solid var(--border);
		background-size: cover;
		font-size: 1rem;
		text-align: center;
		align-items: center;
		justify-content: center;
	}

	.lastfm-player .info {
		display: flex;
		flex-direction: column;
		justify-content: center;
		font-size: 1rem;
		line-height: 1.75rem;
		flex: 1;
		padding-inline-end: 0.5rem;
		overflow: hidden;
	}

	.lastfm-player .info strong {
		line-height: 1.25rem;
		max-height: 1lh;
		overflow: hidden;
		mask-image: var(--mask);
		mask-size: 100% 100%;
		mask-repeat: no-repeat;
	}

	.lastfm-player .info span {
		font-weight: 500;
		color: var(--color-light);
	}

	.lastfm-player .info strong div {
		overflow: hidden;
		white-space: nowrap;
		min-width: auto;
		width: fit-content;
	}

	.lastfm-tags {
		display: flex;
		gap: 0.25rem;
		list-style-type: none;
		flex-wrap: wrap;
	}

	.lastfm-tags li {
		background-color: var(--card);
		border: 1px solid var(--border);
		box-shadow: 0 1.6px 3.6px 0 rgba(0, 0, 0, 0.132), 0 0.3px 0.9px 0 rgba(0, 0, 0, 0.108);
		padding: 0.5rem 0.6rem;
		line-height: 1rem;
		font-size: 1rem;
		font-weight: 500;
		border-radius: 4px;
		white-space: nowrap;
	}

	.lastfm-tags li:hover {
		border-color: var(--border-strong);
	}

	@keyframes marquee {
		0% {
			transform: translateX(0);
		}
		25% {
			transform: translateX(0);
		}
		60% {
			transform: translateX(calc(-100% + var(--visible-width)));
		}
		75% {
			transform: translateX(calc(-100% + var(--visible-width)));
		}
		100% {
			transform: translateX(0);
		}
	}

	.lastfm-player .info strong div.marquee {
		animation: marquee var(--marquee-time) linear infinite;
		will-change: transform;
	}

	.lastfm-player .info:hover strong div.marquee {
		animation-play-state: paused;
	}

	.lastfm-player .album-art {
		width: 75px;
		height: 75px;
		border-radius: 4px;
		background-color: var(--border);
		border: 1px solid var(--border);
		background-size: cover;
		background-position: center;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		overflow: hidden;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.lastfm-player .album-art .art-icon {
		width: 1.5rem;
		height: 1.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.lastfm-player .album-art.loaded .art-icon {
		display: none;
	}

	.lastfm-player .album-art .art-icon.loading {
		animation: spin 0.75s linear infinite;
	}
</style>
