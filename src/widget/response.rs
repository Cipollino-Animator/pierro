
use crate::Pos;

#[derive(Clone, Copy)]
pub struct EdgedInput {
    down: bool,
    prev_down: bool 
}

impl EdgedInput {

    pub fn new() -> Self {
        Self {
            down: false,
            prev_down: false
        }
    }

    pub(crate) fn update(&mut self) {
        self.prev_down = self.down;
    }

    pub(crate) fn set(&mut self, down: bool) {
        self.down = down;
    }

    pub fn down(&self) -> bool {
        self.down
    }
    
    pub fn pressed(&self) -> bool {
        self.down && !self.prev_down
    }

    pub fn released(&self) -> bool {
        !self.down && self.prev_down
    }

}

pub struct Response {
    pub(crate) hover_pos: Option<Pos>,
    pub(crate) left_mouse_button: EdgedInput, 
    pub(crate) right_mouse_button: EdgedInput, 

    pub(crate) global_hover_pos: Option<Pos>,
    pub(crate) global_left_mouse_button: EdgedInput, 
    pub(crate) global_right_mouse_button: EdgedInput 
}

impl Response {

    pub(super) fn new() -> Self {
        Self {
            hover_pos: None,
            left_mouse_button: EdgedInput::new(), 
            right_mouse_button: EdgedInput::new(),

            global_hover_pos: None,
            global_left_mouse_button: EdgedInput::new(), 
            global_right_mouse_button: EdgedInput::new()
        }
    }

    pub fn hovered(&self) -> bool {
        self.hover_pos().is_some()
    }

    pub fn hover_pos(&self) -> Option<Pos> {
        self.hover_pos
    }

    pub fn mouse_clicked(&self) -> bool {
        self.left_mouse_button.pressed()
    }
    
    pub fn mouse_down(&self) -> bool {
        self.left_mouse_button.down()
    }

    pub fn clicked_elsewhere(&self) -> bool {
        (self.global_left_mouse_button.pressed() || self.global_right_mouse_button.pressed()) && !self.left_mouse_button.pressed()
    }

    pub fn global_hover_pos(&self) -> Option<Pos> {
        self.global_hover_pos
    }

    pub fn global_mouse_released(&self) -> bool {
        self.global_left_mouse_button.released() 
    }

}
