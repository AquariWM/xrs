// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Requests] defined in the [core X11 protocol] that relate to an X client or
//! the X server.
//!
//! [Requests] are messages sent from an X client to the X server.
//!
//! [Requests]: Request
//! [core X11 protocol]: crate::x11

extern crate self as xrb;

use std::convert::Infallible;
use xrbk::{
	pad,
	Buf,
	BufMut,
	ConstantX11Size,
	ReadError,
	ReadError::FailedConversion,
	ReadResult,
	Readable,
	Writable,
	WriteResult,
	X11Size,
};
use xrbk_macro::{derive_xrb, Readable, Writable, X11Size};

use crate::{
	message::Request,
	unit::Sec,
	x11::{error, reply},
	KillClientTarget,
	String8,
	Toggle,
	ToggleOrDefault,
	Window,
};

macro_rules! request_error {
	(
		$(#[$meta:meta])*
		$vis:vis enum $Name:ident for $Request:ty {
			$($($Error:ident),+$(,)?)?
		}
	) => {
		#[doc = concat!(
			"An [error](crate::message::Error) generated because of a failed [`",
			stringify!($Request),
			"` request](",
			stringify!($Request),
			")."
		)]
		#[doc = ""]
		$(#[$meta])*
		$vis enum $Name {
			$($(
				#[doc = concat!(
					"A [`",
					stringify!($Error),
					"` error](error::",
					stringify!($Error),
					")."
				)]
				$Error(error::$Error)
			),+)?
		}
	};
}

request_error! {
	pub enum ChangeSavedWindowsError for ChangeSavedWindows {
		Match,
		Value,
		Window,
	}
}

/// Whether something is added or removed.
#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
pub enum AddOrRemove {
	/// The thing is added.
	Add,
	/// The thing is removed.
	Remove,
}

derive_xrb! {
	/// A [request] that [adds] or [removes] the specified [window] from the
	/// set of [windows][window] which you have chosen to save.
	///
	/// When a client's resources are destroyed, each of the client's saved
	/// [windows] which are descendents of [windows] created by the client is
	/// [reparented] to the closest ancestor which is not created by the client.
	///
	/// # Errors
	/// The given `window` must not be a [window] created by you, else a
	/// [`Match` error] is generated.
	///
	/// A [`Window` error] is generated if the `window` does not refer to a
	/// defined [window].
	///
	/// A [`Value` error] is generated if the `change_mode` is encoded
	/// incorrectly. It is a bug in X Rust Bindings if that happens.
	///
	/// [window]: Window
	/// [windows]: Window
	/// [request]: Request
	///
	/// [adds]: AddOrRemove::Add
	/// [removes]: AddOrRemove::Remove
	///
	/// [reparented]: super::ReparentWindow
	#[doc(alias = "ChangeSaveSet")]
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ChangeSavedWindows: Request(6, ChangeSavedWindowsError) {
		#[metabyte]
		/// Whether the `window` is added to or removed from your saved
		/// [windows].
		///
		/// [windows]: Window
		#[doc(alias = "mode")]
		pub change_mode: AddOrRemove,

		/// The [window] which is added to or removed from your saved
		/// [windows][window].
		///
		/// # Errors
		/// A [`Match` error] is generated if you created this [window].
		///
		/// A [`Window` error] is generated if this does not refer to a defined
		/// [window].
		///
		/// [window]: Window
		///
		/// [`Match` error]: error::Match
		/// [`Window` error]: error::Window
		pub window: Window,
	}

	/// A [request] that returns whether the specified extension is present and
	/// the message codes associated with it if it is.
	///
	/// # Replies
	/// This [request] generates a [`QueryExtension` reply].
	///
	/// [request]: Request
	///
	/// [`QueryExtension` reply]: reply::QueryExtension
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable)]
	pub struct QueryExtension: Request(98) -> reply::QueryExtension {
		// Length of `name`.
		#[allow(clippy::cast_possible_truncation)]
		let name_len: u16 = name => name.len() as u16,
		[_; 2],

		/// The name of the extension which is to be queried.
		///
		/// This name should use ISO Latin-1 encoding. Uppercase and lowercase
		/// matter.
		#[context(name_len => usize::from(*name_len))]
		pub name: String8,
		[_; name => pad(name)],
	}

	/// A [request] that returns the names of all extensions supported by the X server.
	///
	/// # Replies
	/// This [request] generates a [`ListExtensions` reply].
	///
	/// [request]: Request
	///
	/// [`ListExtensions` reply]: reply::ListExtensions
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct ListExtensions: Request(99) -> reply::ListExtensions;
}

/// The delay used for `timeout` and `interval` in the
/// [`SetScreenSaver` request].
///
/// [`SetScreenSaver` request]: SetScreenSaver
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Delay {
	/// The default option is used.
	Default,
	/// The option is disabled.
	Disabled,

	/// The option is enabled after the given delay.
	Enabled(Sec<u8>),
}

impl ConstantX11Size for Delay {
	const X11_SIZE: usize = i16::X11_SIZE;
}

impl X11Size for Delay {
	fn x11_size(&self) -> usize {
		Self::X11_SIZE
	}
}

impl Readable for Delay {
	fn read_from(buf: &mut impl Buf) -> ReadResult<Self>
	where
		Self: Sized,
	{
		match buf.get_i16() {
			-1 => Ok(Self::Default),
			0 => Ok(Self::Disabled),

			other => match u8::try_from(other) {
				Ok(sec) => Ok(Self::Enabled(Sec(sec))),
				Err(error) => Err(FailedConversion(Box::new(error))),
			},
		}
	}
}

impl Writable for Delay {
	fn write_to(&self, buf: &mut impl BufMut) -> WriteResult {
		match self {
			Self::Default => buf.put_i16(-1),
			Self::Disabled => buf.put_i16(0),

			Self::Enabled(Sec(sec)) => i16::from(*sec).write_to(buf)?,
		}

		Ok(())
	}
}

derive_xrb! {
	/// A [request] that configures options for the screensaver.
	///
	/// The screensaver is enabled if [`timeout`] is
	/// [`Enabled`](Delay::Enabled). When it is enabled, after [`timeout`]
	/// seconds without any cursor or keyboard input, the screensaver is
	/// activated.
	///
	/// If [`prefer_blanking`] is [`Enabled`], displays that support blanking
	/// will go blank when the screensaver is activated.
	///
	/// Otherwise, if [`prefer_blanking`] is [`Disabled`] or the display does
	/// not support blanking and either [`allow_expose_events`] is [`Enabled`]
	/// or the [screen] can be changed without generating [`Expose` events], the
	/// [screen] is changed with a server-specific screensaver.
	///
	/// Otherwise, if [`prefer_blanking`] is [`Disabled`], the display does
	/// not support blanking, or [`allow_expose_events`] is [`Disabled`] and the
	/// [screen] cannot be changed without generating [`Expose` events], no
	/// screensaver is activated.
	///
	/// [screen]: crate::visual::Screen
	/// [request]: Request
	///
	/// [`Enabled`]: ToggleOrDefault::Enabled
	/// [`Disabled`]: ToggleOrDefault::Disabled
	///
	/// [`timeout`]: SetScreenSaver::timeout
	/// [`prefer_blanking`]: SetScreenSaver::prefer_blanking
	/// [`allow_expose_events`]: SetScreenSaver::allow_expose_events
	///
	/// [`Expose` events]: crate::x11::event::Expose
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct SetScreenSaver: Request(107, error::Value) {
		/// Whether the screensaver is [`Enabled`] and, if so, how long without
		/// input before it is activated.
		///
		/// [`Enabled`]: Delay::Enabled
		pub timeout: Delay,
		/// A hint for screensavers with periodic changes as to the interval
		/// between those changes.
		///
		/// If [`Delay::Disabled`] is specified, this hints that no periodic
		/// change should be made.
		pub interval: Delay,

		/// Whether it is preferred that displays that support blanking go blank
		/// when the screensaver is activated.
		pub prefer_blanking: ToggleOrDefault,
		/// Whether screensavers which generate [`Expose` events] are allowed.
		///
		/// [`Expose` events]: crate::x11::event::Expose
		pub allow_expose_events: ToggleOrDefault,
		[_; 2],
	}

	/// A [request] that returns the current [screensaver options].
	///
	/// See also: [`SetScreenSaver`].
	///
	/// # Replies
	/// This [request] generates a [`GetScreenSaver` reply].
	///
	/// [screensaver options]: SetScreenSaver
	/// [request]: Request
	///
	/// [`GetScreenSaver` reply]: reply::GetScreenSaver
	#[derive(Debug, Hash, PartialEq, Eq, X11Size, Readable, Writable, ConstantX11Size)]
	pub struct GetScreenSaver: Request(108) -> reply::GetScreenSaver;
}
