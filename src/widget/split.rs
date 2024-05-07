
use crate::{painter::{cursor::Cursor, Painter}, Axis, Rect, Response, Vec2, Widget, WidgetNode, WidgetState};

use super::{LayoutContext, LayoutResult};

const LINE_SENSOR_SIZE: f32 = 10.0;

pub struct Split<S> {
    axis: Axis,
    contents: Vec<WidgetNode<S>>
}

pub struct SplitState {
    sizes: Vec<f32>,
    focused_split_idx: usize
}

impl Default for SplitState {
    fn default() -> Self {
        Self {
            sizes: Vec::new(),
            focused_split_idx: 0
        }
    }
}

impl<S: 'static> Split<S> {

    pub fn horizontal(contents: Vec<WidgetNode<S>>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            axis: Axis::X,
            contents
        })
    }
    
    pub fn vertical(contents: Vec<WidgetNode<S>>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            axis: Axis::Y,
            contents
        })
    }

}

impl<S> Widget<S> for Split<S> {
    type State = SplitState;

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, widget_state: &mut WidgetState<S>) -> LayoutResult<S> {
        let mut layout = LayoutResult::new(max_size); 
        let state = Self::get(widget_state);

        while state.sizes.len() > self.contents.len() {
            state.sizes.pop();
        }
        let mut size_total = state.sizes.iter().fold(0.0, |a, b| a + b); 
        let avg_size = if size_total < 0.001 {
            1.0
        } else {
            size_total / (state.sizes.len() as f32) 
        };
        while state.sizes.len() < self.contents.len() {
            state.sizes.push(avg_size);
            size_total += avg_size;
        }
        for size in state.sizes.iter_mut() {
            *size /= size_total;
        }

        let mut curr_offset = Vec2::ZERO;
        for (size_frac, widget) in state.sizes.iter().zip(self.contents.iter()).map(|(size, widget)| (*size, widget)).collect::<Vec<(f32, &WidgetNode<S>)>>() {
            let size = max_size.axis(self.axis) * size_frac;
            let child_max_size = self.axis.unit() * size + self.axis.other().unit() * max_size.axis(self.axis.other());
            layout.add_child(curr_offset, widget.layout(child_max_size, ctx, widget_state));
            curr_offset += self.axis.unit() * size;
        }

        let state = Self::get(widget_state);
        let mut curr_offset = Vec2::ZERO;
        for size in state.sizes[..state.sizes.len() - 1].iter() {
            curr_offset += self.axis.unit() * *size * max_size.axis(self.axis);
            layout.add_sensor(curr_offset - self.axis.unit() * LINE_SENSOR_SIZE / 2.0, self.axis.unit() * LINE_SENSOR_SIZE + self.axis.other().unit() * max_size.axis(self.axis.other()));
        }

        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, widget_state: &mut WidgetState<S>) {
        let state = Self::get(widget_state); 

        let mut curr_pos = rect.min();
        for size in state.sizes[..state.sizes.len() - 1].iter() {
            curr_pos += self.axis.unit() * *size * rect.dimension(self.axis);
            painter.line(curr_pos, curr_pos.with_axis(self.axis.other(), rect.max().axis(self.axis.other())), painter.theme.stroke);
        }

        if !widget_state.focused() {
            let state = Self::get(widget_state); 
            if resp.hovered() {
                painter.cursor = match self.axis {
                    Axis::X => Cursor::ColResize,
                    Axis::Y => Cursor::RowResize,
                };
            }
            if resp.mouse_clicked() {
                if let Some(idx) = resp.sensor_idx() {
                    state.focused_split_idx = idx;
                    widget_state.request_focus(); 
                }
            }
        } else {
            let state = Self::get(widget_state); 
            if state.focused_split_idx >= state.sizes.len() - 1 {
                widget_state.unfocus();
            } else {
                if let Some(hover_pos) = resp.global_hover_pos() {
                    let total_size = state.sizes[state.focused_split_idx] + state.sizes[state.focused_split_idx + 1];
                    let delta = hover_pos.axis(self.axis) / rect.dimension(self.axis) - state.sizes[state.focused_split_idx];
                    let delta = delta.min(state.sizes[state.focused_split_idx + 1] - 0.05 * total_size);
                    let delta = delta.max(-state.sizes[state.focused_split_idx] + 0.05 * total_size);
                    state.sizes[state.focused_split_idx] += delta;
                    state.sizes[state.focused_split_idx + 1] -= delta;
                }
            }
            if resp.global_mouse_released() {
                widget_state.unfocus();
            }
        }

    }

}
