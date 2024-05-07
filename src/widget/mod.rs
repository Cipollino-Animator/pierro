
pub mod response;
pub mod widget_node;
pub mod layout_node;

pub mod text;
pub mod button;
pub mod slider;

pub mod menu_bar;
pub mod dropdown;

pub mod margin;
pub mod center;
pub mod column;
pub mod split;
pub mod scroll_area;

use std::{any::{Any, TypeId}, rc::Rc};

use crate::{painter::{Painter, TextShaper}, state::WidgetState, theme::Theme, LayoutNode, Rect, Vec2, WidgetNode};
use self::response::Response;

pub(crate) struct Message<S> {
    pub(crate) handler: Rc<dyn Fn(&mut S)> 
}

impl<S> Message<S> {

    pub(crate) fn new<F>(handler: F) -> Self where F: Fn(&mut S) + 'static {
        Self {
            handler: Rc::new(handler)
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            handler: self.handler.clone()
        }
    }

}

pub struct LayoutContext<'a> {
    pub text_shaper: &'a TextShaper<'a>,
    pub theme: &'a Theme,

    pub(crate) curr_auto_id: usize,

    pub(crate) window_size: Vec2 
}

pub struct LayoutResult<'ui, S> {
    pub size: Vec2,
    children: Vec<(Vec2, LayoutNode<'ui, S>)>,
    popovers: Vec<(Vec2, LayoutNode<'ui, S>)>,
    sensors: Vec<(Vec2, Vec2)>
}

impl<'ui, S> LayoutResult<'ui, S> {

    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            children: Vec::new(),
            popovers: Vec::new(),
            sensors: Vec::new()
        }
    }

    pub fn add_child(&mut self, offset: Vec2, child: LayoutNode<'ui, S>) {
        self.children.push((offset, child));
    } 

    pub fn add_popover(&mut self, offset: Vec2, popover: LayoutNode<'ui, S>) {
        self.popovers.push((offset, popover));
    }
    
    pub fn add_sensor(&mut self, offset: Vec2, size: Vec2) {
        self.sensors.push((offset, size));
    }

}

pub trait Widget<S> {

    type State: Any + Default;

    fn get(state: &mut WidgetState<S>) -> &mut Self::State {
        if (&*state.state).type_id() != TypeId::of::<Self::State>() {
            state.state = Box::new(Self::State::default());
        }
        state.state.downcast_mut().unwrap()
    }

    /*
        Calculate the size of the widget and layout of any child widgets given a maximum size.
    */
    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S>;

    fn draw(&self, _painter: &mut Painter, _rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {

    } 

    fn post_draw(&self, _painter: &mut Painter, _rect: Rect, _resp: &Response, _state: &mut WidgetState<S>) {

    } 

}

pub(crate) trait WidgetDyn<S> {

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S>;
    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>);
    fn post_draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>);

}

impl<S, W> WidgetDyn<S> for W where W: Widget<S> {

    fn layout(&self, max_size: Vec2, ctx: &mut LayoutContext, state: &mut WidgetState<S>) -> LayoutResult<S> {
        self.layout(max_size, ctx, state)
    }

    fn draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>) {
        self.draw(painter, rect, resp, state)
    }

    fn post_draw(&self, painter: &mut Painter, rect: Rect, resp: &Response, state: &mut WidgetState<S>) {
        self.post_draw(painter, rect, resp, state);
    }

}
