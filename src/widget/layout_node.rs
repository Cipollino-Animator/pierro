
use std::cell::RefCell;

use crate::{Pos, Rect, Response, Vec2, WidgetNode};

// A node in the widget tree with layout information
pub struct LayoutNode<'ui, S> {

    // Stage 1: Layout
    pub(crate) local_id: usize,
    pub(crate) rect: Rect,
    pub(crate) widget: &'ui WidgetNode<S>,
    pub(crate) children: Vec<(Vec2, LayoutNode<'ui, S>)>,
    pub(crate) popovers: Vec<(Vec2, LayoutNode<'ui, S>)>,

    // Stage 2: Input Handling
    pub(crate) response: RefCell<Response>
    
}

impl<'ui, S> LayoutNode<'ui, S> {

    pub(crate) fn new(local_id: usize, size: Vec2, widget: &'ui WidgetNode<S>, children: Vec<(Vec2, LayoutNode<'ui, S>)>, popovers: Vec<(Vec2, LayoutNode<'ui, S>)>) -> Self {
        Self {
            local_id,
            rect: Rect::min_size(Pos::ZERO, size),
            widget,
            children,
            popovers,
            response: RefCell::new(Response::new())
        }
    }

    pub fn size(&self) -> Vec2 {
        self.rect.size()
    }

}
