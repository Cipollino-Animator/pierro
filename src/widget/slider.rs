
use std::{ops::RangeInclusive, rc::Rc};

use crate::{painter::{Painter, RectBuilder}, pos, vec2, Color, Rect, Response, Vec2, Widget, WidgetNode, WidgetState};

use super::{LayoutContext, LayoutResult};

pub struct Slider<S, N> {
    value: N,
    range: RangeInclusive<N>,
    on_set: Option<Rc<dyn Fn(&mut S, N)>>,
    on_finish: Option<Rc<dyn Fn(&mut S)>>
}

impl<S: 'static, N: 'static + Into<f32> + From<f32> + Copy> Slider<S, N> {

    pub fn new(value: N, range: RangeInclusive<N>) -> Self {
        Self {
            value,
            range,
            on_set: None,
            on_finish: None
        }
    }

    pub fn on_set<F>(mut self, handler: F) -> Self where F: Fn(&mut S, N) + 'static {
        self.on_set = Some(Rc::new(handler));
        self
    }

    pub fn on_finish<F>(mut self, handler: F) -> Self where F: Fn(&mut S) + 'static {
        self.on_finish = Some(Rc::new(handler));
        self
    }

    pub fn build(self) -> WidgetNode<S> {
        WidgetNode::new(self).sense_click(true)
    }

}

impl<S, N> Widget<S> for Slider<S, N> where
    S: 'static,
    N: Into<f32> + From<f32> + Copy + 'static { 
    type State = ();

    fn layout(&self, max_size: Vec2, _ctx: &mut LayoutContext, _state: &mut WidgetState<S>) -> LayoutResult<S> {
        LayoutResult::new(max_size.min(vec2(f32::INFINITY, 15.0)))
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>) {

        let slider_bar_h = 7.5;
        let slider_head_r = rect.height() / 2.0;
        let slider_bar_rect = Rect::center_size(rect.center(), vec2(
            rect.size().x - 2.0 * slider_head_r + slider_bar_h,
            slider_bar_h
        ).min(rect.size()));

        painter.rect(RectBuilder::new(slider_bar_rect).fill(painter.theme.bg_light).rounding(slider_bar_h / 2.0));

        let t = (self.value.into() - (*self.range.start()).into()) / ((*self.range.end()).into() - (*self.range.start()).into());
        let head_x = rect.left() + slider_head_r + t * (rect.width() - 2.0 * slider_head_r);
        let head_rect = Rect::center_size(pos(head_x, rect.center().y), vec2(2.0 * slider_head_r, 2.0 * slider_head_r).min(rect.size()));
        let mouse_on_head = if let Some(hover_pos) = resp.hover_pos() {
            (hover_pos - head_rect.center()).length() <= slider_head_r
        } else {
            false
        };
        let head_color = painter.theme.button.lerp(Color::BLACK, if state.focused() { 
            painter.theme.pressed_darkness
        } else if mouse_on_head {
            painter.theme.hovered_darkness
        } else {
            0.0
        });

        if state.focused() {
            if let Some(hover_pos) = resp.global_hover_pos() {
                let hover_x = hover_pos.x;
                let hover_t = (hover_x - (rect.left() + slider_head_r)) / (rect.width() - 2.0 * slider_head_r);
                let hover_t = hover_t.clamp(0.0, 1.0);
                let new_val = (*self.range.start()).into() + hover_t * ((*self.range.end()).into() - (*self.range.start()).into()); 
                if let Some(on_set) = &self.on_set {
                    let on_set = on_set.clone();
                    state.message(move |state| {
                        on_set(state, new_val.into());
                    })
                }
            }

            if resp.global_mouse_released() {
                state.unfocus();
                if let Some(on_finish) = &self.on_finish {
                    let on_finish = on_finish.clone();
                    state.message(move |state| {
                        on_finish(state);
                    });
                }
            }
        }
        if mouse_on_head && resp.mouse_clicked() {
            state.request_focus();
        }

        painter.rect(RectBuilder::new(head_rect).rounding(slider_head_r).fill(head_color).stroke(painter.theme.stroke));
    }
} 
