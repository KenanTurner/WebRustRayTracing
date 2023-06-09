use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3, Matrix4};
use crate::{Ray, VectorFormat};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "CameraFormat", into = "CameraFormat")]
pub struct Camera {
	pub origin: Point3<f64>,
	pub direction: Vector3<f64>,
	pub fovy: f64,
	pub aspect: f64,
}

impl Camera {
	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		let h: f64 = (self.fovy / 2.0).tan();
		let viewport_height: f64 = 2.0 * h;
		let viewport_width: f64 = self.aspect * viewport_height;
		
		let target = self.origin + self.direction;
		let up = Vector3::new(0.0, 1.0, 0.0);
		let view = Matrix4::look_at_rh(self.origin, target, up);
		
		let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
		let vertical = Vector3::new(0.0, viewport_height, 0.0);
		let lower_left_corner = Vector3::new(0.0, 0.0, -1.0) - (horizontal/2.0) - (vertical/2.0);
		
		let direction = lower_left_corner + u*horizontal + v*vertical;
		let direction = (view * direction.extend(0.0)).truncate();
		
		Ray {
			origin: self.origin,
			direction,
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct CameraFormat {
	pub origin: VectorFormat,
	pub direction: VectorFormat,
	pub fovy: f64,
	// pub aspect: f64,
}

impl From<CameraFormat> for Camera {
    fn from(v: CameraFormat) -> Camera { 
        Camera {
            origin: v.origin.into(),
			direction: v.direction.into(),
            fovy: v.fovy,
			aspect: 1.0,
        }
    }
}

impl From<Camera> for CameraFormat {
    fn from(v: Camera) -> CameraFormat { 
        CameraFormat {
			origin: v.origin.into(),
			direction: v.direction.into(),
            fovy: v.fovy,
			// aspect: v.aspect,
        }
    }
}