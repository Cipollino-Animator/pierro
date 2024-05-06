
use crate::{painter::{Painter, RectBuilder}, vec2, Axis, Rect, Response, Vec2, Widget, WidgetNode, WidgetState};

use super::{LayoutContext, LayoutResult};

const SCROLLBAR_SIZE: f32 = 10.0;

pub struct ScrollArea<S> {
    inner: WidgetNode<S>,
    scroll_h: bool,
    scroll_v: bool
}

impl<S: 'static> ScrollArea<S> {

    pub fn new(inner: WidgetNode<S>, scroll_h: bool, scroll_v: bool) -> WidgetNode<S> {
        WidgetNode::new(Self {
            inner,
            scroll_h,
            scroll_v,
        }).sense_click(true)
    }

    pub fn horizontal(inner: WidgetNode<S>) -> WidgetNode<S> {
        Self::new(inner, true, false)
    }

    pub fn vertical(inner: WidgetNode<S>) -> WidgetNode<S> {
        Self::new(inner, false, true)
    }

    pub fn both(inner: WidgetNode<S>) -> WidgetNode<S> {
        Self::new(inner, true, true)
    } 

    fn draw_scrollbar(&self, axis: Axis, widget_state: &mut WidgetState<S>, scroll_area: Rect, painter: &mut Painter, resp: &Response) {
        let focused = widget_state.focused();
        let state = Self::get(widget_state);
        let mut focus = None;
        let mut unfocus = false;

        let scroll_axis = match axis {
            Axis::X => self.scroll_h,
            Axis::Y => self.scroll_v,
        };
        if scroll_axis && state.inner_size.axis(axis) > scroll_area.dimension(axis) {
            let scrollbar_area_size = axis.unit() * scroll_area.dimension(axis) + axis.other().unit() * SCROLLBAR_SIZE;
            let scrollbar_area_min = scroll_area.min() + axis.other().unit() * scroll_area.dimension(axis.other());
            let scrollbar_scale = scrollbar_area_size.axis(axis) / state.inner_size.axis(axis);
            let scrollbar_length = scrollbar_area_size.axis(axis) * scrollbar_scale; 
            let scrollbar_pos = -state.scroll.axis(axis) * scrollbar_scale;
            let scrollbar_rect = Rect::min_size(scrollbar_area_min + axis.unit() * scrollbar_pos, axis.unit() * scrollbar_length + axis.other().unit() * SCROLLBAR_SIZE);

            let hovered = if let Some(hover_pos) = resp.hover_pos() {
                scrollbar_rect.contains(hover_pos) 
            } else {
                false
            };

            let color = painter.theme.button.darken(if focused && state.focused_axis == axis {
                painter.theme.pressed_darkness
            } else {
                if hovered {
                    painter.theme.hovered_darkness
                } else {
                    0.0
                }
            });

            painter.rect(RectBuilder::new(scrollbar_rect)
                .fill(color));

            if hovered && resp.mouse_clicked() {
                focus = Some(axis);
                state.scrollbar_mouse_offset = resp.global_hover_pos().unwrap().axis(axis) - scrollbar_pos; 
            }

            if focused && axis == state.focused_axis {
                if let Some(hover_pos) = resp.global_hover_pos() {
                    let new_scrollbar_pos = hover_pos.axis(axis) - state.scrollbar_mouse_offset;
                    let new_scroll = -new_scrollbar_pos / scrollbar_scale;
                    *state.scroll.axis_mut(axis) = new_scroll;
                }
            }
        }

        if focused && axis == state.focused_axis {
            if resp.global_mouse_released() {
                unfocus = true;
            }
        }

        if let Some(axis) = focus {
            state.focused_axis = axis;
            widget_state.request_focus(); 
        }
        if unfocus {
            widget_state.unfocus(); 
        }
    }

}

pub struct ScrollAreaState {
    scroll: Vec2,
    inner_size: Vec2,
    focused_axis: Axis,
    scrollbar_mouse_offset: f32 
}

impl Default for ScrollAreaState {

    fn default() -> Self {
        Self {
            scroll: Vec2::ZERO,
            inner_size: Vec2::ZERO,
            focused_axis: Axis::X,
            scrollbar_mouse_offset: 0.0 
        }
    }

}

impl<S: 'static> Widget<S> for ScrollArea<S> {

    type State = ScrollAreaState;

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let inner_layout = self.inner.layout(vec2(
            if self.scroll_h { f32::INFINITY } else { max_size.x - if self.scroll_v { SCROLLBAR_SIZE } else { 0.0 } },
            if self.scroll_v { f32::INFINITY } else { max_size.y - if self.scroll_h { SCROLLBAR_SIZE } else { 0.0 } }
        ), ctx, state);

        let state = Self::get(state);
        state.inner_size = inner_layout.size();
        let mut layout = LayoutResult::new(max_size);
        layout.add_child(state.scroll, inner_layout);
        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>) {
        let scroll_area_size = vec2(
            rect.width() - if self.scroll_v { SCROLLBAR_SIZE } else { 0.0 },
            rect.height() - if self.scroll_h { SCROLLBAR_SIZE } else { 0.0 }
        );
        let scroll_area = Rect::min_size(rect.min(), scroll_area_size);

        self.draw_scrollbar(Axis::X, state, scroll_area, painter, resp);
        self.draw_scrollbar(Axis::Y, state, scroll_area, painter, resp);
        let state = Self::get(state);
        
        painter.push_clip_rect(scroll_area);

        state.scroll += resp.scroll();
        if self.scroll_h {
            state.scroll.x = state.scroll.x.clamp((scroll_area_size.x - state.inner_size.x).min(0.0), 0.0);
        } else {
            state.scroll.x = 0.0;
        }
        if self.scroll_v {
            state.scroll.y = state.scroll.y.clamp((scroll_area_size.y - state.inner_size.y).min(0.0), 0.0);
        } else {
            state.scroll.y = 0.0;
        }
    }

    fn post_draw(&self, painter: &mut Painter, _rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {
        painter.pop_clip_rect(); 
    }

}
