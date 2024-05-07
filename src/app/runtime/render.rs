
use std::collections::VecDeque;

use glutin::surface::GlSurface;

use crate::{painter::{Painter, RectBuilder, TextShaper}, pos, state::WidgetState, theme::Theme, vec2, widget::{LayoutContext, Message}, LayoutNode, Pos, Rect, Vec2};

use super::Runtime;

fn calculate_node_rects<S>(node: &mut LayoutNode<S>, rect: Rect, window_size: Vec2) {
    node.rect = rect;
    for (offset, child_node) in &mut node.children {
        let rect = Rect::min_size(rect.min() + *offset, child_node.size());
        calculate_node_rects(child_node, rect, window_size);
    }
    for (offset, popover) in &mut node.popovers {
        let mut rect = Rect::min_size(rect.min() + *offset, popover.size());
        if rect.left() < 0.0 {
            rect = rect.shift(vec2(-rect.left(), 0.0));
        }
        if rect.right() > window_size.x {
            rect = rect.shift(vec2(window_size.x - rect.right(), 0.0));
        } 
        if rect.top() < 0.0 {
            rect = rect.shift(vec2(0.0, -rect.top()));
        }
        if rect.bottom() > window_size.y {
            rect = rect.shift(vec2(0.0, window_size.y - rect.bottom()));
        }
        calculate_node_rects(popover, rect, window_size);
    }
}

fn find_all_popovers<'a, 'ui, S>(node: &'a LayoutNode<'ui, S>, bfs: &mut VecDeque<(Vec<usize>, &'a LayoutNode<'ui, S>)>, id_path: &mut Vec<usize>) {
    for (_, popover) in &node.popovers {
        id_path.push(popover.local_id);
        bfs.push_back((id_path.clone(), popover));
        id_path.pop();
    } 
    for (_, child) in &node.children {
        id_path.push(child.local_id);
        find_all_popovers(child, bfs, id_path);
        id_path.pop();
    }
}

fn get_layer_roots<'a, 'ui, S>(node: &'a LayoutNode<'ui, S>) -> Vec<(Vec<usize>, &'a LayoutNode<'ui, S>)> {
    let mut layer_roots = Vec::new();

    let mut bfs = VecDeque::new();
    bfs.push_back((vec![node.local_id], node));
    while let Some((path, node)) = bfs.pop_front() {
        layer_roots.push((path.clone(), node));

        let mut sub_path = path;
        find_all_popovers(node, &mut bfs, &mut sub_path);
    }

    layer_roots
}

fn render_node<'ui, S>(painter: &mut Painter, node: &'ui LayoutNode<S>, messages: &mut Vec<Message<S>>, state: &mut WidgetState<S>) {
    let response = &*node.response.borrow();
    node.widget.widget.draw(painter, node.rect, response, state);

    if response.mouse_clicked() {
        if let Some(msg) = &node.widget.click_message {
            messages.push((*msg).clone());
        }
    }
    messages.append(&mut state.messages);

    for (_, child_node) in &node.children {
        render_node(painter, child_node, messages, state.get_child(child_node.local_id));
    }
}

fn clear_state_focus<S>(state: &mut WidgetState<S>) {
    state.focused = false;
    for (_, child) in state.child_state.iter_mut() {
        clear_state_focus(child);
    }
}

fn update_focus<S>(state: &mut WidgetState<S>) -> bool {
    let res = if state.requested_focus {
        state.focused = true;
        for (_, child) in state.child_state.iter_mut() {
            clear_state_focus(child);
        }
        true
    } else {
        let mut focus_id = None;
        for (id, child) in state.child_state.iter_mut() {
            if update_focus(child) {
                focus_id = Some(*id);
            }
        }
        if let Some(focus_id) = focus_id {
            for (id, child) in state.child_state.iter_mut() {
                if *id != focus_id {
                    clear_state_focus(child);
                } 
            }
        }
        focus_id.is_some()  
    };
    state.requested_focus = false;
    res
}

fn any_focused<S>(state: &WidgetState<S>) -> bool {
    if state.focused {
        return true;
    }
    for (_, child_state) in state.child_state.iter() {
        if any_focused(child_state) {
            return true;
        }
    }
    false
}

impl<S: 'static> Runtime<S> {

    // The most important function in the whole UI library!
    pub(super) fn render(&mut self) {

        let width = self.window.inner_size().width;
        let height = self.window.inner_size().height;
        let scl = self.window.scale_factor() as f32;
        self.canvas.set_size(width, height, scl);
        let logical_window_size = vec2(width as f32 / scl, height as f32 / scl);

        let theme = Theme::dark(); 

        let fonts = [self.text_font];

        // Build widget tree
        let root_widget = (self.ui)(&self.state);
        
        // Layout widget tree
        let mut text_shaper = TextShaper::new(&mut self.canvas, &fonts, scl);
        let mut layout_context = LayoutContext {
            text_shaper: &mut text_shaper,
            theme: &theme,
            curr_auto_id: 0,
            window_size: logical_window_size
        };

        let mut root_node = root_widget.layout(logical_window_size, &mut layout_context, &mut self.widget_state);
        let root_rect = Rect::min_size(pos(0.0, 0.0), root_node.size());
        calculate_node_rects(&mut root_node, root_rect, logical_window_size);

        // Get root nodes of each "layer"
        let layer_roots = get_layer_roots(&root_node);

        // Distribute input
        if self.any_widget_focused {
            self.input.distribute_input_to_focused(&root_node, &mut self.widget_state);
        } else {
            self.input.distribute_input(layer_roots.iter().rev().map(|(_path, node)| *node));
        }

        // Render widget tree
        let mut painter = Painter::new(&mut self.canvas, &theme, &fonts, scl);
        let fullscreen_rect = Rect::min_size(Pos::ZERO, vec2(width as f32 / scl, height as f32 / scl));
        painter.push_clip_rect(fullscreen_rect);
        painter.rect(RectBuilder::new(fullscreen_rect).fill(theme.bg_dark));
        let mut messages = Vec::new();
        for (path, root) in layer_roots {
            let mut state = &mut self.widget_state;
            for id in path {
                state = state.get_child(id);
            }
            render_node(&mut painter, &root, &mut messages, state);
        }
        self.window.set_cursor_icon(painter.cursor.to_winit_cursor());

        // Update focus
        update_focus(&mut self.widget_state);
        self.any_widget_focused = any_focused(&self.widget_state);

        // Apply messages
        for msg in messages {
            (msg.handler)(&mut self.state);
        }

        self.canvas.flush();
        self.surface.swap_buffers(&self.gl_ctx).expect("Could not swap buffers");
        self.input.update();

        if self.rerender_again {
            self.rerender_again = false;
            self.window.request_redraw();
        }

    }

} 
