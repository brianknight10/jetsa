use crate::digit::Digit;
use crate::poly::Poly;
use crate::util;
use crate::polylabel::get_poly_label;

use anyhow::Result;
use geo::{coord, Centroid, Coord};

pub struct Label {
    pub lines: Vec<Vec<Coord>>,
    pub text: String,
}

pub fn create(polys: &[Poly], mag_var: f64) -> Result<Vec<Label>> {
    let mut labels: Vec<Label> = Vec::new();
    let center = polys.first().unwrap().area.centroid().unwrap().into();

    // Calculate average cell size
    let average_cell_size = calculate_average_cell_size(polys, &center, mag_var);
    // Scale factor based on average cell size
    let scale = average_cell_size.max(0.03).min(0.5);

    for poly in polys {
        // Convert polygon to pixel space first
        let polygon_pixels: Vec<Coord> = poly.area.exterior()
            .coords()
            .map(|coord| util::geo_to_pixels(coord, &center, mag_var))
            .collect();
        
        // Calculate polylabel in pixel space
        let poly_label = get_poly_label(&polygon_pixels, 1.0, false);
        let label_center = coord! { x: poly_label.centroid.x, y: poly_label.centroid.y };
        
        let mva = label_str(&poly.altitude);
        let mut digits: Vec<Vec<Coord>> = Vec::new();
        
        // Calculate label dimensions
        let char_count = mva.chars().count() as f64;
        let label_width = char_count * 20.0 * scale;
        
        // For rotated labels, calculate the starting position differently
        if mag_var != 0.0 {
            let mag_var_rad = mag_var * std::f64::consts::PI / 180.0;
            let cos_mag = mag_var_rad.cos();
            let sin_mag = mag_var_rad.sin();
            
            // Start from center and go back half the width
            let half_width = label_width / 2.0;
            let start_offset_x = -half_width * cos_mag;
            let start_offset_y = -half_width * sin_mag;
            
            let mut current_x = label_center.x + start_offset_x;
            let mut current_y = label_center.y + start_offset_y;
            
            for c in mva.chars() {
                let digit_pos_pixels = coord!{ x: current_x, y: current_y };
                let digit = Digit::new(c);
                
                digits.push(build_digit_lines(digit, digit_pos_pixels, scale, center, mag_var));
                
                // Move to next character position along the rotated baseline
                current_x += 20.0 * scale * cos_mag;
                current_y += 20.0 * scale * sin_mag;
            }
        } else {
            // Original non-rotated positioning
            let mut x = label_center.x - ((char_count - 1.0) * 20.0 * scale);
            let y = label_center.y;

            for c in mva.chars() {
                let digit_pos_pixels = coord!{ x: x, y: y };
                let digit = Digit::new(c);
                
                digits.push(build_digit_lines(digit, digit_pos_pixels, scale, center, mag_var));
                
                x += 20.0 * scale;
            }
        }

        let label = Label { lines: digits, text: mva.clone() };
        labels.push(label);
    }

    Ok(labels)   
}

fn calculate_average_cell_size(polys: &[Poly], center: &Coord, mag_var: f64) -> f64 {
    let mut aggregated_cell_size = 0.0;
    let mut total_tests = 0;
    
    for poly in polys {
        // Handle polygons with holes if needed
        let coords = if poly.area.interiors().is_empty() {
            // Just use exterior
            poly.area.exterior().coords().copied().collect::<Vec<_>>()
        } else {
            // Combine exterior and interior coordinates
            let mut combined = poly.area.exterior().coords().copied().collect::<Vec<_>>();
            for interior in poly.area.interiors() {
                combined.extend(interior.coords().copied());
            }
            combined
        };
        
        // Convert to pixel space for polylabel calculation
        let polygon_pixels: Vec<Coord> = coords.iter()
            .map(|coord| util::geo_to_pixels(coord, center, mag_var))
            .collect();
        
        let poly_label = get_poly_label(&polygon_pixels, 1.0, false);
        
        aggregated_cell_size += poly_label.radius / 100.0;
        total_tests += 1;
    }
    
    aggregated_cell_size / total_tests as f64
}

fn label_str(mva: &str) -> String {
    mva[..mva.len() - 2].to_string()
}

fn build_digit_lines(digit: Digit, pixel_pos: Coord, scale: f64, center: Coord, mag_var: f64) -> Vec<Coord> {
    let mut transformed_coords: Vec<Coord> = Vec::new();
    
    // Convert magnetic variation to radians
    let mag_var_rad = mag_var * std::f64::consts::PI / 180.0;
    let cos_mag = mag_var_rad.cos();
    let sin_mag = mag_var_rad.sin();

    for raw in digit.coords() {
        // Scale the digit coordinates
        let scaled_x = raw.x * scale;
        let scaled_y = -raw.y * scale;  // Y is inverted
        
        // Apply magnetic rotation around origin (0,0)
        let rotated_x = scaled_x * cos_mag - scaled_y * sin_mag;
        let rotated_y = scaled_x * sin_mag + scaled_y * cos_mag;
        
        // Translate to pixel position
        let x = pixel_pos.x + rotated_x;
        let y = pixel_pos.y + rotated_y;
        
        let pixel_coord = coord!{ x: x, y: y };
        
        // Convert from pixel space back to geographic
        let transformed = util::pixels_to_geo(&pixel_coord, &center, mag_var);
        transformed_coords.push(transformed);
    }

    transformed_coords
}
