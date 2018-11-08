#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_ansi_string(&self, body: &str) -> String {
        format!("\x1b[38;2;{};{};{}m{}", self.r, self.g, self.b, body)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color() {
        let color = Color::new(255, 255, 255);

        let ansi_string = color.to_ansi_string("Hello");

        assert_eq!(ansi_string, "\033[38;2;255;255;255mHello");
    }
}
