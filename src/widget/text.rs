
use std::{marker::PhantomData, ops::Range};
use crate::{painter::Painter, vec2, Rect, Response, Vec2, Widget, WidgetNode, WidgetState};

use super::{LayoutContext, LayoutResult};

struct TextBreakLineData {
    range: Range<usize>,
    y: f32 
}

pub struct TextCache {
    break_data: Vec<TextBreakLineData> 
}

impl Default for TextCache {

    fn default() -> Self {
        Self {
            break_data: vec![], 
        }
    }

}

pub struct Text<S> {
    text: String,
    _marker: PhantomData<S>
}

impl<S: 'static> Text<S> {

    pub fn new<T: Into<String>>(text: T) -> WidgetNode<S> {
        WidgetNode::new(Self {
            text: text.into(),
            _marker: PhantomData
        })
    }

}

impl<S> Widget<S> for Text<S> {

    type State = TextCache;

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let mut break_data = Vec::new();
        let mut curr_y = 0.0;
        let mut width: f32 = 0.0;
        for line_range in ctx.text_shaper.break_text(ctx.theme.font_size, self.text.as_str(), max_size.x) {
            let size = ctx.text_shaper.measure_text(ctx.theme.font_size, &self.text[line_range.clone()]);
            curr_y += size.y;
            break_data.push(TextBreakLineData {
                range: line_range,
                y: curr_y
            });
            width = width.max(size.x);
        }

        Self::get(state).break_data = break_data;

        LayoutResult::new(vec2(width, curr_y))
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, _resp: &Response, state: &mut WidgetState<S>) {
        let break_data = &Self::get(state).break_data;
        for line in break_data {
            painter.text(&self.text[line.range.clone()], rect.min() + vec2(0.0, line.y), painter.theme.text, painter.theme.font_size)
        }
    }

}
