/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A task to perform on the Gecko main thread after restyling.

use gecko_bindings::ptr::GeckoArcStyleImageRequest;
use gecko_bindings::structs::nsStyleImageLayers;
use url::Url;
use values::specified::UrlExtraData;

pub struct SendRawPtr<T>(pub *mut T);
unsafe impl<T> Send for SendRawPtr<T> {}
unsafe impl<T> Sync for SendRawPtr<T> {}

pub enum PostRestyleTask {
    ResolveImage(GeckoArcStyleImageRequest, Url, UrlExtraData),
    TrackImages(SendRawPtr<nsStyleImageLayers>),
}
