use crossterm::style::Color;

use rand::Rng;

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();

    Color::Rgb {
        r: rng.gen_range(0..255),
        g: rng.gen_range(0..255),
        b: rng.gen_range(0..255),
    }
}
