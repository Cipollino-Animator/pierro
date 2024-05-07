use winit::window::CursorIcon;


pub enum Cursor {
    Default,
    RowResize,
    ColResize
}

impl Cursor {

    pub(crate) fn to_winit_cursor(&self) -> CursorIcon {
        match self {
            Cursor::Default => CursorIcon::Default,
            Cursor::RowResize => CursorIcon::RowResize,
            Cursor::ColResize => CursorIcon::ColResize,
        }
    }

}