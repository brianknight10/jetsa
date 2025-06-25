mod aixm;
mod digit;
mod label;
mod map;
mod poly;
mod polylabel;
mod writer;
mod util;

use map::Map;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;

pub fn convert(source: PathBuf, name: String, mag_var: f64) -> Result<()> {
    println!("[jetsa] Parsing source XML file...");
    let aixm = fs::read_to_string(source)?;

    println!("[jetsa] Writing GeoJSON file...");
    let map = Map::build(aixm, name, mag_var)?;
    writer::write(map)?;

    println!("[jetsa] Complete!");

    Ok(())
}