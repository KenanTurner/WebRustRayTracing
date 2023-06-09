use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3, Rad, Matrix4};
use cgmath::{InnerSpace, Matrix, Transform, One, MetricSpace, EuclideanSpace};
use crate::{Ray, Hit, Intersect, Material, VectorFormat, MaterialFormat};

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "MeshFormat", into = "MeshFormat")]
pub struct Mesh {
	pub vertices: Vec<Vector3<f64>>,
	pub normals: Vec<Vector3<f64>>,
	pub transform: Matrix4<f64>,
	pub material: Material,
}

struct TriangleHit {
	u: f64,
	v: f64,
	t: f64,
}

const EPSILON: f64 = 0.000001;
impl Mesh {
	fn new(vertices: Vec<Vector3<f64>>, normals: Vec<Vector3<f64>>, origin: Point3<f64>, scale: Vector3<f64>, rot_axis: Vector3<f64>, rot_angle: Rad<f64>, material: Material) -> Mesh {
		Mesh {
			vertices,
			normals,
			transform: Matrix4::one()
				* Matrix4::from_translation(origin.to_vec())
				* Matrix4::from_axis_angle(rot_axis, rot_angle)
				* Matrix4::from_nonuniform_scale(scale.x, scale.y, scale.z),
			material,
		}
	}
	fn intersect_triangle(&self, ray_origin: Vector3<f64>, ray_direction: Vector3<f64>, vert0: Vector3<f64>, vert1: Vector3<f64>, vert2: Vector3<f64>) -> Option<TriangleHit> {

		/* find vectors for two edges sharing vert0 */
		let edge1 = vert1 - vert0;
		let edge2 = vert2 - vert0;

		/* begin calculating determinant - also used to calculate U parameter */
		let pvec = ray_direction.cross(edge2);

		/* if determinant is near zero, ray lies in plane of triangle */
		let det = cgmath::dot(edge1, pvec);

		/* calculate distance from vert0 to ray origin */
		let tvec = ray_origin - vert0;
		let inv_det = 1.0 / det;

		let qvec = tvec.cross(edge1);
		
		let mut u: f64;
		let mut v: f64;
		if det > EPSILON {
			u = cgmath::dot(tvec, pvec);
			if u < 0.0 || u > det { return None; }
			
			/* calculate V parameter and test bounds */
			v = cgmath::dot(ray_direction, qvec);
			if v < 0.0 || u + v > det { return None; }
		} else if det < -EPSILON {
			/* calculate U parameter and test bounds */
			u = cgmath::dot(tvec, pvec);
			if u > 0.0 || u < det { return None; }

			/* calculate V parameter and test bounds */
			v = cgmath::dot(ray_direction, qvec);
			if v > 0.0 || u + v < det { return None; }
		}else{
			return None;  /* ray is parallell to the plane of the triangle */
		}

		let t = cgmath::dot(edge2, qvec) * inv_det;
		u *= inv_det;
		v *= inv_det;
		
		if t < 0.0 { return None; }

		return Some(TriangleHit{
			u,
			v,
			t,
		});
	}
}

impl Intersect for Mesh {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		let transform_inv = self.transform.inverse_transform().unwrap();
		
		let trans_ray_origin = transform_inv.transform_point(ray.origin);
		let trans_ray_origin_vec = Vector3::new(trans_ray_origin.x, trans_ray_origin.y, trans_ray_origin.z); // bruh
		let trans_ray_direction = transform_inv.transform_vector(ray.direction).normalize();
		
		let hit = self.vertices.chunks_exact(3).enumerate().filter_map(|(i, x)|{
			if let Some(triangle_hit) = self.intersect_triangle(trans_ray_origin_vec, trans_ray_direction, x[0], x[1], x[2]) {
				let w = 1.0 - triangle_hit.u - triangle_hit.v;
				let distance = triangle_hit.t;
				let position = Point3::new(0.0, 0.0, 0.0) + (triangle_hit.u * x[1]) + (triangle_hit.v * x[2]) + (w * x[0]);
				let normal = if self.normals.len() == self.vertices.len() {
					let i = (i * 3) as usize;
					(triangle_hit.u * self.normals[i + 1]) + (triangle_hit.v * self.normals[i + 2]) + (w * self.normals[i + 0])
				} else {
					(x[1] - x[0]).cross(x[2] - x[0])
					
				};
				Some(Hit {
					distance,
					position,
					normal: normal.normalize(),
					material: self.material.clone(),
				})
			} else {
				None
			}
		}).min()?;
		
		let mut hit = Hit {
			distance: ray.origin.distance(self.transform.transform_point(hit.position)),
			position: self.transform.transform_point(hit.position),
			normal: transform_inv.transpose().transform_vector(hit.normal).normalize(),
			material: self.material.clone(),
		};
		if cgmath::dot(ray.direction, hit.position - ray.origin) < 0.0 { hit.distance *= -1.0 };
		if hit.distance < 0.0 { return None; }
		Some(hit)
	}
}

#[derive(Serialize, Deserialize)]
pub struct MeshFormat {
	pub filename: String,
	pub origin: VectorFormat,
	pub scale: VectorFormat,
	pub rot_axis: VectorFormat,
	pub rot_angle: f64,
	pub material: MaterialFormat,
}

impl From<MeshFormat> for Mesh {
    fn from(v: MeshFormat) -> Mesh {
		let options = tobj::LoadOptions {
			single_index: true,
			triangulate: false,
			ignore_points: true,
			ignore_lines: true,
		};
		let model_file = tobj::load_obj(v.filename, &options);
		let (mut models, _) = model_file.expect("Failed to load OBJ file!");
		let mesh = models.remove(0).mesh;
		let mut vertices: Vec<Vector3<f64>> = vec![];
		let mut normals: Vec<Vector3<f64>> = vec![];
		
		for index in &mesh.indices {
			let pos_offset = (3 * index) as usize;
			
			let position = Vector3::new(
				mesh.positions[pos_offset],
				mesh.positions[pos_offset + 1],
				mesh.positions[pos_offset + 2],
			);
			vertices.push(position);
			
			if !mesh.normals.is_empty() {
				let normal = Vector3::new(
					mesh.normals[pos_offset],
					mesh.normals[pos_offset + 1],
					mesh.normals[pos_offset + 2],
				);
				normals.push(normal);
			}
		}
		
		Mesh::new(vertices, normals, v.origin.into(), v.scale.into(), v.rot_axis.into(), Rad(v.rot_angle), v.material.into())
    }
}

impl From<Mesh> for MeshFormat {
    fn from(v: Mesh) -> MeshFormat {
        MeshFormat {
			filename: String::from("[filename]"),
            origin: Point3::new(0.0, 0.0, 0.0).into(),
			scale: Vector3::new(0.0, 0.0, 0.0).into(),
			rot_axis: Vector3::new(0.0, 0.0, 0.0).into(),
			rot_angle: 0.0,
            material: v.material.into(),
        }
    }
}