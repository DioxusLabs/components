use std::fmt::Display;
use std::fmt::Write as _;

pub trait AsCss {
    fn as_css(&self) -> String;
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    Auto,
}

impl Size {
    pub fn as_class(&self) -> &str {
        match self {
            Size::Small => "s-sm",
            Size::Medium => "s-md",
            Size::Large => "s-lg",
            Size::Auto => "s-auto",
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Auto
    }
}

// We have a separate color enum so that we can enforce
// usage of type-checking functions.
#[derive(Clone, PartialEq)]
enum ColorType {
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
}

#[derive(Clone, PartialEq)]
pub struct Color {
    color: ColorType,
}

impl Color {
    pub fn hex<T: ToString>(value: T) -> Self {
        Self {
            color: ColorType::Hex(value.to_string()),
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            color: ColorType::Rgb(r, g, b),
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            color: ColorType::Rgba(r, g, b, a),
        }
    }
}

impl AsCss for Color {
    fn as_css(&self) -> String {
        self.to_string()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.color {
            ColorType::Hex(hex) => write!(f, "#{}", hex),
            ColorType::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
            ColorType::Rgba(r, g, b, a) => write!(f, "rgba({},{},{},{})", r, g, b, a),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FontFamily {
    Arial,
}

impl AsCss for FontFamily {
    fn as_css(&self) -> String {
        let mut css = String::new();

        match self {
            Self::Arial => write!(css, "font-family:Arial,Helvetica,sans-serif;").ok(),
        };

        css
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::Arial
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn as_class(&self) -> String {
        match self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
        }
    }

    pub fn as_flex_direction(&self) -> &str {
        match self {
            Self::Horizontal => "row",
            Self::Vertical => "column",
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Horizontal
    }
}
