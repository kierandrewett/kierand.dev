(() => {
	const titleWrap = document.getElementById("track-title-wrap");
	const titleEl = document.getElementById("track-title");
	const artistEl = document.getElementById("artist-name");
	const albumEl = document.getElementById("album-art");
	const artIconEl = document.getElementById("art-icon");
	const progressEl = document.getElementById("progress");
	const progressCurrentEl = document.getElementById("progress-current");
	const progressDurationEl = document.getElementById("progress-duration");
	const progressBarEl = document.getElementById("progress-bar-fill");
	if (
		!titleWrap ||
		!titleEl ||
		!artistEl ||
		!albumEl ||
		!artIconEl ||
		!progressEl ||
		!progressCurrentEl ||
		!progressDurationEl ||
		!progressBarEl
	) {
		return;
	}

	let data = null;
	let progress = null;
	let overflow = false;
	let raf = null;
	let currentAlbumArtURL = null;
	let ws = null;
	let wsConnected = false;
	const fallbackAlbumArtURL = "/images/album_art.svg";
	const initialState = {
		ok: titleWrap.dataset.initialOk === "true",
		trackName: titleWrap.dataset.initialTrack || "",
		artistName: titleWrap.dataset.initialArtist || "",
		albumArt: titleWrap.dataset.initialArt || ""
	};

	function hardReset() {
		titleEl.style.animation = "none";
		titleEl.style.transform = "translateX(0)";
		titleEl.classList.remove("marquee");
		void titleEl.offsetWidth;
		titleEl.style.animation = "";
	}

	function setArtIcon(mode, text = "") {
		artIconEl.className = "art-icon";
		artIconEl.textContent = text;

		if (mode) {
			artIconEl.classList.add(mode);
		}
	}

	function showFallbackAlbumArt() {
		currentAlbumArtURL = fallbackAlbumArtURL;
		albumEl.style.backgroundImage = `url("${fallbackAlbumArtURL}")`;
		albumEl.classList.add("loaded");
		setArtIcon(null, "");
	}

	function loadAlbumArt(url) {
		if (url === currentAlbumArtURL && url !== null) return;

		albumEl.classList.remove("loaded");
		albumEl.style.backgroundImage = "";

		if (!url) {
			showFallbackAlbumArt();
			return;
		}

		setArtIcon("loading", "");
		const img = new Image();
		img.src = url;
		img.style.display = "none";
		document.body.appendChild(img);

		img.onload = () => {
			currentAlbumArtURL = url;
			img.remove();
			albumEl.style.backgroundImage = `url("${url}")`;
			albumEl.classList.add("loaded");
		};

		img.onerror = () => {
			img.remove();
			showFallbackAlbumArt();
		};
	}

	function checkOverflow() {
		const containerWidth = titleWrap.clientWidth;
		const contentWidth = titleEl.scrollWidth;

		titleWrap.style.setProperty("--visible-width", `${containerWidth}px`);
		const newOverflow = contentWidth > containerWidth + 1;

		if (newOverflow !== overflow) {
			overflow = newOverflow;
			titleEl.classList.toggle("marquee", overflow);
			if (!overflow) hardReset();
		}

		if (overflow) updateMarqueeSpeed();
	}

	function updateMarqueeSpeed() {
		const contentWidth = titleEl.scrollWidth;
		const containerWidth = titleWrap.clientWidth;
		const distance = contentWidth - containerWidth;

		if (distance <= 0) {
			titleEl.style.setProperty("--marquee-time", "0s");
			return;
		}

		const duration = distance * 0.25;
		titleEl.style.setProperty("--marquee-time", `${duration}s`);
	}

	function updateMask() {
		const containerWidth = titleWrap.clientWidth;
		const contentWidth = titleEl.scrollWidth;
		const maxFade = 16;
		const fadeRamp = 28;

		if (contentWidth <= containerWidth) {
			titleWrap.style.setProperty("--left-fade", "0px");
			titleWrap.style.setProperty("--right-fade", "0px");
			return;
		}

		let x = 0;
		const matrix = getComputedStyle(titleEl).transform;

		if (matrix && matrix !== "none") {
			const match = matrix.match(/matrix.*\((.+)\)/);
			if (match) {
				const parts = match[1].split(",").map((part) => part.trim());
				if (parts.length >= 6) x = parseFloat(parts[4]);
			}
		}

		const end = -(contentWidth - containerWidth);
		const distance = Math.abs(end);
		const moved = Math.max(0, -x);
		const remaining = Math.max(0, distance - moved);
		const leftStrength = Math.min(1, moved / fadeRamp);
		const rightStrength = Math.min(1, remaining / fadeRamp);

		titleWrap.style.setProperty("--left-fade", `${maxFade * leftStrength}px`);
		titleWrap.style.setProperty("--right-fade", `${maxFade * rightStrength}px`);
	}

	function trackTransform() {
		updateMask();
		raf = requestAnimationFrame(trackTransform);
	}

	function formatTime(ms) {
		const totalSeconds = Math.floor(ms / 1000);
		const minutes = Math.floor(totalSeconds / 60);
		const seconds = totalSeconds % 60;
		return `${minutes}:${String(seconds).padStart(2, "0")}`;
	}

	function updateProgress() {
		if (progress && data && data.ok && data.track_name) {
			progressEl.hidden = false;
			progressCurrentEl.textContent = formatTime(progress.position_ms || 0);
			progressDurationEl.textContent = formatTime(progress.duration_ms || 0);
			const width = progress.duration_ms
				? (progress.position_ms / progress.duration_ms) * 100
				: 0;
			progressBarEl.style.width = `${Math.max(0, Math.min(100, width))}%`;
		} else {
			progressEl.hidden = true;
			progressBarEl.style.width = "0%";
		}
	}

	function updateTrackDisplay() {
		if (data && data.ok) {
			if (data.track_name) {
				titleEl.textContent = data.track_name;
			} else {
				titleEl.textContent = "Nothing playing";
			}
			artistEl.textContent = data.artist_name || "";
		} else {
			titleEl.textContent = "Connecting...";
			artistEl.textContent = "";
		}

		checkOverflow();
		updateMask();
		updateProgress();
	}

	async function fallbackAPI() {
		if (wsConnected) return;

		try {
			const response = await fetch("/api/lastfm");
			if (!response.ok) return;
			data = await response.json();

			if (data && data.ok) {
				const art =
					data.album_art && !data.album_art.includes("2a96cbd8b46e442fc41c2b86b821562f")
						? data.album_art
						: null;
				loadAlbumArt(art);
				updateTrackDisplay();
			}
		} catch (_) {
			// ignore
		}
	}

	function connectWS() {
		try {
			ws = new WebSocket("wss://lastfm.drewett.dev");

			ws.onopen = () => {
				wsConnected = true;
			};

			ws.onmessage = (event) => {
				let message = null;
				try {
					message = JSON.parse(event.data);
				} catch (_) {
					return;
				}

				if ("playing" in message) {
					if (!message.playing) {
						data = { ok: true };
						loadAlbumArt(null);
						updateTrackDisplay();
					}

					if (message.position_ms && message.duration_ms) {
						progress = {
							position_ms: message.position_ms,
							duration_ms: message.duration_ms
						};
					} else {
						progress = null;
					}

					updateProgress();
					return;
				}

				if ("track" in message) {
					data = {
						ok: true,
						track_name: message.track,
						artist_name: message.artist,
						album: message.album,
						album_art:
							message.album_art ||
							message.track_info?.album?.image?.at(-1)?.url ||
							null
					};
					loadAlbumArt(data.album_art);
					updateTrackDisplay();
				}
			};

			ws.onclose = () => {
				wsConnected = false;
				setTimeout(connectWS, 5000);
			};

			ws.onerror = () => {
				// ignore
			};
		} catch (_) {
			// ignore
		}
	}

	window.addEventListener("resize", () => {
		checkOverflow();
		updateMask();
	});

	loadAlbumArt(null);
	if (initialState.ok) {
		data = {
			ok: true,
			track_name: initialState.trackName || null,
			artist_name: initialState.artistName || null,
			album_art: initialState.albumArt || null
		};
		loadAlbumArt(initialState.albumArt || null);
	} else if (initialState.albumArt) {
		loadAlbumArt(initialState.albumArt);
	}

	updateTrackDisplay();
	connectWS();
	setInterval(fallbackAPI, 5000);
	requestAnimationFrame(() => {
		raf = requestAnimationFrame(trackTransform);
	});
})();
