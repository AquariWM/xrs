// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate self as xrb;

use crate::{big_requests::reply, message::Request};
use derivative::Derivative;
use xrbk_macro::derive_xrb;

derive_xrb! {
	#[derive(Derivative, Debug, X11Size, Readable, Writable)]
	#[derivative(Hash, PartialEq, Eq)]
	pub struct EnableBigRequests: Request(0 /* TODO: extensions use dynamic major opcodes */) -> reply::EnableBigRequests {}
}
