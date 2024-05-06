
use crate::{painter::{Painter, RectBuilder}, state::WidgetState, Color, Rect, Vec2, WidgetNode};
use super::{margin::Margin, LayoutContext, LayoutResult, Response, Widget};


pub struct Button<S> {
    inner_margin: WidgetNode<S>,
}

impl<S: 'static> Button<S> {
    
    pub fn new(inner: WidgetNode<S>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            inner_margin: Margin::new(inner),
        }).sense_click(true)
    }

}

impl<S> Widget<S> for Button<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let inner_node = self.inner_margin.layout(max_size, ctx, state);
        let inner_size = inner_node.size();
        let mut layout = LayoutResult::new(inner_size.min(max_size));
        layout.add_child(Vec2::ZERO, inner_node);
        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, _state: &mut WidgetState<S>) {

        let color = painter.theme.button.lerp(Color::BLACK, if resp.mouse_down() {
            painter.theme.pressed_darkness
        } else if resp.hovered() {
            painter.theme.hovered_darkness
        } else {
            0.0
        });

        painter.rect(RectBuilder::new(rect).fill(color).stroke(painter.theme.stroke).rounding(painter.theme.rounding));
    }

}
