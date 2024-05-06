
use crate::{painter::{Painter, RectBuilder}, Rect, Response, Vec2, Widget, WidgetNode, WidgetState};

use super::{margin::Margin, LayoutContext, LayoutResult};

pub struct Dropdown<S> {
    contents: WidgetNode<S>
}

impl<S: 'static> Dropdown<S> {

    pub fn new(contents: WidgetNode<S>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            contents: Margin::new_with_margin(contents, 10.0)
        })
    }

}

impl<S> Widget<S> for Dropdown<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let inner_layout = self.contents.layout(max_size, ctx, state);
        let mut layout = LayoutResult::new(inner_layout.size());
        layout.add_child(Vec2::ZERO, inner_layout);
        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {
        painter.rect(RectBuilder::new(rect)
            .fill(painter.theme.bg_light)
            .stroke(painter.theme.stroke)
            .rounding(painter.theme.rounding));
    }

}
