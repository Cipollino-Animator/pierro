
use std::num::NonZeroU32;

use femtovg::{renderer::OpenGl, Canvas};
use glutin::{config::ConfigTemplateBuilder, context::{ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor, PossiblyCurrentContext}, display::{GetGlDisplay, GlDisplay}, surface::{Surface, WindowSurface}};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::{event_loop::EventLoop, window::{Window, WindowBuilder}};

use crate::{state::WidgetState, WidgetNode};
use self::input::Input;

use super::App;

mod render;
mod event;
mod input;

struct Runtime<S> {
    window: Window,
    canvas: Canvas<OpenGl>,
    surface: Surface<WindowSurface>,
    gl_ctx: PossiblyCurrentContext,
    input: Input,

    text_font: femtovg::FontId,

    ui: Box<dyn Fn(&S) -> WidgetNode<S>>,
    state: S,
    widget_state: WidgetState<S>,
    any_widget_focused: bool,

    rerender_again: bool
}

impl<S: 'static> App<S> {

    pub fn run(self) {

        let event_loop = EventLoop::new();

        let window_builder = WindowBuilder::new()
            .with_maximized(true)
            .with_title(self.title.clone());

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder.build(&event_loop, template, |mut configs| configs.next().unwrap()).unwrap();

        let window = window.unwrap();

        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
        let mut gl_ctx = Some(unsafe {
            gl_display.create_context(&gl_config, &context_attributes).unwrap()
        });

        let w = window.inner_size().width;
        let h = window.inner_size().height;

        let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
            window.raw_window_handle(),
            NonZeroU32::new(w).unwrap(),
            NonZeroU32::new(h).unwrap(),
        );

        let surface = unsafe {
            gl_config.display().create_window_surface(&gl_config, &attrs).unwrap()
        };

        let gl_ctx = gl_ctx.take().unwrap().make_current(&surface).unwrap();

        let renderer = unsafe { femtovg::renderer::OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _) }
            .expect("Could not start renderer.");

        let mut canvas = femtovg::Canvas::new(renderer).expect("Could not create femtovg canvas.");
        canvas.set_size(w, h, window.scale_factor() as f32);

        let text_font = canvas.add_font_mem(include_bytes!("../../../res/Roboto-Regular.ttf")).unwrap();

        let mut runtime = Runtime {
            window,
            canvas,
            surface,
            gl_ctx,
            input: Input::new(),

            text_font, 

            ui: self.ui,
            state: self.init_state,
            widget_state: WidgetState::new(),
            any_widget_focused: false,

            rerender_again: false
        };

        event_loop.run(move |event, _target, control_flow| {
            runtime.handle_event(event, control_flow);
        });

    }

}
