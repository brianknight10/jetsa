use std::f64::consts;

use geo::{coord, Coord};

const NUM_PER_DEGREE_LAT: f64 = 60.0;
const NUM_PER_DEGREE_LON: f64 = 51.0;
const WIDTH: f64 = 1920.0;
const HEIGHT: f64 = 1080.0;

pub fn geo_to_pixels(geo: &Coord, center: &Coord, mag_var: f64) -> Coord {
    let sec_cos_mag: f64 = (-mag_var * (consts::PI / 180.0)).cos();
    let sec_sin_mag: f64 = (-mag_var * (consts::PI / 180.0)).sin();

    let dx = (geo.x - center.x) * NUM_PER_DEGREE_LON;
    let dy = (center.y - geo.y) * NUM_PER_DEGREE_LAT;

    let dx1 = (dx * sec_cos_mag) - (dy * sec_sin_mag);
    let dy1 = (dx * sec_sin_mag) + (dy * sec_cos_mag);

    let x = dx1 + (WIDTH / 2.0);
    let y = dy1 + (HEIGHT / 2.0);

    coord! { x: x, y: y }
}

pub fn pixels_to_geo(pixel: &Coord, center: &Coord, mag_var: f64) -> Coord {
    let rev_sec_cos_mag: f64 = (mag_var * (consts::PI / 180.0)).cos();
    let rev_sec_sin_mag: f64 = (mag_var * (consts::PI / 180.0)).sin();

    let dx = pixel.x - (WIDTH / 2.0);
    let dy = pixel.y - (HEIGHT / 2.0);

    let dx1 = ((dx * rev_sec_cos_mag) - (dy * rev_sec_sin_mag)) / NUM_PER_DEGREE_LON;
    let dy1 = ((dx * rev_sec_sin_mag) + (dy * rev_sec_cos_mag)) / NUM_PER_DEGREE_LAT;

    let x = center.x + dx1;
    let y = center.y - dy1;

    coord! { x: x, y: y }
}