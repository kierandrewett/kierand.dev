/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import { getLatestCV } from "$lib/cv";
import axios from "axios";

export async function GET() {
	const url = await getLatestCV();

	const res = await axios.get(url, { responseType: "arraybuffer" });

	return new Response(res.data, {
		status: 200,
		headers: {
			"content-type": "application/pdf",
			"content-disposition": "inline; filename=cv.pdf"
		}
	});
}
