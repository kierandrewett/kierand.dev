/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production
export const prerender = true;
