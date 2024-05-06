
use crate::WidgetNode;

mod runtime;

pub struct App<S> {
    title: String,
    ui: Box<dyn Fn(&S) -> WidgetNode<S>>,
    init_state: S
}

impl<S> App<S> {
    
    pub fn new<F>(init_state: S, ui: F) -> Self where F: Fn(&S) -> WidgetNode<S> + 'static {
        Self {
            title: "Pierro".to_owned(),
            ui: Box::new(ui),
            init_state 
        }
    }

    pub fn title<T>(mut self, title: T) -> Self where T: Into<String> {
        self.title = title.into();
        self
    }

}
