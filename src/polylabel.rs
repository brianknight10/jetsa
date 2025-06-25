use std::cmp::Ordering;
use std::collections::BinaryHeap;
use geo::Coord;

#[derive(Debug, Clone)]
pub struct PolyLabel {
    pub centroid: Coord<f64>,
    pub radius: f64,
}

#[derive(Debug, Clone)]
struct Cell {
    x: f64,
    y: f64,
    h: f64,
    d: f64,
    max: f64,
}

impl Cell {
    fn new(x: f64, y: f64, h: f64, polygon: &[Coord<f64>]) -> Self {
        let d = point_to_polygon_dist(x, y, polygon);
        let max = d + h * std::f64::consts::SQRT_2;
        
        Cell { x, y, h, d, max }
    }
}

// Implement ordering for priority queue (max heap)
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.max == other.max
    }
}

impl Eq for Cell {}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order for max heap
        self.max.partial_cmp(&other.max)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn get_poly_label(polygon: &[Coord<f64>], precision: f64, debug: bool) -> PolyLabel {
    // Find bounding box
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    
    for (i, p) in polygon.iter().enumerate() {
        if i == 0 || p.x < min_x { min_x = p.x; }
        if i == 0 || p.y < min_y { min_y = p.y; }
        if i == 0 || p.x > max_x { max_x = p.x; }
        if i == 0 || p.y > max_y { max_y = p.y; }
    }
    
    let width = max_x - min_x;
    let height = max_y - min_y;
    let cell_size = width.min(height);
    let h = cell_size / 2.0;
    
    // Handle degenerate case
    if cell_size == 0.0 {
        return PolyLabel {
            centroid: Coord { x: min_x, y: min_y },
            radius: 0.0,
        };
    }
    
    // Create initial grid
    let mut cell_queue = BinaryHeap::new();
    
    let mut x = min_x;
    while x < max_x {
        let mut y = min_y;
        while y < max_y {
            let cell = Cell::new(x + h, y + h, h, polygon);
            cell_queue.push(cell);
            y += cell_size;
        }
        x += cell_size;
    }
    
    // Get centroid cell
    let mut best_cell = get_centroid_cell(polygon);
    
    // Check bounding box center
    let bbox_cell = Cell::new(
        min_x + width / 2.0,
        min_y + height / 2.0,
        0.0,
        polygon
    );
    
    if bbox_cell.d > best_cell.d {
        best_cell = bbox_cell;
    }
    
    let mut num_probes = cell_queue.len();
    
    while let Some(cell) = cell_queue.pop() {
        // Update best cell if we found a better one
        if cell.d > best_cell.d {
            best_cell = cell.clone();
            if debug {
                println!("found best {} after {} probes", 
                    (1e4 * cell.d).round() / 1e4, num_probes);
            }
        }
        
        // Do not drill down further if there's no chance of a better solution
        if cell.max - best_cell.d <= precision {
            continue;
        }
        
        // Split the cell into four cells
        let h = cell.h / 2.0;
        
        cell_queue.push(Cell::new(cell.x - h, cell.y - h, h, polygon));
        cell_queue.push(Cell::new(cell.x + h, cell.y - h, h, polygon));
        cell_queue.push(Cell::new(cell.x - h, cell.y + h, h, polygon));
        cell_queue.push(Cell::new(cell.x + h, cell.y + h, h, polygon));
        
        num_probes += 4;
    }
    
    if debug {
        println!("num probes: {}", num_probes);
        println!("best distance: {}", best_cell.d);
    }
    
    PolyLabel {
        centroid: Coord { x: best_cell.x, y: best_cell.y },
        radius: best_cell.d,
    }
}

fn get_centroid_cell(polygon: &[Coord<f64>]) -> Cell {
    let mut area = 0.0;
    let mut x = 0.0;
    let mut y = 0.0;
    
    let len = polygon.len();
    let mut j = len - 1;
    
    for i in 0..len {
        let a = &polygon[i];
        let b = &polygon[j];
        let f = a.x * b.y - b.x * a.y;
        x += (a.x + b.x) * f;
        y += (a.y + b.y) * f;
        area += f * 3.0;
        j = i;
    }
    
    if area == 0.0 {
        Cell::new(polygon[0].x, polygon[0].y, 0.0, polygon)
    } else {
        Cell::new(x / area, y / area, 0.0, polygon)
    }
}

fn point_to_polygon_dist(x: f64, y: f64, polygon: &[Coord<f64>]) -> f64 {
    let mut inside = false;
    let mut min_dist_sq = f64::INFINITY;
    
    let len = polygon.len();
    let mut j = len - 1;
    
    for i in 0..len {
        let a = &polygon[i];
        let b = &polygon[j];
        
        if (a.y > y) != (b.y > y) && 
           (x < (b.x - a.x) * (y - a.y) / (b.y - a.y) + a.x) {
            inside = !inside;
        }
        
        min_dist_sq = min_dist_sq.min(get_seg_dist_sq(x, y, a, b));
        j = i;
    }
    
    (if inside { 1.0 } else { -1.0 }) * min_dist_sq.sqrt()
}

fn get_seg_dist_sq(px: f64, py: f64, a: &Coord<f64>, b: &Coord<f64>) -> f64 {
    let mut x = a.x;
    let mut y = a.y;
    let dx = b.x - x;
    let dy = b.y - y;
    
    if dx != 0.0 || dy != 0.0 {
        let t = ((px - x) * dx + (py - y) * dy) / (dx * dx + dy * dy);
        
        if t > 1.0 {
            x = b.x;
            y = b.y;
        } else if t > 0.0 {
            x += dx * t;
            y += dy * t;
        }
    }
    
    let dx = px - x;
    let dy = py - y;
    
    dx * dx + dy * dy
}
