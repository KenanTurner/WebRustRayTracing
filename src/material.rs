use serde::{Serialize, Deserialize};
use cgmath::Vector3;
use crate::RgbFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "MaterialFormat", into = "MaterialFormat")]
pub enum Material {
	DebugPosition,
	DebugNormals,
	DebugShadows,
	Emissive {
		color: Vector3<f64>,
	},
	Mirror,
	BlinnPhong {
		ambient: Vector3<f64>,
		diffuse: Vector3<f64>,
		specular: Vector3<f64>,
		intensity: f64,
	}
}

#[derive(Serialize, Deserialize)]
pub enum MaterialFormat {
	DebugPosition,
	DebugNormals,
	DebugShadows,
	Emissive {
		color: RgbFormat,
	},
	Mirror,
	BlinnPhong {
		ambient: RgbFormat,
		diffuse: RgbFormat,
		specular: RgbFormat,
		intensity: f64,
	}
}

impl From<MaterialFormat> for Material {
    fn from(v: MaterialFormat) -> Material {
		match v {
			MaterialFormat::DebugPosition => Material::DebugPosition,
			MaterialFormat::DebugNormals => Material::DebugNormals,
			MaterialFormat::DebugShadows => Material::DebugShadows,
			MaterialFormat::Emissive { color } => Material::Emissive { color: color.into() },
			MaterialFormat::Mirror => Material::Mirror,
			MaterialFormat::BlinnPhong { ambient, diffuse, specular, intensity } => Material::BlinnPhong { 
				ambient: ambient.into(),
				diffuse: diffuse.into(),
				specular: specular.into(),
				intensity
			},
		}
    }
}

impl From<Material> for MaterialFormat {
    fn from(v: Material) -> MaterialFormat { 
        match v {
			Material::DebugPosition => MaterialFormat::DebugPosition,
			Material::DebugNormals => MaterialFormat::DebugNormals,
			Material::DebugShadows => MaterialFormat::DebugShadows,
			Material::Emissive { color } => MaterialFormat::Emissive { color: color.into() },
			Material::Mirror => MaterialFormat::Mirror,
			Material::BlinnPhong { ambient, diffuse, specular, intensity } => MaterialFormat::BlinnPhong { 
				ambient: ambient.into(),
				diffuse: diffuse.into(),
				specular: specular.into(),
				intensity
			},
		}
    }
}