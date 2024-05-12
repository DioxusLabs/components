use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    Auto,
}

impl Default for Size {
    fn default() -> Self {
        Self::Auto
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Small => write!(f, "s-sm"),
            Self::Medium => write!(f, "s-md"),
            Self::Large => write!(f, "s-lg"),
            Self::Auto => write!(f, "s-auto"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, f32),
}

impl Color {
    pub fn hex<T: ToString>(value: T) -> Self {
        Self::Hex(value.to_string())
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::Rgba(r, g, b, a)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hex(hex) => write!(f, "#{}", hex),
            Self::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
            Self::Rgba(r, g, b, a) => write!(f, "rgba({},{},{},{})", r, g, b, a),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum FontFamily {
    Arial,
}

impl Display for FontFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arial => write!(f, "font-family:Arial,Helvetica,sans-serif;"),
        }
    }
}

impl Default for FontFamily {
    fn default() -> Self {
        Self::Arial
    }
}