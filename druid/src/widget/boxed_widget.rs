//! `BoxedWidget` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetId,
    widget::{Align, Flex, Label, Padding},
};

/// Max number of `Widgets` on embedded platforms
pub const MAX_WIDGETS: usize = 10;

/*
/// Specialised Trait for handling static `Widgets` on embedded platforms
pub trait StaticWidgets<D: Data + 'static + Default> {
    /// Fetch the static `Widgets` for the Data type
    fn get_widgets(&self) -> &'static mut [ WidgetType<D> ];
    /// Add a `Widget` for the Data type
    fn add_widget(&self, widget: WidgetType<D>);
}
*/

/*
/// Default Trait will not have static `Widgets`
impl<D: Data + 'static + Default> StaticWidgets<D> for BoxedWidget<D> {
    default fn get_widgets(&self) -> &'static mut [ WidgetType<D> ] { panic!("no global widgets") }
    default fn add_widget(&self, _widget: WidgetType<D>) { panic!("no global widgets") }
}
*/

/*
/// Boxed version of a `Widget`
#[derive(Clone, Default)]
pub struct BoxedWidget<D: Data + 'static>(
    pub WidgetId,    //  Widget ID
    PhantomData<D>,  //  Needed to do compile-time checking for `Data`
);
*/

#[derive(Clone)]
pub struct BoxedWidget<T> (
    pub WidgetId,
    pub Option<T> ////TODO
);

impl<T> BoxedWidget<T> {
    pub fn new(child: impl Widget<T>) -> Self { 
        Self::new_by_id(
            child.id().unwrap()
        )
    }
    pub fn new_by_id(id: WidgetId) -> Self {
        BoxedWidget(
            id,
            None
        ) 
    }
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
        /* impl<T: Data> WinHandler for DruidHandler<T> {
            fn connect(&mut self, handle: &WindowHandle) {
                self.app_state
                    .connect_window(self.window_id, handle.clone());

                let event = Event::WindowConnected;
                self.app_state.do_window_event(event, self.window_id);
            }
        */
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        ////TODO
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        /* impl<T: Data> WinHandler for DruidHandler<T> {
            fn prepare_paint(&mut self) {
                self.app_state.prepare_paint_window(self.window_id);
            }
        */
        Size::ZERO ////TODO
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ////TODO
        /* impl<T: Data> WinHandler for DruidHandler<T> {
            fn paint(&mut self, piet: &mut Piet, region: &Region) {
                self.app_state.paint_window(self.window_id, piet, region);
            }        
        */
    }

    fn id(&self) -> Option<WidgetId> {
        Some(self.0)
    }

    fn type_name(&self) -> &'static str {
        "Unknown" ////TODO
    }
}

/*
/// Enum to store each `Widget`
#[derive(Clone)]
pub enum WidgetType<D: Clone /* Data + 'static + Default */> {
    None,
    Align(Align<D>),
    //  Button(Button<D>),
    Flex(Flex<D>),
    Label(Label<D>),
    Padding(Padding<D>),
}
*/

/*
impl<D: Data + 'static + Default> Default for WidgetType<D> {
    fn default() -> Self { WidgetType::None }
}
*/

/*
/// Generic implementation of `BoxedWidget`
impl<D: Data + 'static + Default> BoxedWidget<D> {
    /// Create a new box for the `Widget`
    pub fn new<W: Widget<D> + Clone>(widget: W) -> Self {
        let id = widget.clone().get_id();
        let widget_type: WidgetType<D> = widget.to_type();
        let widget_box: BoxedWidget<D> = BoxedWidget(
            id,
            PhantomData,
        );
        widget_box.clone().add_widget(widget_type);
        widget_box
    }
}

/// Implementation of `Widget` trait for `BoxedWidget`. We just forward to the inner `Widget`.
impl<D: Data + 'static + Default> Widget<D> for BoxedWidget<D> {
    fn paint(
        &mut self, 
        paint_ctx: &mut PaintCtx, 
        base_state: &BaseState, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Button(w)  => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Flex(w)    => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Label(w)   => w.paint(paint_ctx, base_state, data, env),
            WidgetType::Padding(w) => w.paint(paint_ctx, base_state, data, env),
            WidgetType::None => {}
        };
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &D,
        env: &Env,
    ) -> Size {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Button(w)  => w.layout(layout_ctx, bc, data, env),
            WidgetType::Flex(w)    => w.layout(layout_ctx, bc, data, env),
            WidgetType::Label(w)   => w.layout(layout_ctx, bc, data, env),
            WidgetType::Padding(w) => w.layout(layout_ctx, bc, data, env),
            WidgetType::None => Size::ZERO,
        }
    }

    fn event(
        &mut self, 
        ctx: &mut EventCtx<D>, 
        event: &Event, 
        data: &mut D, 
        env: &Env
    ) {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.event(ctx, event, data, env),
            WidgetType::Button(w)  => w.event(ctx, event, data, env),
            WidgetType::Flex(w)    => w.event(ctx, event, data, env),
            WidgetType::Label(w)   => w.event(ctx, event, data, env),
            WidgetType::Padding(w) => w.event(ctx, event, data, env),
            WidgetType::None => {}
        };
    }

    fn update(
        &mut self, 
        ctx: &mut UpdateCtx<D>, 
        old_data: Option<&D>, 
        data: &D, 
        env: &Env
    ) {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Button(w)  => w.update(ctx, old_data, data, env),
            WidgetType::Flex(w)    => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Padding(w) => w.update(ctx, old_data, data, env),
            WidgetType::None => {}
        };
    }

    fn to_type(self) -> WidgetType<D> {
        WidgetType::None
    }

    fn new_window(self) -> WindowBox<D> {
        WindowBox::new()
    }

    fn get_id(self) -> WidgetId {
        match &mut self.get_widgets()[self.0 as usize] {
            WidgetType::Align(w)   => w.clone().get_id(),
            WidgetType::Button(w)  => w.clone().get_id(),
            WidgetType::Flex(w)    => w.clone().get_id(),
            WidgetType::Label(w)   => w.clone().get_id(),
            WidgetType::Padding(w) => w.clone().get_id(),
            WidgetType::None => panic!("no id")
        }
    }
}
*/