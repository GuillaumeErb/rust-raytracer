use core::ops;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub const BLACK: Color = Color {
    red: 0f64,
    green: 0f64,
    blue: 0f64,
};

pub const WHITE: Color = Color {
    red: 1f64,
    green: 1f64,
    blue: 1f64,
};

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            red: (self.red * scalar).min(1f64).max(0f64),
            green: (self.green * scalar).min(1f64).max(0f64),
            blue: (self.blue * scalar).min(1f64).max(0f64),
        }
    }
}

impl ops::Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, color: &Color) -> Color {
        color * self
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;

    fn mul(self, other: &Color) -> Color {
        Color {
            red: (self.red * other.red).min(1f64).max(0f64),
            green: (self.green * other.green).min(1f64).max(0f64),
            blue: (self.blue * other.blue).min(1f64).max(0f64),
        }
    }
}

impl ops::Add<&Color> for &Color {
    type Output = Color;

    fn add(self, other: &Color) -> Color {
        Color {
            red: (self.red + other.red).min(1f64).max(0f64),
            green: (self.green + other.green).min(1f64).max(0f64),
            blue: (self.blue + other.blue).min(1f64).max(0f64),
        }
    }
}
