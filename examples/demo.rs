
use std::marker::PhantomData;
use pierro::{color, painter::{Painter, RectBuilder}, vec2, widget::{button::Button, center::Center, column::Column, menu_bar::MenuBar, scroll_area::ScrollArea, slider::Slider, text::Text, LayoutContext, LayoutResult, Widget}, Color, Rect, Response, Vec2, WidgetNode, WidgetState};

struct Companion<S> {
    color: Color,
    _marker: PhantomData<S> 
}

impl<S> Widget<S> for Companion<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, _ctx: &mut LayoutContext, _state: &mut WidgetState<S>) -> LayoutResult<S> {
        LayoutResult::new(vec2(5000.0, 200.0).min(max_size))
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {
        painter.rect(RectBuilder::new(rect).fill(self.color));
        for x in 0..((rect.width() / 100.0) as i32) {
            painter.rect(RectBuilder::new(Rect::min_size(rect.min() + vec2(x as f32 * 100.0, 0.0), vec2(10.0, rect.height())))
                .fill(Color::BLACK));
        }
    }

}

pub fn main() {

    pierro::app::App::new(color(1.0, 0.5, 1.0, 1.0), |state| {

        let mut scroll_list = Vec::new();
        for i in 0..100 {
            scroll_list.push(Text::new(format!("Hello World! {}", i + 1)));
            scroll_list.push(WidgetNode::new(Companion {
                color: color(i as f32 / 99.0, 0.0, 0.5, 1.0),
                _marker: PhantomData,
            }));
        }

        MenuBar::new()
            .item("File", Column::new(vec![
                Button::new(
                    Text::new("Make it red!")
                ).on_click(|state: &mut Color| {
                    *state = color(0.9, 0.2, 0.2, 1.0);
                }),
                Button::new(
                    Text::new("Make it green!")
                ).on_click(|state: &mut Color| {
                    *state = color(0.2, 0.9, 0.4, 1.0);
                }),
                Button::new(
                    Text::new("Make it blue!")
                ).on_click(|state: &mut Color| {
                    *state = color(0.2, 0.4, 0.9, 1.0);
                })
            ]))
            .item("Edit", Column::new(vec![
                Text::new("day of")
            ]))
            .item("View", Column::new(vec![
                Text::new("Sun")
            ]))
            .main_content(
                // Center::new(
                //     Column::new(vec![
                //         Button::new(WidgetNode::new(Companion {
                //             color: *state,
                //             _marker: PhantomData
                //         })).on_click(|state: &mut Color| {
                //             state.r -= 0.1;
                //         }),
                //         Button::new(
                //             Text::new("Hello World!")
                //         ).on_click(|state: &mut Color| {
                //             state.r += 0.1;
                //         }),
                //         Slider::new(state.r, 0.0..=1.0)
                //             .on_set(|state: &mut Color, val| {
                //                 state.r = val;
                //             }).on_finish(|_state| {
                //                 println!("HELLO!");
                //             })
                //             .build()
                //     ])
                // )
                ScrollArea::both(Column::new(scroll_list))
            ).build()
    }).run();

}
