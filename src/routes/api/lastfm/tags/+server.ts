/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import { getTopTags } from "$lib/lastfm";

export async function GET() {
	const data = await getTopTags();

	return new Response(JSON.stringify(data), {
		status: data.ok ? 200 : 400,
		headers: {
			"content-type": "application/json"
		}
	});
}
