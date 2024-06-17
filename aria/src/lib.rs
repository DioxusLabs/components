mod button;
pub use button::*;

#[derive(Clone, PartialEq)]
pub struct Icon {
    pub src: String,
    pub height: u32,
    pub width: u32,
}
