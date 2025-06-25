use crate::label::{self, Label};
use crate::poly::{self, Poly};

use anyhow::Result;

pub struct Map {
    pub digits: Vec<Label>,
    pub name: String,
    pub polys: Vec<Poly>,
}

impl Map {
    pub fn build(aixm: String, name: String, mag_var: f64) -> Result<Self> {
        let polys = poly::from_aixm_str(&aixm)?;
        let labels = label::create(&polys, mag_var)?;

        let map = Map { name, polys, digits: labels };

        Ok(map)
    }
}