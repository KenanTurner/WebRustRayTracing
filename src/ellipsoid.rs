use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3, Rad, Matrix4};
use cgmath::{Transform, One, MetricSpace, EuclideanSpace};
use crate::{Ray, Hit, Intersect, Material, VectorFormat, MaterialFormat};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "EllipsoidFormat", into = "EllipsoidFormat")]
pub struct Ellipsoid {
	pub transform: Matrix4<f64>,
	pub material: Material,
}

impl Ellipsoid {
	fn new(origin: Point3<f64>, scale: Vector3<f64>, rot_axis: Vector3<f64>, rot_angle: Rad<f64>, material: Material) -> Ellipsoid {
		Ellipsoid {
			transform: Matrix4::one()
				* Matrix4::from_translation(origin.to_vec())
				* Matrix4::from_axis_angle(rot_axis, rot_angle)
				* Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z),
			material,
		}
	}
}

impl Intersect for Ellipsoid {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		let transform_inv = self.transform.inverse_transform().unwrap();
		
		let trans_ray_origin = transform_inv.transform_point(ray.origin);
		let trans_ray_origin_vec = Vector3::new(trans_ray_origin.x, trans_ray_origin.y, trans_ray_origin.z); // bruh
		let trans_ray_direction = transform_inv.transform_vector(ray.direction);
		
		let a = cgmath::dot(trans_ray_direction, trans_ray_direction);
		let b = 2.0 * cgmath::dot(trans_ray_direction, trans_ray_origin_vec);
		let c = cgmath::dot(trans_ray_origin_vec, trans_ray_origin_vec) - 1.0;
		let d = (b*b) - (4.0*a*c);
		if d < 0.0 { return None };
		let t_plus = (-b + d.sqrt()) / (2.0*a);
		let t_minus = (-b - d.sqrt()) / (2.0*a);
		let distance = t_plus.min(t_minus);
		if distance < 0.0 { return None };
		let position = trans_ray_origin + (trans_ray_direction * distance);
		let normal = trans_ray_origin_vec + (trans_ray_direction * distance);
		
		let mut hit = Hit {
			distance: ray.origin.distance(self.transform.transform_point(position)),
			position: self.transform.transform_point(position),
			normal: transform_inv.transform_vector(normal),
			material: self.material.clone(),
		};
		if cgmath::dot(ray.direction, hit.position - ray.origin) < 0.0 { hit.distance *= -1.0 };
		if hit.distance < 0.0 { return None; }
		Some(hit)
	}
}

#[derive(Serialize, Deserialize)]
pub struct EllipsoidFormat {
	pub origin: VectorFormat,
	pub scale: VectorFormat,
	pub rot_axis: VectorFormat,
	pub rot_angle: f64,
	pub material: MaterialFormat,
}

impl From<EllipsoidFormat> for Ellipsoid {
    fn from(v: EllipsoidFormat) -> Ellipsoid { 
		Ellipsoid::new(v.origin.into(), v.scale.into(), v.rot_axis.into(), Rad(v.rot_angle), v.material.into())
    }
}

impl From<Ellipsoid> for EllipsoidFormat {
    fn from(v: Ellipsoid) -> EllipsoidFormat {
		// TODO: decompose transform matrix
        EllipsoidFormat {
            origin: Point3::new(0.0, 0.0, 0.0).into(),
			scale: Vector3::new(0.0, 0.0, 0.0).into(),
			rot_axis: Vector3::new(0.0, 0.0, 0.0).into(),
			rot_angle: 0.0,
            material: v.material.into(),
        }
    }
}