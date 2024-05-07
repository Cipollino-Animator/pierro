use pierro::{widget::{column::Column, scroll_area::ScrollArea, split::Split, text::Text}, WidgetNode};


struct State {
    left: Vec<&'static str>,
    right: Vec<&'static str>
}

fn str_list(strings: &Vec<&'static str>) -> WidgetNode<State> {
    ScrollArea::vertical(
        Column::new(
            strings.iter().map(|str| Text::new(*str)).collect()
        )
    )
}

pub fn main() {
    pierro::app::App::new(State {
        left: vec!["Hello", "World"],
        right: vec!["Goodbye", "Test", "Blarmagedoid"]
    }, |state| {
        Split::vertical(vec![
            str_list(&state.left),
            str_list(&state.right)
        ])
    }).run();
}