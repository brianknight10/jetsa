use crate::map::Map;

use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};

pub fn write(map: Map) -> Result<()> {
    let output = File::create(output_name(&map.name))?;
    let mut writer = BufWriter::new(output);

    let mut polys = map.polys.iter();
    let mut features: Vec<Feature> = Vec::new();

    while let Some(poly) = polys.next() {
        let boundary: Geometry = poly.area.exterior().into();

        features.push(Feature {
            bbox: None,
            foreign_members: None,
            geometry: Some(boundary),
            id: None,
            properties: None,
        });

        match poly.area.interiors().first() {
            Some(interior ) => {
                let g: Geometry = interior.into();

                features.push(Feature {
                    bbox: None,
                    foreign_members: None,
                    geometry: Some(g),
                    id: None,
                    properties: None,
                });
            }
            None => {}
        }
    }

    let mut labels = map.digits.iter();
    while let Some(label) = labels.next() {
        for line in &label.lines {
            let coords: Vec<Vec<f64>> = line.iter().map(|c| vec![c.x, c.y]).collect();
            let g = Geometry::new(geojson::Value::LineString(coords));
            features.push(Feature {
                bbox: None,
                foreign_members: None,
                geometry: Some(g),
                id: None,
                properties: None,
            });
        }
    }

    let geojson = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    writer.write_all(geojson.to_string().as_bytes())?;
    
    Ok(())
}

fn output_name(map_name: &str) -> String {
    format!("{map_name}.geojson")
}