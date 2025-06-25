use crate::aixm::{self, AirspaceVolume, PolygonPatch};

use anyhow::Result;
use geo::{coord, Coord, LineString, Polygon};

#[derive(Debug)]
pub struct Poly {
    pub altitude: String,
    pub area: Polygon,
}

pub fn from_aixm_str(s: &str) -> Result<Vec<Poly>> {
    let aixm = aixm::from_aixm_str(s)?;
    let mut polys: Vec<Poly> = Vec::new();

    let mut iter = aixm.has_member.into_iter();
    while let Some(member) = iter.next() {
        let vol: AirspaceVolume = member
                                    .airspace
                                    .time_slice
                                    .airspace_time_slice
                                    .geometry_component
                                    .airspace_geometry_component
                                    .the_airspace_volume
                                    .airspace_volume;

        let poly = build_polygon(vol.horizontal_projection.surface.patches.polygon_patch);

        polys.push(Poly {
            altitude: vol.minimum_limit,
            area: poly,
        });
    }

    Ok(polys)
}

fn build_polygon(p: PolygonPatch) -> Polygon {
    let exterior_list = split_pos_list(&p.exterior.linear_ring.pos_list);
    let mut interiors: Vec<LineString> = Vec::new();

    let boundary = build_line_string(&exterior_list);

    // Handle multiple interior rings if they exist
    if let Some(interior_rings) = p.interior {
        for interior_ring in interior_rings {
            let interior_list = split_pos_list(&interior_ring.linear_ring.pos_list);
            interiors.push(build_line_string(&interior_list));
        }
    }

    Polygon::new(boundary, interiors)
}

fn build_line_string(l: &Vec<String>) -> LineString {
    let pos = l.clone();
    let mut coords: Vec<Coord> = Vec::new();

    let mut iter = pos.chunks(2);
    while let Some(c) = iter.next() {
        coords.push(coord! { x: c[0].parse().unwrap(), y: c[1].parse().unwrap() });
    }

    LineString::new(coords)
}

fn split_pos_list(s: &str) -> Vec<String> {
    s.split_whitespace().map(|c| c.to_string()).collect::<Vec<_>>()
}
