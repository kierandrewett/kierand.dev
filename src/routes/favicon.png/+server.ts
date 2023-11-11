/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import axios from "axios";

export async function GET() {
	const favicon = await axios.get("https://github.com/EnderDev.png", {
		responseType: "arraybuffer"
	});

	return new Response(favicon.data, {
		headers: {
			"Content-Type": "image/png",
			"Cache-Control": "max-age=604800"
		}
	});
}
