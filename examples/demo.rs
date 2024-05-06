
use std::marker::PhantomData;
use pierro::{color, painter::{Painter, RectBuilder}, widget::{button::Button, center::Center, column::Column, menu_bar::MenuBar, slider::Slider, text::Text, LayoutContext, LayoutResult, Widget}, Color, Rect, Response, Vec2, WidgetNode, WidgetState};

struct Companion<S> {
    color: Color,
    _marker: PhantomData<S> 
}

impl<S> Widget<S> for Companion<S> {

    type State = ();

    fn layout(&self, max_size: Vec2, _ctx: &mut LayoutContext, _state: &mut WidgetState<S>) -> LayoutResult<S> {
        LayoutResult::new(Vec2::splat(200.0).min(max_size))
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {
        painter.rect(RectBuilder::new(rect).fill(self.color));
    }

}

pub fn main() {

    pierro::app::App::new(color(1.0, 0.5, 1.0, 1.0), |state| {
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
                Center::new(
                    Column::new(vec![
                        Button::new(WidgetNode::new(Companion {
                            color: *state,
                            _marker: PhantomData
                        })).on_click(|state: &mut Color| {
                            state.r -= 0.1;
                        }),
                        Button::new(
                            Text::new("Hello World!")
                        ).on_click(|state: &mut Color| {
                            state.r += 0.1;
                        }),
                        Slider::new(state.r, 0.0..=1.0)
                            .on_set(|state: &mut Color, val| {
                                state.r = val;
                            }).on_finish(|_state| {
                                println!("HELLO!");
                            })
                            .build()
                    ])
                )
            ).build()
    }).run();

}
