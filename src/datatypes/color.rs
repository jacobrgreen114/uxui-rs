use glm::Vec4;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// create a new color from red, green, blue
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// create a new color from red, green, blue, and alpha
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// create a new greyscale color
    pub const fn grey(gray: f32) -> Self {
        Self::rgb(gray, gray, gray)
    }

    /// create a new color from red, green, blue, and alpha bytes
    pub fn bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// create a new color from hue, saturation, value, and alpha
    ///
    /// # Params
    /// - hue - the hue in degrees [0-360)
    /// - saturation - the saturation [0-1]
    /// - value - the value [0-1]
    /// - alpha - the alpha [0-1]
    pub fn hsva(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
        assert!(0.0 <= hue && hue < 360.0, "Hue must be in range [0, 360)");
        assert!(
            0.0 <= saturation && saturation <= 1.0,
            "Saturation must be in range [0, 1]"
        );
        assert!(
            0.0 <= value && value <= 1.0,
            "Value must be in range [0, 1]"
        );
        assert!(
            0.0 <= alpha && alpha <= 1.0,
            "Alpha must be in range [0, 1]"
        );

        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        Self::hcxma(hue, c, x, m, alpha)
    }

    /// create a new color from hue, saturation, and value
    ///
    /// # Params
    /// - hue - the hue in degrees [0-360)
    /// - saturation - the saturation [0-1]
    /// - value - the value [0-1]
    pub fn hsv(hue: f32, saturation: f32, value: f32) -> Self {
        Self::hsva(hue, saturation, value, 1.0)
    }

    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        assert!(0.0 <= hue && hue < 360.0, "Hue must be in range [0, 360)");
        assert!(
            0.0 <= saturation && saturation <= 1.0,
            "Saturation must be in range [0, 1]"
        );
        assert!(
            0.0 <= lightness && lightness <= 1.0,
            "Lightness must be in range [0, 1]"
        );
        assert!(
            0.0 <= alpha && alpha <= 1.0,
            "Alpha must be in range [0, 1]"
        );

        let c = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = lightness - c / 2.0;

        Self::hcxma(hue, c, x, m, alpha)
    }

    pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::hsla(hue, saturation, lightness, 1.0)
    }

    fn hcxma(hue: f32, chroma: f32, x: f32, m: f32, alpha: f32) -> Self {
        let (r_, g_, b_) = match hue {
            h if h < 60.0 => (chroma, x, 0.0),
            h if h < 120.0 => (x, chroma, 0.0),
            h if h < 180.0 => (0.0, chroma, x),
            h if h < 240.0 => (0.0, x, chroma),
            h if h < 300.0 => (x, 0.0, chroma),
            h if h < 360.0 => (chroma, 0.0, x),
            _ => panic!(),
        };

        Self::rgba(r_ + m, g_ + m, b_ + m, alpha)
    }
}

/// Const Colors
impl Color {
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);

    pub const BLACK: Self = Self::grey(0.0);
    pub const GRAY25: Self = Self::grey(0.25);
    pub const GRAY50: Self = Self::grey(0.5);
    pub const GRAY75: Self = Self::grey(0.75);
    pub const WHITE: Self = Self::grey(1.0);

    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const BURNT_ORANGE: Self = Self::rgb(1.0, 0.10, 0.0);
    pub const ORANGE: Self = Self::rgb(1.0, 0.25, 0.0);
    pub const AMBER: Self = Self::rgb(1.0, 0.5, 0.0);
    pub const GOLD: Self = Self::rgb(1.0, 0.75, 0.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const LIME: Self = Self::rgb(0.5, 1.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const MINT: Self = Self::rgb(0.0, 1.0, 0.25);
    pub const SEAGREEN_DARK: Self = Self::rgb(0.0, 1.0, 0.5);
    pub const SEAGREEN: Self = Self::rgb(0.0, 1.0, 0.75);
    pub const CYAN: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const SKYBLUE: Self = Self::rgb(0.0, 0.5, 1.0);
    pub const LIGHT_BLUE: Self = Self::rgb(0.0, 0.25, 1.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const INDIGO: Self = Self::rgb(0.10, 0.0, 1.0);
    pub const VIOLET: Self = Self::rgb(0.25, 0.0, 1.0);
    pub const PURPLE: Self = Self::rgb(0.50, 0.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);
    pub const PINK: Self = Self::rgb(1.0, 0.0, 0.5);
    pub const ROSE: Self = Self::rgb(1.0, 0.0, 0.25);
    pub const SALMON: Self = Self::rgb(1.0, 0.0, 0.10);
}

impl Into<Vec4> for Color {
    fn into(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}

impl From<Vec4> for Color {
    fn from(value: Vec4) -> Self {
        Self {
            r: value.x,
            g: value.y,
            b: value.z,
            a: value.w,
        }
    }
}

impl From<wgpu::Color> for Color {
    fn from(value: wgpu::Color) -> Self {
        Self {
            r: value.r as f32,
            g: value.g as f32,
            b: value.b as f32,
            a: value.a as f32,
        }
    }
}
