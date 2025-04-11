use const_format::formatcp;
use std::time::{SystemTime, UNIX_EPOCH};


mod vec3;
mod hittable;
mod camera;
mod utilities;
mod material;

use crate::vec3::*;
use crate::hittable::*;
use crate::camera::*;
use crate::material::*;

const OUTPUT_PATH : &str = "image.ppm";

const ASPECT_RATIO : f64 = 16.0/9.0;
const CAMERA_ORIGIN : Vec3 = Vec3::new(13.0,2.0,3.0);
const CAMERA_DIRECTION : Vec3 = Vec3::new(0.0,0.0,0.0);
const VFOV : f64 = 20.0;

const WIDTH : i32 = 800;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;

const SAMPLES_PER_PIXEL : i32 = 40;
const MAX_DEPTH : i32 = 20;

pub const RGB_MAX : i32 = 255;
const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

const GROUND_MATERIAL : Material = Material::Lambertian { albedo: Color::new(0.5,0.5,0.5) };
const LEFT_MATERIAL : Material = Material::Lambertian { albedo: Color::new(0.4,0.2,0.5) };
const CENTER_MATERIAL : Material = Material::Metal { albedo: Color::new(0.7,0.6,0.5), fuzz: 0.0 };
const RIGHT_MATERIAL_1 : Material = Material::Dielectric { refraction_index: 1.5 };
const RIGHT_MATERIAL_2 : Material = Material::Dielectric { refraction_index: 1.0/1.5 };

const S1 : Sphere = Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,CENTER_MATERIAL);
const S2 : Sphere = Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0, LEFT_MATERIAL);
const S3 : Sphere = Sphere::new(Vec3::new(4.0,1.0,0.0),1.0, RIGHT_MATERIAL_1);
const S4 : Sphere = Sphere::new(Vec3::new(4.0,1.0,0.0),0.8, RIGHT_MATERIAL_2);

fn generate_random_spheres(world : &mut HittableList) {
    for a in -3..3 {
        for b in -3..3 {
            let choose_material = rand::random::<f64>();
            let center : Vec3 = Vec3::new(a as f64+1.5*rand::random::<f64>(), 0.2, b as f64+1.5*rand::random::<f64>());
            if (center-Vec3::new(4.0,0.2,-1.0)).norm_squared() > 0.9 {
                if choose_material < 0.7 {
                    let albedo : Color = Vec3::random_vec() * Vec3::random_vec();
                    world.add(Box::new(Sphere::new(center, 0.2, Material::Lambertian { albedo })));
                } else if choose_material < 0.85 {
                    let albedo : Color = (Vec3::random_vec()+1.0)/2.0;
                    world.add(Box::new(Sphere::new(center, 0.2, Material::Metal { albedo, fuzz: 0.0 })));
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, Material::Dielectric { refraction_index: 1.5 })));
                }
            }
        }
    }
}
fn main() {
    // -- CAMERA --
    let camera : Camera = Camera::new(PPM_CONFIG,WIDTH,ASPECT_RATIO,SAMPLES_PER_PIXEL,MAX_DEPTH,VFOV, CAMERA_ORIGIN, CAMERA_DIRECTION);

    // -- WORLD --
    let ground_sphere : Sphere = Sphere::new(Vec3::new(0.0,-1000.0,-1.0),1000.0, GROUND_MATERIAL);
    let mut world : HittableList = HittableList::new();
    world.add(Box::new(ground_sphere));
    
    generate_random_spheres(&mut world);
    println!("Generating random spheres done.");
    
    world.add(Box::new(S1));
    world.add(Box::new(S2));
    world.add(Box::new(S3));
    world.add(Box::new(S4));
    
    // -- RENDER --
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    camera.render(OUTPUT_PATH, &world);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let elapsed = (end.as_millis() - start.as_millis()) as f64 /1000.0;
    println!("Rendering took {elapsed} secs.")
}
