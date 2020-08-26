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

//! Simple data-oriented GUI.
//!
//! Druid lets you build simple interactive graphical applications that
//! can be deployed on Windows, macOS, Linux, and the web.
//!
//! Druid is built on top of [`druid-shell`], which implements all of the
//! lower-level, platform-specific code, providing a common abstraction
//! for things like key and mouse events, creating windows, and launching
//! an application. Below [`druid-shell`] is [`piet`], which is a cross-platform
//! 2D graphics library, providing a simple and familiar drawing API that can be
//! implemented for various platforms.
//!
//! Druid is a data-driven, declarative framework. You describe your application
//! model in terms of the [`Data`] trait, and then you build up a tree of
//! [`widget`] s that can display and modify your data.
//!
//! Your widgets handle [`Event`]s, such as mouse movement, and can modify the data;
//! these changes are then delivered to relevant widgets, which can update
//! their state and redraw.
//!
//! As your application grows, you can use [`Lens`]es to expose only certain
//! subsets of your data model to certains subsets of your widget tree.
//!
//! For more information you should read the [druid book].
//!
//! # Examples
//!
//! For many more examples, see [`druid/examples`].
//!
//! ```no_run
//! use druid::widget::{Align, Flex, Label, TextBox};
//! use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
//!
//! const VERTICAL_WIDGET_SPACING: f64 = 20.0;
//! const TEXT_BOX_WIDTH: f64 = 200.0;
//! const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");
//!
//! #[derive(Clone, Data, Lens)]
//! struct HelloState {
//!     name: String,
//! }
//!
//! fn main() {
//!     // describe the main window
//!     let main_window = WindowDesc::new(build_root_widget)
//!         .title(WINDOW_TITLE)
//!         .window_size((400.0, 400.0));
//!
//!     // create the initial app state
//!     let initial_state = HelloState {
//!         name: "World".into(),
//!     };
//!
//!     // start the application
//!     AppLauncher::with_window(main_window)
//!         .launch(initial_state)
//!         .expect("Failed to launch application");
//! }
//!
//! fn build_root_widget() -> impl Widget<HelloState> {
//!     // a label that will determine its text based on the current app data.
//!     let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
//!     // a textbox that modifies `name`.
//!     let textbox = TextBox::new()
//!         .with_placeholder("Who are we greeting?")
//!         .fix_width(TEXT_BOX_WIDTH)
//!         .lens(HelloState::name);
//!
//!     // arrange the two widgets vertically, with some padding
//!     let layout = Flex::column()
//!         .with_child(label)
//!         .with_spacer(VERTICAL_WIDGET_SPACING)
//!         .with_child(textbox);
//!
//!     // center the two widgets in the available space
//!     Align::centered(layout)
//! }
//! ```
//!
//! # Optional Features
//!
//! * `im` - Efficient immutable data structures using the [`im` crate],
//!          which is made available via the [`im` module].
//! * `svg` - Scalable Vector Graphics for icons and other scalable images using the [`usvg` crate].
//! * `image` - Bitmap image support using the [`image` crate].
//! * `x11` - Work-in-progress X11 Linux backend instead of GTK.
//!
//! Features can be added with `cargo`. For example, in your `Cargo.toml`:
//! ```no_compile
//! [dependencies.druid]
//! version = "0.6.0"
//! features = ["im", "svg", "image"]
//! ```
//!
//! [`Widget`]: trait.Widget.html
//! [`Data`]: trait.Data.html
//! [`Lens`]: trait.Lens.html
//! [`widget`]: ./widget/index.html
//! [`Event`]: enum.Event.html
//! [`druid-shell`]: https://docs.rs/druid-shell
//! [`piet`]: https://docs.rs/piet
//! [`druid/examples`]: https://github.com/linebender/druid/tree/v0.6.0/druid/examples
//! [druid book]: https://linebender.org/druid/
//! [`im` crate]: https://crates.io/crates/im
//! [`im` module]: im/index.html
//! [`usvg` crate]: https://crates.io/crates/usvg
//! [`image` crate]: https://crates.io/crates/image

#![no_std] //  Don't link with standard Rust library, which is not compatible with embedded systems ////
#![deny(intra_doc_link_resolution_failure, /* unsafe_code */)] ////
#![allow(clippy::new_ret_no_self, clippy::needless_doctest_main)]
#![deny(clippy::trivially_copy_pass_by_ref)]
#![cfg_attr(docsrs, feature(doc_cfg))]

////Begin
use ::core::fmt;
use ::core::marker::PhantomData;

type StringLength = heapless::consts::U20; //// Max length of strings
type String = heapless::String::<StringLength>; //// Alias for standard String

type VecLength = heapless::consts::U10; //// Max length of vectors
type Vec<T> = heapless::Vec::<T, VecLength>; //// Alias for standard Vec

/// Numeric type for screen coordinates
pub type ScreenCoord = u8;  //  Previously f64

/// Numeric type for flex factors
pub type ScreenFactor = f32;  //  Previously f64

/// Numeric type for Widget Id
pub type CounterType = u8;  //  Previously u64

/// A 2D point.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Point { ////
    /// The x coordinate.
    pub x: ScreenCoord,
    /// The y coordinate.
    pub y: ScreenCoord,
}
impl Point {
    pub const ORIGIN: Point = Point { x: 0, y: 0 };
}

/// A 2D size.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Size { ////
    /// The width.
    pub width: ScreenCoord,
    /// The height.
    pub height: ScreenCoord,
}
impl Size {
    pub fn new(width: ScreenCoord, height: ScreenCoord) -> Self { Size{ width, height } }
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
impl Rect {
    pub const ZERO: Rect = Rect{ x0: 0, y0: 0, x1: 0, y1: 0 };
    pub fn from_origin_size(point: Point, size: Size) -> Rect { 
        Rect { 
            x0: point.x, 
            y0: point.y,
            x1: point.x + size.width - 1,
            y1: point.y + size.height - 1,
        }
    }
    pub fn size(self) -> Size {
        Size {
            width:  self.x1 - self.x0 + 1,
            height: self.y1 - self.y0 + 1,
        }
    }
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

/// A single line.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line { ////
    /// The line's start point.
    pub p0: Point,
    /// The line's end point.
    pub p1: Point,
}
impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

/// A datatype representing color.
///
/// Currently this is only a 32 bit RGBA value, but it will likely
/// extend to some form of wide-gamut colorspace, and in the meantime
/// is useful for giving programs proper type.
#[derive(Clone)]
pub enum Color {
    Rgba32(u32),
}
////End

// Allows to use macros from druid_derive in this crate
extern crate self as druid;
pub use druid_derive::Lens;

////use druid_shell as shell;
#[doc(inline)]
////pub use druid_shell::{kurbo, piet};

// the im crate provides immutable data structures that play well with druid
#[cfg(feature = "im")]
#[doc(inline)]
pub use im;

mod app;
////mod app_delegate;
////mod bloom;
mod box_constraints;
////mod command;
////mod contexts;
mod core;
mod data;
////mod env;
mod event;
////mod ext_event;
////pub mod lens;
////mod localization;
////mod menu;
////mod mouse;
////#[cfg(not(target_arch = "wasm32"))]
////#[cfg(test)]
////mod tests;
////pub mod text;
////pub mod theme;
////mod util;
pub mod widget;
////mod win_handler;
////mod window;

// Types from kurbo & piet that are required by public API.
////pub use kurbo::{Affine, Insets, Point, Rect, Size, Vec2};
////pub use piet::{Color, LinearGradient, RadialGradient, RenderContext, UnitPoint};
// these are the types from shell that we expose; others we only use internally.
////pub use shell::keyboard_types;
////pub use shell::{
    ////Application, Clipboard, ClipboardFormat, Code, Cursor, Error as PlatformError,
    ////FileDialogOptions, FileInfo, FileSpec, FormatId, HotKey, KbKey, KeyEvent, Location, Modifiers,
    ////MouseButton, MouseButtons, RawMods, Scalable, Scale, SysMods, Text, TimerToken, WindowHandle,
////};

pub use crate::core::WidgetPod;
pub use app::{AppLauncher, WindowDesc};
////pub use app_delegate::{AppDelegate, DelegateCtx};
pub use box_constraints::BoxConstraints;
////pub use command::{sys as commands, Command, Selector, SingleUse, Target};
////pub use contexts::{EventCtx, LayoutCtx, LifeCycleCtx, PaintCtx, Region, UpdateCtx};
pub use data::Data;
////pub use env::{Env, Key, KeyOrValue, Value, ValueType};
pub use event::{Event, InternalEvent, InternalLifeCycle, LifeCycle};
////pub use ext_event::{ExtEventError, ExtEventSink};
////pub use lens::{Lens, LensExt, LensWrap};
////pub use localization::LocalizedString;
////pub use menu::{sys as platform_menus, ContextMenu, MenuDesc, MenuItem};
////pub use mouse::MouseEvent;
pub use widget::{Widget, 
    ////WidgetExt, 
WidgetId};
pub use widget::BoxedWidget; ////
////pub use win_handler::DruidHandler;
////pub use window::{Window, WindowId};

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
////pub(crate) use event::{StateCell, StateCheckFn};

/// The meaning (mapped value) of a keypress.
///
/// Note that in previous versions, the `KeyCode` field referred to the
/// physical position of the key, rather than the mapped value. In most
/// cases, applications should dispatch based on the value instead. This
/// alias is provided to make that transition easy, but in any case make
/// an explicit choice whether to use meaning or physical location and
/// use the appropriate type.
#[deprecated(since = "0.7.0", note = "Use KbKey instead")]
pub type KeyCode = KbKey;

#[deprecated(since = "0.7.0", note = "Use Modifiers instead")]
pub type KeyModifiers = Modifiers;

//// Begin

#[derive(Copy, Clone)]
pub struct Application<T>(Option<T>);
impl<T> Application<T> {
    pub fn new() -> Result<Self, PlatformError> { Ok(Self(None)) }
    pub fn run(self, _: Option<BoxedAppHandler<T>>) {} ////TODO
}

pub trait AppDelegate<T> {}

#[derive(Copy, Clone)]
pub struct AppHandler<T> {
    state: AppState<T>    
}
impl<T> AppHandler<T> {
    pub fn new(state: AppState<T>) -> Self { Self{ state } }
}

#[derive(Copy, Clone)]
pub struct AppState<T> {
    app: Application<T>,
    data: T,
    env: Env,
    delegate: Option<BoxedAppDelegate<T>>,
    ext_event_host: ExtEventHost,    
}
impl<T> AppState<T> {
    pub fn new(
        app: Application<T>,
        data: T,
        env: Env,
        delegate: Option<BoxedAppDelegate<T>>,
        ext_event_host: ExtEventHost,    
    ) -> Self { 
        Self{ app, data, env, delegate, ext_event_host }
    }
    pub fn app(self) -> Application<T> { self.app }
    pub fn data(self) -> T { self.data }
    pub fn env(self) -> Env { self.env }
    pub fn add_window(self, id: WindowId, window: WindowDesc<T>) {}
}

#[derive(Copy, Clone)]
pub struct Bloom<T>(Option<T>);

#[derive(Copy, Clone)]
pub struct BoxedAppDelegate<T> (Option<T>);
impl<T> BoxedAppDelegate<T> {
    pub fn new(delegate: impl AppDelegate<T> + 'static) -> Self { Self(None) }
}

#[derive(Copy, Clone)]
pub struct BoxedAppHandler<T> (Option<T>);
impl<T> BoxedAppHandler<T> {
    pub fn new(_handler: AppHandler<T>) -> Self { Self(None) }
}

#[derive(Copy, Clone)]
pub struct BoxedDruidHandler<T> (Option<T>);
impl<T> BoxedDruidHandler<T> {
    pub fn new(_handler: DruidHandler<T>) -> Self { Self(None) }
}

#[derive(Copy, Clone)]
pub struct BoxedEnvSetupFn<T> (Option<T>);
impl<T> BoxedEnvSetupFn<T> {
    pub fn new(_f: impl Fn(&mut Env, &T) + 'static) -> Self { Self(None) }
}

#[derive(Copy, Clone)]
pub struct BoxedText (
    //  Fn(&T, &Env) -> String,
);
impl BoxedText {
    pub fn new() -> BoxedText { BoxedText{} }
    pub fn resolve<T>(self, data: T, env: &Env) -> String { String::new() }
}

#[derive(Copy, Clone)]
pub struct Clipboard();

#[derive(Copy, Clone)]
pub struct Command();

#[derive(Copy, Clone)]
pub struct ContextState();

#[derive(Copy, Clone)]
pub struct Counter();
impl Counter {
    pub fn new() -> Self { Counter{}}
    pub fn next_nonzero(self) -> CounterType {
        let count = COUNTER;
        unsafe { COUNTER += 1 };
        count
    }
}

static mut COUNTER: u8 = 1;

#[derive(Copy, Clone)]
pub struct DruidHandler<T> {
    state: AppState<T>,
    id: WindowId,
}
impl<T> DruidHandler<T> {
    pub fn new_shared(state: AppState<T>, id: WindowId) -> Self { Self{ state, id } }
}

#[derive(Copy, Clone)]
pub struct Env();

#[derive(Copy, Clone)]
pub struct EventCtx();

#[derive(Copy, Clone)]
pub struct ExtEventHost();
impl ExtEventHost {
    pub fn new() -> Self { Self{} }
    pub fn make_sink(self) -> ExtEventSink { ExtEventSink{} }
}

#[derive(Copy, Clone)]
pub struct ExtEventSink();
impl ExtEventSink {
    pub fn new() -> Self { Self{} }
}

#[derive(Copy, Clone)]
pub struct KeyOrValue<T>(T);
impl<T> KeyOrValue<T> {
    pub fn resolve(self, env: &Env) -> T { self.0 } 
}

#[derive(Copy, Clone)]
pub struct HashMap<K, V>(Option<K>, Option<V>);

#[derive(Copy, Clone)]
pub struct LayoutCtx();
impl LayoutCtx {
    pub fn text(self) -> PietText { PietText{} }
}

#[derive(Copy, Clone)]
pub struct LifeCycleCtx();

#[derive(Copy, Clone)]
pub struct LocalizedString<T>(Option<T>);
impl<T> LocalizedString<T> {
    pub fn new(_app_name: &str) -> Self { Self(None) }
    pub fn localized_str(self) -> &'static str { "localized_str" }
    pub fn resolve(self, data: &T, env: &Env) -> bool { true }
}

#[derive(Copy, Clone)]
pub struct KbKey();

#[derive(Copy, Clone)]
pub struct KeyEvent();

#[derive(Copy, Clone)]
pub struct Modifiers();

#[derive(Copy, Clone)]
pub struct MouseEvent();

#[derive(Copy, Clone)]
pub struct MenuDesc<T>(Option<T>);
impl<T> MenuDesc<T> {
    pub fn platform_default() -> Option<Self> { Some(Self(None)) }
    pub fn build_window_menu(self, data: &T, env: &Env) -> MenuDesc<T> { MenuDesc(None) }
}

#[derive(Copy, Clone)]
pub struct NonZeroU64();

#[derive(Copy, Clone)]
pub struct PaintCtx();

#[derive(Copy, Clone)]
pub struct PietText();
impl PietText {

}

#[derive(Copy, Clone)]
pub struct PietTextLayout();
impl PietTextLayout {
    pub fn width(self) -> ScreenCoord { 10 }  ////TODO
}

#[derive(Copy, Clone)]
pub struct Region();

#[derive(Copy, Clone)]
pub struct Shape();

#[derive(Copy, Clone)]
pub struct SizedBox {
    width:  ScreenCoord,
    height: ScreenCoord,
}
impl SizedBox {
    pub fn empty() -> Self { Self{ width: 0, height: 0 } }
    pub fn expand_width(self) -> Self { self }  ////TODO
    pub fn expand_height(self) -> Self { self }  ////TODO
}

#[derive(Copy, Clone)]
pub struct Target();

#[derive(Copy, Clone)]
pub struct TimerToken();

#[derive(Copy, Clone)]
pub struct UnitPoint();

#[derive(Copy, Clone)]
pub struct UpdateCtx();
impl UpdateCtx {
    pub fn request_layout(self) {}  ////TODO
}

#[derive(Copy, Clone)]
pub struct VecDeque<T>(Option<T>);

#[derive(Copy, Clone)]
pub struct WindowBuilder<T> {
    app: Application<T>,
}
impl<T> WindowBuilder<T> {
    pub fn new(app: Application<T>) -> Self { Self{ app } }
    pub fn build(self) -> Result<WindowHandle, PlatformError> { Ok(WindowHandle{}) }  ////TODO
    pub fn resizable(self, resizable: bool) {}
    pub fn show_titlebar(self, show_titlebar: bool) {}
    pub fn set_handler(self, handler: BoxedDruidHandler<T>) {}
    pub fn set_size(self, size: Size) {}
    pub fn set_min_size(self, min_size: Size) {}
    pub fn set_title(self, title: &str) {}
    pub fn set_menu(self, menu: MenuDesc<T>) {}
}

#[derive(Copy, Clone)]
pub struct WindowHandle();
impl WindowHandle {
    pub fn show(self) { }  ////TODO
}

#[derive(Copy, Clone)]
pub struct WindowId();
impl WindowId {
    pub fn next() -> Self { Self{} }  ////TODO
}

pub mod theme {
    use crate::{ Color, Env, KeyOrValue, ScreenFactor };
    pub fn init() -> Env { Env{} }
    pub static LABEL_COLOR: KeyOrValue<Color> = KeyOrValue(Color::Rgba32(0xffffff));
    pub static TEXT_SIZE_NORMAL: KeyOrValue<ScreenFactor> = KeyOrValue(1.0);
    pub static FONT_NAME: KeyOrValue<&'static str> = KeyOrValue("standard_font");
}

pub type PlatformError = String; ////

mod hello; ////

//// End
