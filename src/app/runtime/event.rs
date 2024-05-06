
use glutin::surface::GlSurface;
use winit::{dpi::PhysicalPosition, event::{Event, WindowEvent}, event_loop::ControlFlow};

use crate::{pos, vec2};

use super::Runtime;

impl<S: 'static> Runtime<S> {

    fn handle_window_event(&mut self, event: WindowEvent, control_flow: &mut ControlFlow) {
        let scl = self.window.scale_factor() as f32;
        match event {
            WindowEvent::Resized(physical_size) => {
                self.surface.resize(&self.gl_ctx, physical_size.width.try_into().unwrap(), physical_size.height.try_into().unwrap());
                self.rerender_again = true;
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.input.mouse_pos = Some(pos(position.x as f32 / scl, position.y as f32 / scl));
                self.rerender_again = true;
            },
            WindowEvent::CursorLeft { .. } => {
                self.input.mouse_pos = None;
                self.rerender_again = true;
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let down = match state {
                    winit::event::ElementState::Pressed => true,
                    winit::event::ElementState::Released => false,
                };
                match button {
                    winit::event::MouseButton::Left => self.input.left_mouse_button.set(down),
                    winit::event::MouseButton::Right => self.input.right_mouse_button.set(down),
                    _ => {}
                };
                self.rerender_again = true;
            },
            WindowEvent::MouseWheel { delta, .. } => {
                let line_scale = scl * 20.0;
                self.input.scroll = match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => vec2(x * line_scale, y * line_scale),
                    winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition {x, y}) => vec2(x as f32 / scl, y as f32 / scl),
                };
            },
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit
            },
            _ => {}
        }
    }

    pub(super) fn handle_event(&mut self, event: Event<'_, ()>, control_flow: &mut ControlFlow) {
        match event {
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                self.render();
            },
            Event::WindowEvent { window_id: _, event } => {
                self.handle_window_event(event, control_flow); 
            }
            _ => {}
        }

        if self.rerender_again {
            self.window.request_redraw();
        }
    }

}
