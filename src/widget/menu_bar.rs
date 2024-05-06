
use std::{cell::RefCell, marker::PhantomData};

use crate::{painter::{Painter, RectBuilder}, pos, state::WidgetState, vec2, Color, Rect, Response, Vec2, Widget, WidgetNode};

use super::{dropdown::Dropdown, LayoutContext, LayoutResult};

struct MenuBarItem<S> {
    label: String,
    label_size: RefCell<Option<Vec2>>,
    dropdown: WidgetNode<S>, 
    _marker: PhantomData<S> 
}

impl<S: 'static> MenuBarItem<S> {

    fn new(label: String, contents: WidgetNode<S>) -> WidgetNode<S> {
        WidgetNode::new(Self {
            label: label.into(),
            label_size: RefCell::new(None),
            dropdown: Dropdown::new(contents),
            _marker: PhantomData
        }).sense_click(true)
    }
    
}

impl<S: 'static> Widget<S> for MenuBarItem<S> {

    type State = bool;

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let label_size = ctx.text_shaper.measure_text(ctx.theme.font_size, &self.label);
        *self.label_size.borrow_mut() = Some(label_size);
        let mut layout = LayoutResult::new(vec2(label_size.x + 25.0, max_size.y));

        if *Self::get(state) {
            layout.add_popover(vec2(0.0, max_size.y), self.dropdown.layout_popover(ctx, state));
        }

        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>) {
        let color = painter.theme.bg_window.lerp(Color::BLACK, if *Self::get(state) {
            painter.theme.pressed_darkness
        } else if resp.hovered() {
            painter.theme.hovered_darkness
        } else {
            0.0
        });

        if resp.mouse_clicked() {
            *Self::get(state) = true;
        }
        if resp.clicked_elsewhere() {
            *Self::get(state) = false;
        }

        painter.rect(RectBuilder::new(Rect::new(rect.min() + Vec2::splat(0.5), rect.max() - Vec2::splat(0.5))).fill(color));

        let label_size = *self.label_size.borrow().as_ref().unwrap();
        let label_rect = Rect::center_size(rect.center(), label_size);
        painter.text(&self.label, label_rect.bottom_left(), painter.theme.text, painter.theme.font_size);
        painter.line(pos(rect.right(), rect.top()), pos(rect.right(), rect.bottom()), painter.theme.stroke);
    }

}

pub struct MenuBar<S> {
    main_content: Option<WidgetNode<S>>,
    items: Vec<WidgetNode<S>>
}

impl<S: 'static> MenuBar<S> {

    pub const HEIGHT: f32 = 25.0;

    pub fn new() -> Self {
        Self {
            main_content: None,
            items: Vec::new()
        }
    }

    pub fn main_content(mut self, content: WidgetNode<S>) -> Self {
        self.main_content = Some(content);
        self
    }

    pub fn item<T>(mut self, name: T, dropdown_contents: WidgetNode<S>) -> Self where T: Into<String> {
        self.items.push(MenuBarItem::new(name.into(), dropdown_contents));
        self
    }

    pub fn build(self) -> WidgetNode<S> {
        WidgetNode::new(self)
    }

}

impl<S: 'static> Widget<S> for MenuBar<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        let menu_bar_size = vec2(max_size.x, Self::HEIGHT).min(max_size);
        let mut layout = LayoutResult::new(menu_bar_size);

        let mut remaining_menu_bar_size = menu_bar_size;
        let mut item_x = 0.0;
        for item in &self.items {
            let item_layout = item.layout(remaining_menu_bar_size, ctx, state);
            if remaining_menu_bar_size.x < item_layout.size().x {
                break;
            } 
            let size = item_layout.size();
            layout.add_child(
                vec2(item_x, 0.0),
                item_layout
            );
            item_x += size.x;
            remaining_menu_bar_size.x -= size.x; 
        }

        if let Some(content) = &self.main_content {
            let max_content_size = vec2(max_size.x, max_size.y - menu_bar_size.y);
            let content_layout = content.layout(max_content_size, ctx, state);
            layout.size += content_layout.size();
            layout.add_child(vec2(0.0, menu_bar_size.y), content_layout);
        }

        layout
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {
        let height = Self::HEIGHT.min(rect.height());
        painter.rect(RectBuilder::new(Rect::min_size(rect.min(), vec2(rect.width(), height)))
            .fill(painter.theme.bg_window));
        painter.line(
            pos(rect.left(), rect.top() + height),
            pos(rect.right(), rect.top() + height),
            painter.theme.stroke);
    }

}
