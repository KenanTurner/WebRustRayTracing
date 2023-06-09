use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3};

#[derive(Serialize, Deserialize)]
pub struct VectorFormat {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl From<VectorFormat> for Point3<f64> {
    fn from(v: VectorFormat) -> Point3<f64> { 
        Point3::new(v.x, v.y, v.z)
    }
}

impl From<Point3<f64>> for VectorFormat {
    fn from(v: Point3<f64>) -> VectorFormat { 
        VectorFormat {
            x: v.x,
			y: v.y,
			z: v.z,
        }
    }
}

impl From<VectorFormat> for Vector3<f64> {
    fn from(v: VectorFormat) -> Vector3<f64> { 
        Vector3::new(v.x, v.y, v.z)
    }
}

impl From<Vector3<f64>> for VectorFormat {
    fn from(v: Vector3<f64>) -> VectorFormat { 
        VectorFormat {
            x: v.x,
			y: v.y,
			z: v.z,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RgbFormat {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl From<RgbFormat> for Point3<f64> {
    fn from(v: RgbFormat) -> Point3<f64> { 
        Point3::new(v.r, v.g, v.b)
    }
}

impl From<Point3<f64>> for RgbFormat {
    fn from(v: Point3<f64>) -> RgbFormat { 
        RgbFormat {
            r: v.x,
			g: v.y,
			b: v.z,
        }
    }
}

impl From<RgbFormat> for Vector3<f64> {
    fn from(v: RgbFormat) -> Vector3<f64> { 
        Vector3::new(v.r, v.g, v.b)
    }
}

impl From<Vector3<f64>> for RgbFormat {
    fn from(v: Vector3<f64>) -> RgbFormat { 
        RgbFormat {
            r: v.x,
			g: v.y,
			b: v.z,
        }
    }
}