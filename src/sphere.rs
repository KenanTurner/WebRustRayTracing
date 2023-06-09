use serde::{Serialize, Deserialize};
use cgmath::{Point3};
use cgmath::InnerSpace;
use crate::{Ray, Hit, Intersect, Material, VectorFormat, MaterialFormat};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "SphereFormat", into = "SphereFormat")]
pub struct Sphere {
	pub origin: Point3<f64>,
	pub radius: f64,
	pub material: Material,
}

impl Intersect for Sphere {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		let pc = ray.origin - self.origin;
		let a = cgmath::dot(ray.direction, ray.direction);
		let b = 2.0 * cgmath::dot(ray.direction, pc);
		let c = cgmath::dot(pc, pc) - (self.radius * self.radius);
		let d = (b*b) - (4.0*a*c);
		if d < 0.0 { return None };
		let t_plus = (-b + d.sqrt()) / (2.0*a);
		let t_minus = (-b - d.sqrt()) / (2.0*a);
		let distance = t_plus.min(t_minus);
		if distance < 0.0 { return None };
		let position = ray.at(distance);
		let normal = (position - self.origin).normalize();
		Some(Hit {
			distance,
			position,
			normal,
			material: self.material.clone(),
		})
	}
}

#[derive(Serialize, Deserialize)]
pub struct SphereFormat {
	pub origin: VectorFormat,
	pub radius: f64,
	pub material: MaterialFormat,
}

impl From<SphereFormat> for Sphere {
    fn from(v: SphereFormat) -> Sphere { 
        Sphere {
            origin: v.origin.into(),
            radius: v.radius,
            material: v.material.into(),
        }
    }
}

impl From<Sphere> for SphereFormat {
    fn from(v: Sphere) -> SphereFormat { 
        SphereFormat {
            origin: v.origin.into(),
			radius: v.radius,
			material: v.material.into(),
        }
    }
}