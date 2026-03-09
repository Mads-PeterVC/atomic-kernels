use bevy::color::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum ScalarColorMap {
    Viridis,
    Inferno,
    Plasma,
    Sampled(Vec<[f32; 4]>),
}

impl ScalarColorMap {
    pub fn color(&self, value: f32) -> Color {
        match self {
            Self::Viridis => sample_stops(value, &viridis_stops()),
            Self::Inferno => sample_stops(value, &inferno_stops()),
            Self::Plasma => sample_stops(value, &plasma_stops()),
            Self::Sampled(colors) => sample_colors(value, colors),
        }
    }
}

fn sample_stops(value: f32, stops: &[(f32, [f32; 4])]) -> Color {
    let clamped = value.clamp(0.0, 1.0);
    for window in stops.windows(2) {
        let (start_t, start) = window[0];
        let (end_t, end) = window[1];
        if clamped <= end_t {
            let local = (clamped - start_t) / (end_t - start_t);
            return lerp_color(start, end, local);
        }
    }

    let [r, g, b, a] = stops.last().map(|(_, color)| *color).unwrap_or([1.0; 4]);
    Color::srgba(r, g, b, a)
}

fn sample_colors(value: f32, colors: &[[f32; 4]]) -> Color {
    match colors.len() {
        0 => Color::WHITE,
        1 => {
            let [r, g, b, a] = colors[0];
            Color::srgba(r, g, b, a)
        }
        len => {
            let position = value.clamp(0.0, 1.0) * (len - 1) as f32;
            let lower = position.floor() as usize;
            let upper = position.ceil() as usize;
            let local = position - lower as f32;
            lerp_color(colors[lower], colors[upper], local)
        }
    }
}

fn lerp_color(start: [f32; 4], end: [f32; 4], t: f32) -> Color {
    Color::srgba(
        (1.0 - t) * start[0] + t * end[0],
        (1.0 - t) * start[1] + t * end[1],
        (1.0 - t) * start[2] + t * end[2],
        (1.0 - t) * start[3] + t * end[3],
    )
}

fn viridis_stops() -> [(f32, [f32; 4]); 6] {
    [
        (0.0, [68.0 / 255.0, 1.0 / 255.0, 84.0 / 255.0, 1.0]),
        (0.2, [64.0 / 255.0, 67.0 / 255.0, 135.0 / 255.0, 1.0]),
        (0.4, [41.0 / 255.0, 120.0 / 255.0, 142.0 / 255.0, 1.0]),
        (0.6, [34.0 / 255.0, 167.0 / 255.0, 132.0 / 255.0, 1.0]),
        (0.8, [121.0 / 255.0, 209.0 / 255.0, 81.0 / 255.0, 1.0]),
        (1.0, [253.0 / 255.0, 231.0 / 255.0, 37.0 / 255.0, 1.0]),
    ]
}

fn inferno_stops() -> [(f32, [f32; 4]); 6] {
    [
        (0.0, [0.0 / 255.0, 0.0 / 255.0, 4.0 / 255.0, 1.0]),
        (0.2, [42.0 / 255.0, 11.0 / 255.0, 84.0 / 255.0, 1.0]),
        (0.4, [101.0 / 255.0, 21.0 / 255.0, 110.0 / 255.0, 1.0]),
        (0.6, [159.0 / 255.0, 42.0 / 255.0, 99.0 / 255.0, 1.0]),
        (0.8, [225.0 / 255.0, 100.0 / 255.0, 40.0 / 255.0, 1.0]),
        (1.0, [252.0 / 255.0, 1.0, 164.0 / 255.0, 1.0]),
    ]
}

fn plasma_stops() -> [(f32, [f32; 4]); 6] {
    [
        (0.0, [13.0 / 255.0, 8.0 / 255.0, 135.0 / 255.0, 1.0]),
        (0.2, [84.0 / 255.0, 3.0 / 255.0, 160.0 / 255.0, 1.0]),
        (0.4, [139.0 / 255.0, 10.0 / 255.0, 165.0 / 255.0, 1.0]),
        (0.6, [190.0 / 255.0, 56.0 / 255.0, 132.0 / 255.0, 1.0]),
        (0.8, [240.0 / 255.0, 128.0 / 255.0, 77.0 / 255.0, 1.0]),
        (1.0, [240.0 / 255.0, 249.0 / 255.0, 33.0 / 255.0, 1.0]),
    ]
}
