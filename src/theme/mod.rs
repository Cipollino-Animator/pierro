
use crate::Color;

pub struct Theme {

    pub bg_dark: Color,
    pub bg_light: Color,
    pub bg_window: Color,
    pub stroke: Color,

    pub text: Color,

    pub button: Color,
    pub text_field: Color,

    pub hovered_darkness: f32,
    pub pressed_darkness: f32,

    pub font_size: f32,
    pub item_spacing: f32,
    pub rounding: f32
}

impl Theme {

    pub fn dark() -> Self {
        Self {
            bg_dark: Color::from_hex(0x2E2D31FF),
            bg_light: Color::from_hex(0x363738FF),
            bg_window: Color::from_hex(0x404143FF),
            stroke: Color::from_hex(0x1D1D1DFF),

            text: Color::from_hex(0xDCDEE0FF),

            button: Color::from_hex(0x56585AFF),
            text_field: Color::from_hex(0x242328FF),

            hovered_darkness: 0.2,
            pressed_darkness: 0.4,
            
            font_size: 13.0,
            item_spacing: 10.0,
            rounding: 7.0
        }
    }

}
