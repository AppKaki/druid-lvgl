// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Platform abstraction for druid toolkit.
//!
//! `druid-shell` is an abstraction around a given platform UI & application
//! framework. It provides common types, which then defer to a platform-defined
//! implementation.
//!
//! # Env
//!
//! For testing and debugging, `druid-shell` can change its behavior based on environment
//! variables. Here is a list of environment variables that `druid-shell` supports:
//!
//! - `DRUID_SHELL_DISABLE_X11_PRESENT`: if this is set and `druid-shell` is using the `x11`
//! backend, it will avoid using the Present extension.

#![deny(intra_doc_link_resolution_failure)]
#![allow(clippy::new_without_default)]
#![deny(clippy::trivially_copy_pass_by_ref)]

////pub use kurbo;
////pub use piet_common as piet;

////Begin
/// Numeric type for screen coordinates
pub type ScreenCoord = u8;  //  Previously f64

/// A 2D point.
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point { ////
    /// The x coordinate.
    pub x: ScreenCoord,
    /// The y coordinate.
    pub y: ScreenCoord,
}

/// A 2D size.
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size { ////
    /// The width.
    pub width: ScreenCoord,
    /// The height.
    pub height: ScreenCoord,
}

/// A 2D vector.
///
/// This is intended primarily for a vector in the mathematical sense,
/// but it can be interpreted as a translation, and converted to and
/// from a point (vector relative to the origin) and size.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec2 { ////
    /// The x-coordinate.
    pub x: ScreenCoord,
    /// The y-coordinate.
    pub y: ScreenCoord,
}

/// A rectangle.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Rect { ////
    /// The minimum x coordinate (left edge).
    pub x0: ScreenCoord,
    /// The minimum y coordinate (top edge in y-down spaces).
    pub y0: ScreenCoord,
    /// The maximum x coordinate (right edge).
    pub x1: ScreenCoord,
    /// The maximum y coordinate (bottom edge in y-down spaces).
    pub y1: ScreenCoord,
}

/// Insets from the edges of a rectangle.
///
///
/// The inset value for each edge can be thought of as a delta computed from
/// the center of the rect to that edge. For instance, with an inset of `2.0` on
/// the x-axis, a rectange with the origin `(0.0, 0.0)` with that inset added
/// will have the new origin at `(-2.0, 0.0)`.
///
/// Put alternatively, a positive inset represents increased distance from center,
/// and a negative inset represents decreased distance from center.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Insets { ////
    /// The minimum x coordinate (left edge).
    pub x0: ScreenCoord,
    /// The minimum y coordinate (top edge in y-down spaces).
    pub y0: ScreenCoord,
    /// The maximum x coordinate (right edge).
    pub x1: ScreenCoord,
    /// The maximum y coordinate (bottom edge in y-down spaces).
    pub y1: ScreenCoord,
}

/// A 2D affine transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affine([ScreenCoord; 6]); ////
////End

#[macro_use]
mod util;

mod application;
mod clipboard;
mod common_util;
mod dialog;
mod error;
mod hotkey;
mod keyboard;
mod menu;
mod mouse;
mod platform;
mod scale;
mod window;

pub use application::{AppHandler, Application};
pub use clipboard::{Clipboard, ClipboardFormat, FormatId};
pub use common_util::Counter;
pub use dialog::{FileDialogOptions, FileInfo, FileSpec};
pub use error::Error;
pub use hotkey::{HotKey, RawMods, SysMods};
pub use keyboard::{Code, IntoKey, KbKey, KeyEvent, KeyState, Location, Modifiers};
pub use menu::Menu;
pub use mouse::{Cursor, MouseButton, MouseButtons, MouseEvent};
pub use scale::{Scalable, Scale, ScaledArea};
pub use window::{
    IdleHandle, IdleToken, Text, TimerToken, WinHandler, WindowBuilder, WindowHandle,
};

pub use keyboard_types;
