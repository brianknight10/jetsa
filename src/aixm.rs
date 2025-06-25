use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AixmBasicMessage {
    #[serde(rename = "hasMember")]
    pub has_member: Vec<HasMember>,
}

#[derive(Debug, Deserialize)]
pub struct Airspace {
    #[serde(rename = "timeSlice")]
    pub time_slice: TimeSlice,
}

#[derive(Debug, Deserialize)]
pub struct AirspaceGeometryComponent {
    #[serde(rename = "theAirspaceVolume")]
    pub the_airspace_volume: TheAirspaceVolume,
}

#[derive(Debug, Deserialize)]
pub struct AirspaceTimeSlice {
    #[serde(rename = "geometryComponent")]
    pub geometry_component: GeometryComponent,
}

#[derive(Debug, Deserialize)]
pub struct AirspaceVolume {
    #[serde(rename = "horizontalProjection")]
    pub horizontal_projection: HorizontalProjection,

    #[serde(rename = "minimumLimit")]
    pub minimum_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct GeometryComponent {
    #[serde(rename = "AirspaceGeometryComponent")]
    pub airspace_geometry_component: AirspaceGeometryComponent,
}

#[derive(Debug, Deserialize)]
pub struct HasMember {
    #[serde(rename = "Airspace")]
    pub airspace: Airspace,
}

#[derive(Debug, Deserialize)]
pub struct HorizontalProjection {
    #[serde(rename = "Surface")]
    pub surface: Surface,
}

#[derive(Debug, Deserialize)]
pub struct LinearRing {
    #[serde(rename = "posList")]
    pub pos_list: String,
}

#[derive(Debug, Deserialize)]
pub struct Patches {
    #[serde(rename = "PolygonPatch")]
    pub polygon_patch: PolygonPatch,
}

#[derive(Debug, Deserialize)]
pub struct PolygonPatch {
    pub exterior: PolygonPatchGeometry,

    pub interior: Option<Vec<PolygonPatchGeometry>>,
}

#[derive(Debug, Deserialize)]
pub struct PolygonPatchGeometry {
    #[serde(rename = "LinearRing")]
    pub linear_ring: LinearRing,
}

#[derive(Debug, Deserialize)]
pub struct Surface {
    pub patches: Patches,
}

#[derive(Debug, Deserialize)]
pub struct TheAirspaceVolume {
    #[serde(rename = "AirspaceVolume")]
    pub airspace_volume: AirspaceVolume,
}

#[derive(Debug, Deserialize)]
pub struct TimeSlice {
    #[serde(rename = "AirspaceTimeSlice")]
    pub airspace_time_slice: AirspaceTimeSlice,
}

pub fn from_aixm_str(s: &str) -> Result<AixmBasicMessage> {
    let aixm: AixmBasicMessage = quick_xml::de::from_str(s)?;

    Ok(aixm)
}