/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import { getLatestCV } from "$lib/cv";

export async function GET() {
	const url = await getLatestCV();

	return new Response("", {
		status: 301,
		headers: {
			location: url
		}
	});
}
