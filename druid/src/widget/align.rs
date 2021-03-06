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

//! A widget that aligns its child (for example, centering it).

use crate::{Rect, Size}; ////
////use crate::kurbo::{Rect, Size};
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    UpdateCtx, Widget, WidgetPod,
};

////use crate::piet::UnitPoint;
use crate::{BoxedWidget, ScreenCoord, ScreenFactor, UnitPoint, WidgetId, WidgetType}; ////

/// A widget that aligns its child.
#[derive(Clone)] ////
pub struct Align<T> {
    id: WidgetId, ////
    align: UnitPoint,
    child: WidgetPod<T, BoxedWidget<T>>, ////
    ////child: WidgetPod<T, Box<dyn Widget<T>>>,
    width_factor: Option<ScreenFactor>, ////
    ////width_factor: Option<f64>,
    height_factor: Option<ScreenFactor>, ////
    ////height_factor: Option<f64>,
}

impl<T: Data + Clone> Align<T> { ////
////impl<T> Align<T> {
    /// Create widget with alignment.
    ///
    /// Note that the `align` parameter is specified as a `UnitPoint` in
    /// terms of left and right. This is inadequate for bidi-aware layout
    /// and thus the API will change when druid gains bidi capability.
    pub fn new(align: UnitPoint, child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn new(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: WidgetId::next(), ////
            align,
            child: WidgetPod::new(child).boxed(),
            width_factor: None,
            height_factor: None,
        }
    }

    /// Create centered widget.
    pub fn centered(child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn centered(child: impl Widget<T> + 'static) -> Align<T> {
        Align::new(UnitPoint::CENTER, child)
    }

    /// Create right-aligned widget.
    pub fn right(child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn right(child: impl Widget<T> + 'static) -> Align<T> {
        Align::new(UnitPoint::RIGHT, child)
    }

    /// Create left-aligned widget.
    pub fn left(child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn left(child: impl Widget<T> + 'static) -> Align<T> 
        Align::new(UnitPoint::LEFT, child)
    }

    /// Align only in the horizontal axis, keeping the child's size in the vertical.
    pub fn horizontal(align: UnitPoint, child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn horizontal(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: WidgetId::next(), ////
            align,
            child: WidgetPod::new(child).boxed(),
            width_factor: None,
            height_factor: Some(1.0),
        }
    }

    /// Align only in the vertical axis, keeping the child's size in the horizontal.
    pub fn vertical(align: UnitPoint, child: impl Widget<T> + 'static + Clone) -> Align<T> { ////
    ////pub fn vertical(align: UnitPoint, child: impl Widget<T> + 'static) -> Align<T> {
        Align {
            id: WidgetId::next(), ////
            align,
            child: WidgetPod::new(child).boxed(),
            width_factor: Some(1.0),
            height_factor: None,
        }
    }
}

impl<T: Data> Widget<T> for Align<T> {
    fn id(&self) -> Option<WidgetId> { Some(self.id) } ////
    
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env)
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Align");

        let size = self.child.layout(ctx, &bc.loosen(), data, env);

        log_size_warnings(size);

        let mut my_size = size;
        if bc.is_width_bounded() {
            my_size.width = bc.max().width;
        }
        if bc.is_height_bounded() {
            my_size.height = bc.max().height;
        }

        if let Some(width) = self.width_factor {
            my_size.width = (size.width as ScreenFactor * width) as ScreenCoord; ////
            ////my_size.width = size.width * width;
        }
        if let Some(height) = self.height_factor {
            my_size.height = (size.height as ScreenFactor * height) as ScreenCoord; ////
            ////my_size.height = size.height * height;
        }

        my_size = bc.constrain(my_size);
        let extra_width = (my_size.width - size.width).max(0); ////
        ////let extra_width = (my_size.width - size.width).max(0.);
        let extra_height = (my_size.height - size.height).max(0); ////
        ////let extra_height = (my_size.height - size.height).max(0.);
        let origin = self
            .align
            .resolve(Rect::new(0, 0, extra_width, extra_height)); ////
            ////.resolve(Rect::new(0., 0., extra_width, extra_height));
        self.child
            .set_layout_rect(ctx, data, env, Rect::from_origin_size(origin, size));

        let my_insets = self.child.compute_parent_paint_insets(my_size);
        ctx.set_paint_insets(my_insets);
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.child.paint(ctx, data, env);
    }

    fn to_type(self) -> WidgetType<T> { ////
        WidgetType::Align(self)
    }
}

fn log_size_warnings(size: Size) {
    if size.width == ScreenCoord::MAX { ////
    ////if size.width.is_infinite() {
        log::warn!("Align widget's child has an infinite width.");
    }

    if size.height == ScreenCoord::MAX { ////
    ////if size.height.is_infinite() {
            log::warn!("Align widget's child has an infinite height.");
    }
}
