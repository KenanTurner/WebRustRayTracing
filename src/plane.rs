use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3};
use crate::{Ray, Hit, Intersect, Material, VectorFormat, MaterialFormat};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "PlaneFormat", into = "PlaneFormat")]
pub struct Plane {
	pub origin: Point3<f64>,
	pub normal: Vector3<f64>,
	pub material: Material,
}

impl Intersect for Plane {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		let distance = cgmath::dot(self.normal, self.origin - ray.origin) / cgmath::dot(self.normal, ray.direction);
		let position = ray.at(distance);
		let normal = self.normal;
		if distance < 0.0 { return None };
		Some(Hit {
			distance,
			position,
			normal,
			material: self.material.clone(),
		})
	}
}

#[derive(Serialize, Deserialize)]
pub struct PlaneFormat {
	pub origin: VectorFormat,
	pub normal: VectorFormat,
	pub material: MaterialFormat,
}

impl From<PlaneFormat> for Plane {
    fn from(v: PlaneFormat) -> Plane { 
        Plane {
            origin: v.origin.into(),
            normal: v.normal.into(),
            material: v.material.into(),
        }
    }
}

impl From<Plane> for PlaneFormat {
    fn from(v: Plane) -> PlaneFormat { 
        PlaneFormat {
            origin: v.origin.into(),
			normal: v.normal.into(),
			material: v.material.into(),
        }
    }
}