/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import axios from "axios";

export const getLatestCV = async () => {
	const latestRelease = await axios.get(
		"https://api.github.com/repos/EnderDev/cv/releases/latest"
	);

	return latestRelease.data.assets[0].browser_download_url;
};
