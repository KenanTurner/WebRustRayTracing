use serde::{Serialize, Deserialize};
use cgmath::{Vector3, Point3};
use crate::{VectorFormat, RgbFormat};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "LightFormat", into = "LightFormat")]
pub struct Light {
	pub position: Point3<f64>,
	pub color: Vector3<f64>,
	pub intensity: f64,
	pub attenuation: Option<(f64, f64, f64)>,
}

#[derive(Serialize, Deserialize)]
pub struct LightFormat {
	pub position: VectorFormat,
	pub color: RgbFormat,
	pub intensity: f64,
	pub attenuation: Option<(f64, f64, f64)>,
}

impl From<LightFormat> for Light {
    fn from(v: LightFormat) -> Light { 
        Light {
            position: v.position.into(),
            color: v.color.into(),
            intensity: v.intensity,
			attenuation: v.attenuation,
        }
    }
}

impl From<Light> for LightFormat {
    fn from(v: Light) -> LightFormat { 
        LightFormat {
            position: v.position.into(),
            color: v.color.into(),
            intensity: v.intensity,
			attenuation: v.attenuation,
        }
    }
}