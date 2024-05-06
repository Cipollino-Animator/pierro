
use crate::{state::WidgetState, vec2, Vec2, Widget, WidgetNode};

use super::{LayoutContext, LayoutResult};

pub struct Column<S> {
    contents: Vec<WidgetNode<S>>
}

impl<S: 'static> Column<S> {

    pub fn new(contents: Vec<WidgetNode<S>>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            contents
        })
    }

}

impl<S> Widget<S> for Column<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let mut layout = LayoutResult::new(Vec2::ZERO);
        for item in &self.contents {
            let item_layout = item.layout(vec2(max_size.x, max_size.y - layout.size.y - ctx.theme.item_spacing), ctx, state);
            let size = item_layout.size();
            layout.add_child(vec2(0.0, layout.size.y), item_layout);

            layout.size.y += size.y + ctx.theme.item_spacing;
            layout.size.x = layout.size.x.max(size.x);
            if layout.size.y >= max_size.y - 0.05 {
                break;
            }
        }

        layout.size.y -= ctx.theme.item_spacing;
        
        layout
    }

}
