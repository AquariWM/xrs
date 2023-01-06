// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{
	mask::{ConfigureWindowMask, ModifierMask},
	Atom,
	Button,
	Colormap,
	CurrentableTime,
	Drawable,
	GrabMode,
	Keycode,
	Point,
	StackMode,
	Timestamp,
	Window,
};
use bitflags::bitflags;

use xrbk_macro::{derive_xrb, DataSize, Readable, StaticDataSize, Writable};
extern crate self as xrb;

derive_xrb! {
	/// An event generated when a key is pressed.
	///
	/// This event is generated for all keys: that includes modifier keys.
	pub struct KeyPress: Event(2) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key which was pressed.
		pub keycode: Keycode,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a key is released.
	///
	/// This event is generated for all keys: that includes modifier keys.
	pub struct KeyRelease: Event(3) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The keycode of the key which was released.
		pub keycode: Keycode,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was located
		/// within when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs or the currently focused window may modify how the
		/// `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was
		/// generated, relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a mouse button is pressed.
	pub struct ButtonPress: Event(4) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was pressed.
		pub button: Button,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}

	/// An event generated when a mouse button is released.
	pub struct ButtonRelease: Event(5) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The mouse button which was released.
		pub button: Button,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}
}

/// The type of [`Motion`] event sent.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum MotionNotificationType {
	/// The [`Motion`] event was not one generated for a client selecting
	/// `MOTION_HINT`.
	Normal,

	/// The [`Motion`] event was generated for a client selecting `MOTION_HINT`.
	///
	/// The X server is free to send only one [`Motion`] event to the client
	/// until:
	/// - a mouse button or key is pressed or released; or
	/// - the pointer leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`] request.
	///
	/// [`QueryCursor`]: super::request::QueryCursor
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	Hint,
}

derive_xrb! {
	/// An event generated when the cursor moves within a [`Window`].
	///
	/// Motion events are only generated when the cursor motion begins and ends
	/// in the same window. If the cursor leaves the window, a [`LeaveWindow`] event
	/// will be generated instead, accompanied by an [`EnterWindow`] event for the
	/// window which it moves into.
	///
	/// Selecting for `ANY_MOTION` events means `Motion` events will be received
	/// independently of the currently pressed mouse buttons. Selecting for
	/// button motion events (`BUTTON_1_MOTION`..`BUTTON_5_MOTION` and
	/// `ANY_BUTTON_MOTION`), however, means `Motion` events will only be
	/// received while at least one of the selected mouse buttons is pressed.
	///
	/// If `MOTION_HINT` is selected, the server is free to send only one
	/// `Motion` event with a [`MotionNotificationType`] of [`Hint`] until:
	/// - a mouse button or key is pressed or released; or
	/// - the pointer leaves the `event_window`; or
	/// - the client sends a [`QueryCursor`] or [`GetMotionEvents`] request.
	///
	/// [`Hint`]: MotionNotificationType::Hint
	/// [`QueryCursor`]: super::request::QueryCursor
	/// [`GetMotionEvents`]: super::request::GetMotionEvents
	pub struct Motion: Event(6) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// The type of `Motion` event sent.
		pub notification_type: MotionNotificationType,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// The direct child of the `event_window` which is an ancestor of the
		/// window in which the cursor was located when this event was
		/// generated, if one exists.
		///
		/// If the window in which the cursor was located within (the source
		/// window) when this event was generated is a descendant of the
		/// `event_window` (that is, it was a child of it, or a child of a
		/// child of it, or a child of a child of a child of it, etc.), then
		/// this is set to the direct child of the `event_window` which is the
		/// ancestor, or is, the source window. Otherwise, if the source window
		/// was not a descendent of the `event_window`, then this is set to
		/// `None`.
		///
		/// That means if the source window was a child of a child of the
		/// `event_window`, then this would be set to the source window's
		/// parent, as that is an ancestor of the source window and a direct
		/// child of the `event_window`.
		pub child_window: Option<Window>,

		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		pub root_coords: Point,
		/// The coordinates of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin.
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,

		/// Whether the cursor is on the same screen as the `event_window`.
		pub same_screen: bool,
		_,
	}
}

/// Detail that describes how a window receiving a [`LeaveWindow`] or
/// [`EnterWindow`] event relates to the event which took place.
///
/// If the cursor moves from window A to window B and A is a descendent of B:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Ancestor`].
/// - A [`LeaveWindow`] event is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of
///   [`Descendent`].
///
/// If the cursor moves from window A to window B and A is an ancestor of B:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Descendent`].
/// - An [`EnterWindow`] event is generated on each window between A and B
///   exclusive (in that order) with a detail of [`Intermediate`]
/// - An [`EnterWindow`] event is generated on B with a detail of [`Ancestor`].
///
/// If the cursor moves from window A to window B and window C is their least
/// common ancestor:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Nonlinear`].
/// - A [`LeaveWindow`] event is generated on each window between A and C
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on each window between C and B
///   exclusive (in that order) with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of [`Nonlinear`].
///
/// If the cursor moves from window A to window B and A and B are on different
/// screens:
/// - A [`LeaveWindow`] event is generated on A with a detail of [`Nonlinear`].
/// - If A is not a root window, a [`LeaveWindow`] event is generated on each
///   ancestor of A including its root, in order from A's parent to its root,
///   with a detail of [`NonlinearIntermediate`].
/// - If B is not a root window, an [`EnterWindow`] event is generated on each
///   ancestor of B including its root, in order from B's root to B's parent,
///   with a detail of [`NonlinearIntermediate`].
/// - An [`EnterWindow`] event is generated on B with a detail of [`Nonlinear`].
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum EnterLeaveDetail {
	/// Used for [`LeaveWindow`] events when the cursor leaves a window and
	/// enters an ancestor of that window, and for [`EnterWindow`] events
	/// when the cursor enters a window and leaves an ancestor of that window.
	Ancestor,
	/// Used in [`LeaveWindow`] and [`EnterWindow`] events for all windows
	/// between the newly entered window and the previous window if one is a
	/// descendent of the other.
	Intermediate,
	/// Used for [`LeaveWindow`] events when the cursor leaves a window and
	/// enters a descendent of that window, and for [`EnterWindow`] events
	/// when the cursor enters a window and leaves a descendent of that window.
	Descendant,

	/// Used for [`LeaveWindow`] and [`EnterWindow`] events for the newly
	/// entered window and the previous window if neither is a descendent of the
	/// other.
	Nonlinear,
	/// Used for [`LeaveWindow`] and [`EnterWindow`] events when neither the
	/// window that was left nor the window that was entered are a descendent of
	/// the other.
	///
	/// This is the detail for the [`LeaveWindow`] events generated for all the
	/// windows between the window that was left and the least common ancestor
	/// of that window and the window that was entered (exclusive).
	///
	/// This is the detail for the [`EnterWindow`] events generated for all the
	/// windows between the window that was entered and the least common
	/// ancestor of that window and the window that was left (exclusive).
	NonlinearIntermediate,
}

bitflags! {
	/// A bitmask used in the [`EnterWindow`] and [`LeaveWindow`] events.
	#[derive(Default, DataSize, Readable, StaticDataSize, Writable)]
	pub struct EnterLeaveMask: u8 {
		/// Whether the `event_window` is the focused window or a descendant
		/// of the focused window.
		const FOCUS = 0x01;
		/// Whether the cursor is on the same screen as the `event_window`.
		const SAME_SCREEN = 0x02;
	}
}

derive_xrb! {
	/// An event generated when the cursor enters a [`Window`].
	///
	/// This event is triggered both when the cursor moves to be in a different
	/// window than it was before, as well as when the window under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`WindowUnmapped`],
	/// [`WindowMapped`], [`WindowConfigured`], [`GravityChanged`],
	/// [`WindowCirculated`]).
	///
	/// This event is received only be clients selecting `ENTER_WINDOW` on a
	/// window.
	///
	/// `EnterWindow` events caused by a hierarchy change are generated after
	/// that hierarchy change event (see above), but there is no restriction
	/// as to whether `EnterWindow` events should be generated before or
	/// after [`WindowUnfocused`], [`VisibilityChanged`], or [`Expose`] events.
	pub struct EnterWindow: Event(7) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		pub detail: EnterLeaveDetail,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the final cursor position
		/// (`event_coords`), this is that child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The position of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		pub root_coords: Point,
		/// The position of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin, if the `event_window` is on
		/// the [`SAME_SCREEN`].
		///
		/// If the `event_window` is on a different screen, these coordinates
		/// are zero.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		///
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
		/// [`Normal`] for normal `EnterWindow` events, [`Grab`] and
		/// [`Ungrab`] for events generated by grabs and ungrabs.
		///
		/// [`Normal`]: GrabMode::Normal
		/// [`Grab`]: GrabMode::Grab
		/// [`Ungrab`]: GrabMode::Ungrab
		pub grab_mode: GrabMode,

		/// A bitmask containing two boolean fields, [`FOCUS`] and [`SAME_SCREEN`].
		///
		/// [`FOCUS`]: EnterLeaveMask::FOCUS
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub mask: EnterLeaveMask,
	}

	/// An event generated when the cursor leaves a [`Window`].
	///
	/// This event is triggered both when the cursor moves to be in a different
	/// window than it was before, as well as when the window under the cursor
	/// changes due to a change in the window hierarchy (i.e. [`WindowUnmapped`],
	/// [`WindowMapped`], [`WindowConfigured`], [`GravityChanged`],
	/// [`WindowCirculated`]).
	///
	/// This event is received only be clients selecting `LEAVE_WINDOW` on a
	/// window.
	///
	/// `LeaveWindow` events caused by a hierarchy change are generated after
	/// that hierarchy change event (see above), but there is no restriction
	/// as to whether `LeaveWindow` events should be generated before or
	/// after [`WindowUnfocused`], [`VisibilityChanged`], or [`Expose`] events.
	pub struct LeaveWindow: Event(8) {
		#[sequence]
		/// The sequence number associated with the last [`Request`] related
		/// to this event prior to this event being generated.
		///
		/// [`Request`]: crate::Request
		pub sequence: u16,

		#[metabyte]
		/// Detail about how the event was generated.
		///
		/// See [`EnterLeaveDetail`] for more information.
		pub detail: EnterLeaveDetail,

		/// The time at which this event was generated.
		pub time: Timestamp,

		/// The root window containing the window in which the cursor was
		/// located when this event was generated.
		pub root: Window,
		/// The window which this event was generated in relation to.
		///
		/// This window is found by beginning with the window which the cursor
		/// is located within, then looking up the window hierarchy for the
		/// first window on which any client has selected interest in this
		/// event (provided no window between the two prohibits this event from
		/// generating in its `do_not_propagate_mask`).
		///
		/// Active grabs may modify how the `event_window` is chosen.
		pub event_window: Window,
		/// If a child of the `event_window` contains the initial cursor position
		/// (`event_coords`), this is that child.
		///
		/// Otherwise, this is [`None`].
		pub child_window: Option<Window>,

		/// The position of the cursor at the time this event was generated,
		/// relative to the `root` window's origin.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		pub root_coords: Point,
		/// The position of the cursor at the time this event was generated,
		/// relative to the `event_window`'s origin, if the `event_window` is on
		/// the [`SAME_SCREEN`].
		///
		/// If the `event_window` is on a different screen, these coordinates
		/// are zero.
		///
		/// This is always the final position of the cursor, not its initial
		/// position.
		///
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub event_coords: Point,

		/// The state of mouse buttons and modifier keys immediately
		/// before this event was generated.
		pub modifiers: ModifierMask,
		/// [`Normal`] for normal `LeaveWindow` events, [`Grab`] and
		/// [`Ungrab`] for events generated by grabs and ungrabs.
		///
		/// [`Normal`]: GrabMode::Normal
		/// [`Grab`]: GrabMode::Grab
		/// [`Ungrab`]: GrabMode::Ungrab
		pub grab_mode: GrabMode,

		/// A bitmask containing two boolean fields, [`FOCUS`] and [`SAME_SCREEN`].
		///
		/// [`FOCUS`]: EnterLeaveMask::FOCUS
		/// [`SAME_SCREEN`]: EnterLeaveMask::SAME_SCREEN
		pub mask: EnterLeaveMask,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum FocusDetail {
	Ancestor,
	Virtual,
	Inferior,
	Nonlinear,
	NonlinearVirtual,
	Pointer,
	PointerRoot,
	None,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum FocusGrabMode {
	Normal,
	Grab,
	Ungrab,
	WhileGrabbed,
}

derive_xrb! {
	pub struct WindowFocused: Event(9) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: FocusDetail,

		pub window: Window,
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	pub struct WindowUnfocused: Event(10) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub detail: FocusDetail,

		pub window: Window,
		pub grab_mode: FocusGrabMode,
		[_; ..],
	}

	pub struct Keymap: Event(11) {
		pub keys: [Keycode; 31],
	}

	pub struct Expose: Event(12) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,

		pub x: u16,
		pub y: u16,
		pub width: u16,
		pub height: u16,

		pub count: u16,
		[_; ..],
	}

	pub struct GraphicsExposure: Event(13) {
		#[sequence]
		pub sequence: u16,

		pub drawable: Drawable,

		pub x: u16,
		pub y: u16,
		pub width: u16,
		pub height: u16,

		pub minor_opcode: u16,
		pub count: u16,
		pub major_opcode: u8,
		[_; ..],
	}

	pub struct NoExposure: Event(14) {
		#[sequence]
		pub sequence: u16,

		pub drawable: Drawable,
		pub minor_opcode: u16,
		pub major_opcode: u8,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum Visibility {
	Unobscured,
	PartiallyObscured,
	FullyObscured,
}

derive_xrb! {
	pub struct VisibilityChanged: Event(15) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub visibility: Visibility,
		[_; ..],
	}

	pub struct WindowCreated: Event(16) {
		#[sequence]
		pub sequence: u16,

		pub parent: Window,
		pub window: Window,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub border_width: u16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct WindowDestroyed: Event(17) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub destroyed_window: Window,
		[_; ..],
	}

	pub struct WindowUnmapped: Event(18) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub unmapped_window: Window,

		/// Whether the window was unmapped with a [`ConfigureWindow`] request.
		///
		/// [`ConfigureWindow`]: crate::x11::request::ConfigureWindow
		pub from_configure: bool,
		[_; ..],
	}

	pub struct WindowMapped: Event(19) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub mapped_window: Window,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct MapRequest: Event(20) {
		#[sequence]
		pub sequence: u16,

		pub parent: Window,
		pub window: Window,
		[_; ..],
	}

	pub struct WindowReparented: Event(21) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these fields better; work out what they mean
		pub window: Window,
		pub reparented_window: Window,
		pub parent: Window,

		pub x: i16,
		pub y: i16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct WindowConfigured: Event(22) {
		#[sequence]
		pub sequence: u16,

		pub event: Window,
		pub window: Window,
		pub above_sibling: Option<Window>,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub border_width: u16,

		pub override_redirect: bool,
		[_; ..],
	}

	pub struct ConfigureRequest: Event(23) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub stack_mode: StackMode,

		pub parent: Window,
		pub window: Window,
		pub sibling: Option<Window>,

		pub x: i16,
		pub y: i16,
		pub width: u16,
		pub height: u16,

		pub mask: ConfigureWindowMask,
		[_; ..],
	}

	pub struct GravityChanged: Event(24) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these fields better
		pub event: Window,
		pub window: Window,

		pub x: i16,
		pub y: i16,
		[_; ..],
	}

	pub struct WindowResized: Event(25) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,

		pub width: u16,
		pub height: u16,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum Placement {
	Top,
	Bottom,
}

derive_xrb! {
	pub struct WindowCirculated: Event(26) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these better
		pub event: Window,
		pub window: Window,
		// FIXME: in the protocol it says this is a window with the name
		//        `unused`... I think that is a mistake, especially given the
		//        next event not having such a field, but we should make sure.
		[_; 4],

		pub placement: Placement,
		[_; ..],
	}

	pub struct CirculateRequest: Event(27) {
		#[sequence]
		pub sequence: u16,

		// TODO: name these better
		pub event: Window,
		pub window: Window,
		[_; 4],

		pub placement: Placement,
		[_; ..],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum PropertyChange {
	// This might be for new properties added too, if so mention that in the docs when written.
	Modified,
	Deleted,
}

derive_xrb! {
	pub struct PropertyChanged: Event(28) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub property: Atom,
		pub time: Timestamp,
		pub change: PropertyChange,
		[_; ..],
	}

	pub struct SelectionClear: Event(29) {
		#[sequence]
		pub sequence: u16,

		pub time: Timestamp,
		pub owner: Window,
		pub selection: Atom,
		[_; ..],
	}

	pub struct SelectionRequest: Event(30) {
		#[sequence]
		pub sequence: u16,

		pub time: CurrentableTime,

		pub owner: Window,
		pub requestor: Window,

		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		[_; ..],
	}

	pub struct SelectionNotify: Event(31) {
		#[sequence]
		pub sequence: u16,

		pub time: CurrentableTime,

		pub requestor: Window,

		pub selection: Atom,
		pub target: Atom,
		pub property: Option<Atom>,
		[_; ..],
	}

	pub struct ColormapNotify: Event(32) {
		#[sequence]
		pub sequence: u16,

		pub window: Window,
		pub colormap: Option<Colormap>,
	}

	pub struct ClientMessage: Event(33) {
		#[sequence]
		pub sequence: u16,
		#[metabyte]
		pub format: u8,

		pub window: Window,
		pub r#type: Atom,

		pub data: [u8; 20],
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, DataSize, Readable, Writable)]
pub enum MappingNotifyRequest {
	Modifier,
	Keyboard,
	Pointer,
}

derive_xrb! {
	pub struct MappingNotify: Event(34) {
		#[sequence]
		pub sequence: u16,

		pub request: MappingNotifyRequest,

		pub first_keycode: Keycode,
		pub count: u8,
		[_; ..],
	}
}
