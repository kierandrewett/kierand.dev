/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import { LASTFM_API_KEY } from "$env/static/private";
import axios from "axios";

export const getCurrentlyScrobbling = async () => {
	if (!LASTFM_API_KEY) {
		console.log("No LASTFM_API_KEY supplied!");

		return Promise.reject({
			ok: false,
			error: new Error("No API key provided.")
		});
	}

	const apiUrl = "https://ws.audioscrobbler.com/2.0";

	const params = new URLSearchParams({
		method: "user.getrecenttracks",
		user: "EnderDev",
		api_key: LASTFM_API_KEY,
		format: "json",
		limit: "1"
	});

	try {
		const response = await axios.get(`${apiUrl}?${params.toString()}`);
		const parsedData = response.data;

		const lastfmTrack =
			parsedData.recenttracks?.track?.[0] ||
			({} as {
				name?: string;
				artist?: { "#text"?: string };
				image?: { "#text"?: string }[];
			});

		const lastfmTrackName = lastfmTrack.name || "Track";
		const lastfmArtistName = lastfmTrack.artist?.["#text"] || "Artist";

		const lastfmLastAlbumArt = lastfmTrack.image?.slice(-1)[0];
		const lastfmAlbumArt = lastfmLastAlbumArt?.["#text"] || "/images/album_art.png";

		const isPlaying = !!(lastfmTrack["@attr"] && lastfmTrack["@attr"].nowplaying == "true");

		const result = {
			ok: true,
			track_name: isPlaying ? lastfmTrackName : null,
			artist_name: isPlaying ? lastfmArtistName : null,
			album_art: isPlaying ? lastfmAlbumArt : null,
			now_playing: isPlaying
		};

		return result;
	} catch (e: any) {
		console.log(e);
		return Promise.resolve({
			ok: false as const,
			error: e
		});
	}
};

export const getTopTags = async () => {
	try {
		const { data } = await axios.get(
			"https://raw.githubusercontent.com/kierandrewett/lastfm-data/main/month_10_tags.json"
		);

		const result = {
			ok: true,
			tags: data
		};

		return result;
	} catch (e: any) {
		console.log(e);
		return Promise.resolve({
			ok: false,
			error: e.toString()
		});
	}
};
