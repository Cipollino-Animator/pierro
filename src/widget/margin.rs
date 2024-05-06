
use crate::{state::WidgetState, Vec2};
use super::{LayoutContext, LayoutResult, Widget, WidgetNode};


pub struct Margin<S> {
    inner: WidgetNode<S>,
    margin: f32
}

impl<S: 'static> Margin<S> {

    pub fn new(inner: WidgetNode<S>) -> WidgetNode<S> {
        Self::new_with_margin(inner, 10.0)
    }

    pub fn new_with_margin(inner: WidgetNode<S>, margin: f32) -> WidgetNode<S> {
        WidgetNode::new(Self {
            inner,
            margin 
        })
    }

}

impl<S> Widget<S> for Margin<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let inner_node = self.inner.layout(max_size - Vec2::splat(2.0 * self.margin), ctx, state);
        let inner_size = inner_node.size();
        let size = (inner_size + Vec2::splat(2.0 * self.margin)).min(max_size);

        let mut layout = LayoutResult::new(size);
        layout.add_child(Vec2::splat(self.margin), inner_node);
        layout
    }

}
