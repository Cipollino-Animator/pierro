
use crate::{state::WidgetState, vec2, Vec2, WidgetNode};
use super::{LayoutContext, LayoutResult, Widget};

pub struct Center<S> {
    inner: WidgetNode<S>
}

impl<S: 'static> Center<S> {

    pub fn new(inner: WidgetNode<S>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            inner
        })
    }

}

impl<S> Widget<S> for Center<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let inner_node = self.inner.layout(max_size, ctx, state);
        let inner_size = inner_node.size();
        let center = vec2(max_size.x / 2.0, max_size.y / 2.0); 
        let inner_min = center - inner_size / 2.0;

        let mut layout = LayoutResult::new(max_size);
        layout.add_child(inner_min, inner_node);
        layout
    }

}
