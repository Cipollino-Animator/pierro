
use std::{cell::RefCell, ops::Range};

use femtovg::{renderer::OpenGl, Canvas, FontId, Paint, Path};

use crate::{theme::Theme, vec2, Color, Pos, Rect, Vec2};

pub struct TextShaper<'a> {
    canvas: RefCell<&'a mut Canvas<OpenGl>>,
    text_paint: RefCell<Paint>,
    scl: f32
}

impl<'a> TextShaper<'a> {

    pub(crate) fn new(canvas: &'a mut Canvas<OpenGl>, fonts: &[FontId], scl: f32) -> Self {
        let mut text_paint = Paint::color(femtovg::Color::white());
        text_paint.set_font(fonts);
        Self {
            canvas: RefCell::new(canvas),
            text_paint: RefCell::new(text_paint),
            scl
        }
    }

    pub fn break_text(&self, font_size: f32, text: &str, max_width: f32) -> Vec<Range<usize>> {
        let mut text_paint = self.text_paint.borrow_mut();
        let text_paint = &mut *text_paint;
        let mut canvas = self.canvas.borrow_mut();
        let canvas = &mut *canvas;

        text_paint.set_font_size(font_size * self.scl);
        canvas.break_text_vec(max_width * self.scl, text, &text_paint).unwrap()
    }

    pub fn measure_text(&self, font_size: f32, text: &str) -> Vec2 {
        let mut text_paint = self.text_paint.borrow_mut();
        let text_paint = &mut *text_paint;
        let mut canvas = self.canvas.borrow_mut();
        let canvas = &mut *canvas;

        text_paint.set_font_size(font_size * self.scl);
        let metrics = canvas.measure_text(0.0, 0.0, text, &text_paint).unwrap();
        vec2(metrics.width() / self.scl, metrics.height() / self.scl)
    }

}

pub struct Painter<'a> {
    canvas: &'a mut Canvas<OpenGl>,
    pub theme: &'a Theme,
    text_paint: Paint,
    scl: f32,
    clip_rects: Vec<Rect>
}

fn to_color(color: Color) -> femtovg::Color {
    femtovg::Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a
    }
}

pub struct RectBuilder {
    rect: Rect,
    fill: Option<Color>,
    stroke: Option<Color>,
    rounding: f32
}

impl RectBuilder {

    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            fill: None,
            stroke: None,
            rounding: 0.0
        }
    }

    pub fn fill(mut self, fill: Color) -> Self {
        self.fill = Some(fill);
        self
    }

    pub fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = Some(stroke);
        self
    }

    pub fn rounding(mut self, rounding: f32) -> Self {
        self.rounding = rounding;
        self
    }

}

impl<'a> Painter<'a> {

    pub(crate) fn new(canvas: &'a mut Canvas<OpenGl>, theme: &'a Theme, fonts: &[FontId], scl: f32) -> Self {
        let mut text_paint = Paint::color(femtovg::Color::white());
        text_paint.set_font(fonts);
        Self {
            canvas,
            theme,
            text_paint,
            scl,
            clip_rects: Vec::new()
        }
    }

    pub fn rect(&mut self, rect: RectBuilder) {
        let mut path = Path::new();
        path.rounded_rect(rect.rect.left() * self.scl, rect.rect.top() * self.scl, rect.rect.width() * self.scl, rect.rect.height() * self.scl, rect.rounding * self.scl);
        if let Some(fill) = rect.fill {
            self.canvas.fill_path(&path, &Paint::color(to_color(fill)));
        }
        if let Some(stroke) = rect.stroke {
            self.canvas.stroke_path(&path, &Paint::color(to_color(stroke)).with_line_width(self.scl));
        }
    }

    pub fn line(&mut self, a: Pos, b: Pos, color: Color) {
        let mut path = Path::new();
        path.move_to(a.x * self.scl, a.y * self.scl);
        path.line_to(b.x * self.scl, b.y * self.scl);
        self.canvas.stroke_path(&path, &Paint::color(to_color(color)).with_line_width(self.scl));
    }

    pub fn text(&mut self, text: &str, pos: Pos, color: Color, font_size: f32) {
        self.text_paint.set_color(to_color(color));
        self.text_paint.set_font_size(font_size * self.scl);
        let _ = self.canvas.fill_text(pos.x * self.scl, pos.y * self.scl, text, &self.text_paint);
    } 

    fn set_clip_rect(&mut self, rect: Rect) {
        self.canvas.scissor(rect.left() * self.scl, rect.top() * self.scl, rect.width() * self.scl, rect.height() * self.scl);
    }

    pub fn push_clip_rect(&mut self, rect: Rect) {
        self.clip_rects.push(rect);
        self.set_clip_rect(rect);
    }

    pub fn pop_clip_rect(&mut self) {
        self.clip_rects.pop();
        let rect = *self.clip_rects.last().expect("popped too many times.");
        self.set_clip_rect(rect);
    }

}
