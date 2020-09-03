//! `BoxedWidget` contains a `Widget`. Allows for dynamic dispatch with static `Widgets` in `[no_std]`.
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, Size, UpdateCtx, Widget, WidgetId,
    widget::{Align, Flex, Label, Padding, SizedBox, Spacer},
};

/// Max number of `Widgets` on embedded platforms
pub const MAX_WIDGETS: usize = 10;

/// Specialised Trait for handling static `Widgets` on embedded platforms
pub trait StaticWidgets<D: Clone /* Data + 'static + Default */> {
    /// Fetch the static `Widgets` for the Data type
    fn get_widgets(&self) -> &'static mut [ WidgetType<D> ];
    /// Add a `Widget` for the Data type
    fn add_widget(&self, widget: WidgetType<D>);
}

/// Default Trait will not have static `Widgets`
impl<D: Clone /* Data + 'static + Default */> StaticWidgets<D> for BoxedWidget<D> {
    default fn get_widgets(&self) -> &'static mut [ WidgetType<D> ] { panic!("no global widgets") }
    default fn add_widget(&self, _widget: WidgetType<D>) { panic!("no global widgets") }
}

#[derive(Clone)]
pub struct BoxedWidget<D> (
    pub WidgetId,
    pub Option<D> ////TODO: Remove this
);

/// Generic implementation of `BoxedWidget`
impl<D: Clone> BoxedWidget<D> {
    /// Create a new box for the `Widget`
    pub fn new<W: Widget<D> + Clone>(widget: W) -> Self {
        let id = widget.id().unwrap();
        let boxed_widget = Self::new_by_id(id);
        let widget_type: WidgetType<D> = widget.to_type();
        boxed_widget.clone().add_widget(widget_type);
        boxed_widget
    }
    pub fn new_by_id(id: WidgetId) -> Self {
        BoxedWidget(
            id,
            None
        ) 
    }
}

impl<D: Data> Widget<D> for BoxedWidget<D> { ////
////impl<D> Widget<D> for Box<dyn Widget<D>> {

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut D, env: &Env) {
        match &mut self.get_widgets()[self.0.0 as usize] {
            WidgetType::Align(w)   => w.event(ctx, event, data, env),
            //  WidgetType::Button(w)  => w.event(ctx, event, data, env),
            WidgetType::Flex(w)    => w.event(ctx, event, data, env),
            WidgetType::Label(w)   => w.event(ctx, event, data, env),
            WidgetType::Padding(w) => w.event(ctx, event, data, env),
            WidgetType::SizedBox(w) => w.event(ctx, event, data, env),
            WidgetType::None => {}
        };
    }

    /*  Called by
        impl<T: Data> WinHandler for DruidHandler<D> {
            fn connect(&mut self, handle: &WindowHandle) {
                self.app_state
                    .connect_window(self.window_id, handle.clone());

                let event = Event::WindowConnected;
                self.app_state.do_window_event(event, self.window_id);
            }
    */
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &D, env: &Env) {
        match &mut self.get_widgets()[self.0.0 as usize] {
            WidgetType::Align(w)   => w.lifecycle(ctx, event, data, env),
            //  WidgetType::Button(w)  => w.lifecycle(ctx, event, data, env),
            WidgetType::Flex(w)    => w.lifecycle(ctx, event, data, env),
            WidgetType::Label(w)   => w.lifecycle(ctx, event, data, env),
            WidgetType::Padding(w) => w.lifecycle(ctx, event, data, env),
            WidgetType::SizedBox(w) => w.lifecycle(ctx, event, data, env),
            WidgetType::None => {}
        };
    }
    
    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &D, data: &D, env: &Env) {
        match &mut self.get_widgets()[self.0.0 as usize] {
            WidgetType::Align(w)   => w.update(ctx, old_data, data, env),
            //  WidgetType::Button(w)  => w.update(ctx, old_data, data, env),
            WidgetType::Flex(w)    => w.update(ctx, old_data, data, env),
            WidgetType::Label(w)   => w.update(ctx, old_data, data, env),
            WidgetType::Padding(w) => w.update(ctx, old_data, data, env),
            WidgetType::SizedBox(w) => w.update(ctx, old_data, data, env),
            WidgetType::None => {}
        };
    }
    
    /*  Called by
        impl<T: Data> WinHandler for DruidHandler<D> {
            fn prepare_paint(&mut self) {
                self.app_state.prepare_paint_window(self.window_id);
            }
    */
    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &D, env: &Env) -> Size {
        match &mut self.get_widgets()[self.0.0 as usize] {
            WidgetType::Align(w)   => w.layout(ctx, bc, data, env),
            //  WidgetType::Button(w)  => w.layout(ctx, bc, data, env),
            WidgetType::Flex(w)    => w.layout(ctx, bc, data, env),
            WidgetType::Label(w)   => w.layout(ctx, bc, data, env),
            WidgetType::Padding(w) => w.layout(ctx, bc, data, env),
            WidgetType::SizedBox(w) => w.layout(ctx, bc, data, env),
            WidgetType::None => Size::ZERO
        }
    }
    
    /*  Called by
        impl<T: Data> WinHandler for DruidHandler<D> {
            fn paint(&mut self, piet: &mut Piet, region: &Region) {
                self.app_state.paint_window(self.window_id, piet, region);
            }        
    */
    fn paint(&mut self, ctx: &mut PaintCtx, data: &D, env: &Env) {
        match &mut self.get_widgets()[self.0.0 as usize] {
            WidgetType::Align(w)   => w.paint(ctx, data, env),
            //  WidgetType::Button(w)  => w.paint(ctx, data, env),
            WidgetType::Flex(w)    => w.paint(ctx, data, env),
            WidgetType::Label(w)   => w.paint(ctx, data, env),
            WidgetType::Padding(w) => w.paint(ctx, data, env),
            WidgetType::SizedBox(w) => w.paint(ctx, data, env),
            WidgetType::None => {}
        };
    }
    
    fn id(&self) -> Option<WidgetId> {
        Some(self.0)
    }

    fn type_name(&self) -> &'static str {
        "Unknown" ////TODO
    }

    fn to_type(self) -> WidgetType<D> {
        WidgetType::None
    }
}

/*
/// Default WidgetType is None
impl<D: Data + 'static + Default> Default for WidgetType<D> {
    fn default() -> Self { WidgetType::None }
}
*/

/// Enum to store each `Widget`
#[derive(Clone)]
pub enum WidgetType<D: Clone /* Data + 'static + Default */> {
    None,
    Align(Align<D>),
    //  Button(Button<D>),
    Flex(Flex<D>),
    Label(Label<D>),
    Padding(Padding<D>),
    SizedBox(SizedBox<D>),
    ////Spacer(Spacer<D>), ////TODO
}

////////////////////////////// TODO: Generate via Data trait

type State = ();

/// DATA is the Application Data
//// static mut DATA_STATE: State = (); ////TODO State { count: 0 };  //  Generated based on `State`

/// Static list of Widgets for embedded platforms
static mut ALL_WIDGETS_STATE: [ druid::WidgetType<State>; druid::MAX_WIDGETS ] = [ 
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
    druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None, druid::WidgetType::None,
];

/*
/// ALL_WINDOWS[i] is the WindowBox for the Window with window ID i. i=0 is not used.
static mut ALL_WINDOWS_STATE: [ druid::WindowBox<State>; druid::MAX_WINDOWS ] = [
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
    druid::WindowBox::<State>( druid::WindowType::None ), 
];

/// ALL_HANDLERS[i] is the Window Handler for the Window with window ID i. i=0 is not used.
static mut ALL_HANDLERS_STATE: [ druid::DruidHandler<State>; druid::MAX_WINDOWS ] = [
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
    druid::DruidHandler::<State> { window_id: druid::WindowId(0), phantom: core::marker::PhantomData },
];
*/

/// Specialised Trait to reference Widgets statically on embedded platforms
impl druid::StaticWidgets<State> for druid::BoxedWidget<State> {
    /// Fetch the static Widgets for the Data type
    fn get_widgets(&self) -> &'static mut [ druid::WidgetType<State> ] {
        unsafe { &mut ALL_WIDGETS_STATE }
    }
    /// Add a Widget for the Data type
    fn add_widget(&self, widget: druid::WidgetType<State>) {
        assert!((self.0.0 as usize)< druid::MAX_WIDGETS, "too many widgets");
        unsafe { ALL_WIDGETS_STATE[self.0.0 as usize] = widget; }        
    }    
}

/*
/// Specialised Trait to reference Windows and Window Handlers statically on embedded platforms
impl druid::GlobalWindows<State> for druid::AppState<State> {
    fn add_window(&self, window_id: druid::WindowId, window: druid::WindowBox<State>) {
        unsafe { ALL_WINDOWS_STATE[window_id.0 as usize] = window; }
    }
    fn add_handler(&self, window_id: druid::WindowId, handler: druid::DruidHandler<State>) {
        unsafe { ALL_HANDLERS_STATE[window_id.0 as usize] = handler; }
    }
    fn get_handle(&self, window_id: druid::WindowId) -> druid::WindowHandle<druid::DruidHandler<State>> {
        let handler = unsafe { ALL_HANDLERS_STATE[window_id.0 as usize].clone() };
        druid::WindowHandle(
            druid::PlatformWindowHandle {
                window_id: window_id.0,
                state: druid::PlatformWindowState {
                    window_id: window_id.0,
                    handler,                
                }            
            }
        )
    }
    fn set_data(&self, data: State) {
        unsafe { DATA_STATE = data; }
    }
    fn window_event(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::EventCtx<State>, 
        event: &druid::Event, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].event(
                ctx, 
                event, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            );
        }
    }
    fn window_update(
        &mut self, 
        window_id: druid::WindowId,
        ctx: &mut druid::UpdateCtx<State>, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].update(
                ctx,
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_layout(
        &mut self,
        window_id: druid::WindowId,
        layout_ctx: &mut druid::LayoutCtx,
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].layout(
                layout_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_paint(
        &mut self, 
        window_id: druid::WindowId,
        paint_ctx: &mut druid::PaintCtx, 
    ) {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].paint(
                paint_ctx, 
                &mut DATA_STATE,  //  Data
                &Env {}           //  Env
            ); 
        }
    }
    fn window_has_active(
        &mut self,
        window_id: druid::WindowId,
    ) -> bool {
        unsafe { 
            ALL_WINDOWS_STATE[window_id.0 as usize].has_active() 
        }
    }
}
*/
