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

//! Common widgets.

mod align;
////mod button;
////mod checkbox;
////mod click;
mod common;
////mod container;
////mod controller;
////mod either;
////mod env_scope;
mod flex;
////mod identity_wrapper;
////mod image;
////mod invalidation;
mod label;
////mod list;
mod padding;
////mod painter;
////mod parse;
////mod progress_bar;
////mod radio;
////mod scroll;
mod sized_box;
////mod slider;
////mod spinner;
////mod split;
////mod stepper;
#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
mod svg;
////mod switch;
////mod textbox;
////mod view_switcher;
#[allow(clippy::module_inception)]
mod widget;
////mod widget_ext;

////pub use self::image::{Image, ImageData};
pub use align::Align;
////pub use button::Button;
////pub use checkbox::Checkbox;
////pub use click::Click;
pub use common::FillStrat;
////pub use container::Container;
////pub use controller::{Controller, ControllerHost};
////pub use either::Either;
////pub use env_scope::EnvScope;
pub use flex::{CrossAxisAlignment, Flex, FlexParams, MainAxisAlignment};
////pub use identity_wrapper::IdentityWrapper;
pub use label::{Label, LabelText};
////pub use list::{List, ListIter};
pub use padding::Padding;
////pub use painter::{BackgroundBrush, Painter};
////pub use parse::Parse;
////pub use progress_bar::ProgressBar;
////pub use radio::{Radio, RadioGroup};
////pub use scroll::Scroll;
pub use sized_box::SizedBox;
////pub use slider::Slider;
////pub use spinner::Spinner;
////pub use split::Split;
////pub use stepper::Stepper;
#[cfg(feature = "svg")]
pub use svg::{Svg, SvgData};
////pub use switch::Switch;
////pub use textbox::TextBox;
////pub use view_switcher::ViewSwitcher;
#[doc(hidden)]
pub use widget::{Widget, WidgetId};
#[doc(hidden)]
////pub use widget_ext::WidgetExt;

/// The types required to implement a `Widget`.
///
/// # Structs
/// [`BoxConstraints`](../../struct.BoxConstraints.html)\
/// [`Env`](../../struct.Env.html)\
/// [`EventCtx`](../../struct.EventCtx.html)\
/// [`LayoutCtx`](../../struct.LayoutCtx.html)\
/// [`LifeCycleCtx`](../../struct.LifeCycleCtx.html)\
/// [`PaintCtx`](../../struct.PaintCtx.html)\
/// [`Size`](../../struct.Size.html)\
/// [`UpdateCtx`](../../struct.UpdateCtx.html)\
/// [`WidgetId`](../../struct.WidgetId.html)\
///
/// # Enums
/// [`Event`](../../enum.Event.html)\
/// [`LifeCycle`](../../enum.LifeCycle.html)\
///
/// # Traits
/// [`RenderContext`](../../trait.RenderContext.html)\
/// [`Widget`](../../trait.Widget.html)
// NOTE: \ at the end works as a line break, but skip on last line!
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
        ////RenderContext, 
        Size, UpdateCtx, Widget, WidgetId,
    };
}

//// Begin
use crate::{ BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx };

#[derive(Clone)]
pub struct BoxedWidget<T> (
    Option<T> ////TODO
);

impl<T> BoxedWidget<T> {
    pub fn new(child: impl Widget<T>) -> Self { BoxedWidget(None) }  ////TODO
    pub fn deref(&self) -> &Self { &self.clone() }
    pub fn deref_mut(&self) -> &Self { &self.clone() }
}

impl<T> Widget<T> for BoxedWidget<T> { ////
////impl<T> Widget<T> for Box<dyn Widget<T>> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        ////TODO
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        ////TODO
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        ////TODO
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        Size::ZERO ////TODO
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ////TODO
    }

    fn id(&self) -> Option<WidgetId> {
        None ////TODO
    }

    fn type_name(&self) -> &'static str {
        "Unknown" ////TODO
    }
}
//// End
