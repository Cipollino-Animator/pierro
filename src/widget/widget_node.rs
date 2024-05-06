
use crate::{state::WidgetState, LayoutNode, Vec2};

use super::{LayoutContext, LayoutResult, Message, Widget, WidgetDyn};

// A node in the widget tree
pub struct WidgetNode<S> {
    pub(crate) widget: Box<dyn WidgetDyn<S>>,
    pub(crate) local_id: Option<usize>,

    // Whether the widget captures mouse clicks and hovering
    pub(crate) sense_click: bool,
    pub(crate) click_message: Option<Message<S>>
}

impl<S> WidgetNode<S> {

    pub fn new<W: Widget<S> + 'static>(widget: W) -> Self {
        Self {
            widget: Box::new(widget),
            local_id: None,
            sense_click: false,
            click_message: None 
        }
    }

    pub fn id(mut self, id: usize) -> Self {
        self.local_id = Some(id);
        self
    }

    pub fn sense_click(mut self, sense_click: bool) -> Self {
        self.sense_click = sense_click;
        self 
    }

    pub fn on_click<F>(mut self, handler: F) -> Self where F: Fn(&mut S) + 'static {
        self.sense_click = true;
        self.click_message = Some(Message::new(handler));
        self
    }

    pub fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutNode<S> {
        let local_id = if let Some(id) = self.local_id {
            id
        } else {
            ctx.curr_auto_id += 1;
            ctx.curr_auto_id - 1
        };

        let mut child_ctx = LayoutContext {
            text_shaper: ctx.text_shaper,
            theme: ctx.theme,
            curr_auto_id: 0,
            window_size: ctx.window_size
        };

        let LayoutResult {size, children, popovers} = self.widget.layout(max_size, &mut child_ctx, state.get_child(local_id));
        LayoutNode::new(local_id, size, self, children, popovers)
    }

    pub fn layout_popover(&self, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutNode<S> {
        self.layout(ctx.window_size, ctx, state)
    }

}
