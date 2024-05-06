
use std::{any::Any, collections::HashMap};

use crate::widget::Message;

pub struct WidgetState<S> {
    pub(crate) state: Box<dyn Any>,
    pub(crate) child_state: HashMap<usize, WidgetState<S>>,
    pub(crate) focused: bool,
    pub(crate) requested_focus: bool,
    pub(crate) messages: Vec<Message<S>>
}

impl<S> WidgetState<S> {

    pub(crate) fn new() -> Self {
        Self {
            state: Box::new(()),
            child_state: HashMap::new(),
            focused: false,
            requested_focus: false,
            messages: Vec::new() 
        }
    }

    pub(crate) fn get_child(&mut self, local_id: usize) -> &mut Self {
        if !self.child_state.contains_key(&local_id) {
            self.child_state.insert(local_id, WidgetState::new());
        }
        self.child_state.get_mut(&local_id).unwrap()
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn request_focus(&mut self) {
        self.requested_focus = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn message<F>(&mut self, handler: F) where F: Fn(&mut S) + 'static {
        self.messages.push(Message::new(handler));
    }

}
