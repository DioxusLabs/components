use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
    Auto,
}

impl Size {
    pub fn as_css_class(&self) -> &str {
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

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_css_class())
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    pub fn as_class(&self) -> String {
        self.to_string()
    }
    
    pub fn as_flex_direction(&self) -> &str {
        match self {
            Self::Horizontal => "row",
            Self::Vertical => "column",
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

