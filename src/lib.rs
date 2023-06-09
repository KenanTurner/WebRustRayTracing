use serde::{Serialize, Deserialize};
use cgmath::{Point3, Vector3};
use cgmath::{InnerSpace, ElementWise, MetricSpace};
use core::cmp::Ordering;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use rand::Rng;
extern crate console_error_panic_hook;
use std::panic;

pub use plane::Plane;
pub use sphere::Sphere;
pub use camera::Camera;
pub use ellipsoid::Ellipsoid;
pub use mesh::Mesh;
pub use material::Material;
pub use material::MaterialFormat;
pub use light::Light;
pub use format::{VectorFormat, RgbFormat};

mod plane;
mod sphere;
mod camera;
mod ellipsoid;
mod mesh;
mod material;
mod light;
mod format;

#[derive(Debug, PartialEq)]
pub struct Ray {
	pub origin: Point3<f64>,
	pub direction: Vector3<f64>,
}

impl Ray {
	pub fn at(&self, t: f64) -> Point3<f64> {
		self.origin + (self.direction * t)
	}
}

#[derive(Debug)]
pub struct Hit {
	pub distance: f64,
	pub position: Point3<f64>,
	pub normal: Vector3<f64>,
	pub material: Material,
}

impl PartialOrd for Hit {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Hit {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.distance < other.distance {
			Ordering::Less
		} else if self.distance > other.distance {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl PartialEq for Hit {
	fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Hit {}

pub trait Intersect {
	fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
	Plane(Plane),
	Sphere(Sphere),
	Ellipsoid(Ellipsoid),
	Mesh(Mesh),
}
impl Intersect for Object {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		match self {
			Object::Plane(object) => object.intersect(&ray),
			Object::Sphere(object) => object.intersect(&ray),
			Object::Ellipsoid(object) => object.intersect(&ray),
			Object::Mesh(object) => object.intersect(&ray),
		}
	}
}
pub type Objects = Vec<Object>;

impl Intersect for Objects {
	fn intersect(&self, ray: &Ray) -> Option<Hit> {
		self.iter().filter_map(|component|{
			component.intersect(ray)
		}).min()
	}
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
	pub lights: Vec<Light>,
	pub objects: Objects,
}

#[allow(non_snake_case)]
fn reflect(I: Vector3<f64>, N: Vector3<f64>) -> Vector3<f64> {
	I - N * cgmath::dot(N, I) * 2.0f64
}

pub fn shade_ray(objects: &Objects, lights: &Vec<Light>, camera: &Camera, ray: &Ray, bounces_remaining: u32) -> Option<Vector3<f64>> {
	if let Some(hit) = objects.intersect(&ray) {
		match hit.material {
			Material::DebugPosition => {
				return Some(Vector3::new(hit.position.x, hit.position.y, hit.position.z));
			}
			Material::DebugNormals => {
				return Some(hit.normal);
			}
			Material::DebugShadows => {
				let mut ray_color = Vector3::new(0.0, 0.0, 0.0);
				
				for light in lights {
					if cgmath::dot(hit.normal, light.position - hit.position) < 0.0 { 
						ray_color += Vector3::new(1.0, 0.0, 0.0);
						continue;
					}
					
					let mut shadow_ray = Ray {
						origin: hit.position,
						direction: (light.position - hit.position).normalize(),
					};
					shadow_ray.origin = shadow_ray.at(0.0001);
					
					if let Some(shadow_hit) = objects.intersect(&shadow_ray){
						if shadow_hit.distance <= light.position.distance(hit.position) {
							ray_color += Vector3::new(0.0, 1.0, 0.0);
							continue;
						}
					}
					ray_color += Vector3::new(0.0, 0.0, 1.0);
				}
				return Some(ray_color / lights.len() as f64);
			}
			Material::Emissive { color } => {
				return Some(color);
			}
			Material::Mirror => {
				if bounces_remaining == 0 { return None; }
				let mut reflection_ray = Ray {
					origin: hit.position,
					direction: reflect(ray.direction, hit.normal),
				};
				reflection_ray.origin = reflection_ray.at(0.0001);
				let reflection_camera = Camera {
					origin: reflection_ray.origin,
					direction: reflection_ray.direction,
					fovy: camera.fovy,
					aspect: camera.aspect,
				};
				return shade_ray(objects, lights, &reflection_camera, &reflection_ray, bounces_remaining-1);
			}
			Material::BlinnPhong { ambient, diffuse, specular, intensity } => {
				let mut ray_color = ambient;
		
				for light in lights {
					if cgmath::dot(hit.normal, light.position - hit.position) < 0.0 { continue; }
					
					let mut shadow_ray = Ray {
						origin: hit.position,
						direction: (light.position - hit.position).normalize(),
					};
					shadow_ray.origin = shadow_ray.at(0.0001);
					
					if let Some(shadow_hit) = objects.intersect(&shadow_ray){
						if shadow_hit.distance <= light.position.distance(hit.position) {
							continue;
						}
					}
					let r = light.position.distance(hit.position);
					ray_color += {
						let p_nor = hit.normal.normalize();
						let p_eye = (camera.origin - hit.position).normalize();
						let p_light = (light.position - hit.position).normalize();
						let p_half = (p_light + p_eye).normalize();
						let cd = diffuse * 0.0_f64.max(cgmath::dot(p_light, p_nor));
						let cs = specular * 0.0_f64.max(cgmath::dot(p_half, p_nor)).powf(intensity);
						let (constant, linear, quadratic) = light.attenuation.unwrap_or((1.0, 0.0, 0.0));
						let attenuation = 1.0 / (constant + (linear*r) + (quadratic*r*r));
						(light.color * light.intensity).mul_element_wise(cd + cs) * attenuation
					}
				}
				
				return Some(ray_color);
			}
		}
	}
	None
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
	json: &str,
    width: u32,
    height: u32,
	num_samples: u32,
	max_bounces: u32,
) -> Result<(), JsValue> {
	let mut scene = serde_json::from_str::<Scene>(&json).expect("Unable to parse scene json!");
	scene.camera.aspect = width as f64 / height as f64;
	
	let mut data = Vec::new();
	let mut rng = rand::thread_rng();
	let Scene { ref camera, ref lights, ref objects } = scene;
	for y in 0..height {
        for x in 0..width {
			let u = x as f64 / (width - 1) as f64;
			let v = (height - y - 1) as f64 / (height - 1) as f64;
			
			let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
			for i in 0..num_samples {
				let du: f64 = rng.gen_range(-0.5..=0.5) / (width - 1) as f64;
				let dv: f64 = rng.gen_range(-0.5..=0.5) / (height - 1) as f64;
				
				let ray = if i == 0 { camera.get_ray(u,v) } else { camera.get_ray(u + du,v + dv) };
				if let Some(ray_color) = shade_ray(&objects, &lights, &camera, &ray, max_bounces) {
					pixel_color += ray_color;
				}
			}
			let pixel_color = pixel_color.map(|v| (v / num_samples as f64)).map(|v| 255.0 * v.clamp(0.0, 1.0)).map(|v| v as u8);
			data.push(pixel_color.x);
			data.push(pixel_color.y);
			data.push(pixel_color.z);
			data.push(255u8);
		}
	}
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	Ok(())
}