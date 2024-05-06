
use crate::{widget::response::EdgedInput, LayoutNode, Pos, Response, WidgetNode, WidgetState};

// Contains all the raw input to the app 
pub(super) struct Input {
    pub mouse_pos: Option<Pos>,
    pub left_mouse_button: EdgedInput,
    pub right_mouse_button: EdgedInput
}

impl Input {
    
    pub(super) fn new() -> Self {
        Self {
            mouse_pos: None,
            left_mouse_button: EdgedInput::new(),
            right_mouse_button: EdgedInput::new() 
        }
    }

    fn make_widget_input(&self) -> WidgetInput {
        WidgetInput {
            hover_pos: self.mouse_pos,
            left_mouse_button: self.left_mouse_button,
            right_mouse_button: self.right_mouse_button,
            global_hover_pos: self.mouse_pos,
            global_left_mouse_button: self.left_mouse_button,
            global_right_mouse_button: self.right_mouse_button 
        }
    }

    pub(super) fn distribute_input<'a, S, I>(&self, layer_roots: I) where I: Iterator<Item = &'a LayoutNode<'a, S>> + 'a, S: 'static {
        let mut widget_input = self.make_widget_input(); 
        for root in layer_roots {
            widget_input.distribute_to_node(root);
        }
    }

    pub(super) fn distribute_input_to_focused<S>(&self, root_node: &LayoutNode<S>, state: &mut WidgetState<S>) {
        let mut widget_input = self.make_widget_input(); 
        widget_input.distribute_to_focused_node(root_node, state);
    }

    pub(super) fn update(&mut self) {
        self.left_mouse_button.update();
        self.right_mouse_button.update();
    }

}

/*
    Contains the input that must be "distributed" to the widgets 

    Certain inputs, such as mouse clicks, should only affect one widget at a time.
    The input distribution system ensures that, for example, a click isn't registered
    on both a button widget and the contents(child widget) of the button. 
*/
struct WidgetInput {
    hover_pos: Option<Pos>,
    left_mouse_button: EdgedInput, 
    right_mouse_button: EdgedInput, 

    global_hover_pos: Option<Pos>,
    global_left_mouse_button: EdgedInput,
    global_right_mouse_button: EdgedInput
}

impl WidgetInput {

    fn distribute_global_input(&self, response: &mut Response) {
        response.global_hover_pos = self.global_hover_pos; 
        response.global_left_mouse_button = self.global_left_mouse_button;
        response.global_right_mouse_button = self.global_right_mouse_button;
    }

    fn distribute_active_input(&mut self, response: &mut Response, hover_pos: Pos) {
        response.hover_pos = Some(hover_pos);
        response.left_mouse_button = self.left_mouse_button; 
        response.right_mouse_button = self.right_mouse_button; 
        self.hover_pos = None;
    }

    fn distribute_to_node<S>(&mut self, node: &LayoutNode<S>) {
        for (_, popover) in &node.popovers {
            self.distribute_to_node(popover);
        }
        for (_, child) in &node.children {
            self.distribute_to_node(child);
        }

        let response = &mut *node.response.borrow_mut();
        self.distribute_global_input(response);

        if let Some(hover_pos) = self.hover_pos {
            if node.rect.contains(hover_pos) && node.widget.sense_click {
                self.distribute_active_input(response, hover_pos);
            }
        }
    }

    fn distribute_to_focused_node<S>(&mut self, node: &LayoutNode<S>, state: &mut WidgetState<S>) {
        for (_, popover) in &node.popovers {
            self.distribute_to_focused_node(popover, state.get_child(popover.local_id));
        }
        for (_, child) in &node.children {
            self.distribute_to_focused_node(child, state.get_child(child.local_id));
        }

        let response = &mut *node.response.borrow_mut();
        self.distribute_global_input(response);
        if state.focused {
            if let Some(hover_pos) = self.hover_pos {
                self.distribute_active_input(response, hover_pos);
            }
        }
    }

}
