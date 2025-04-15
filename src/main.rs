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
use crate::utilities::*;

const OUTPUT_PATH : &str = "image.ppm";

const ASPECT_RATIO : f64 = 16.0/9.0;
const CAMERA_ORIGIN : Vec3 = Vec3::new(0.0,0.0,5.0);
const CAMERA_DIRECTION : Vec3 = Vec3::new(0.0,0.0,0.0);
const VFOV : f64 = 80.0;

const WIDTH : i32 = 800;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;

const SAMPLES_PER_PIXEL : i32 = 500;
const MAX_DEPTH : i32 = 30;

pub const RGB_MAX : i32 = 255;
const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

const GROUND_MATERIAL : Material = Material::Lambertian { albedo: Color::new(0.5,0.5,0.5) };
const LIGHT_MATERIAL : Material = Material::LightSource { intensity: Color::new(5.0,5.0,5.0) };
const GROUND_SPHERE : Sphere = Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0, GROUND_MATERIAL);
const LIGHT : Sphere = Sphere::new(Vec3::new(0.0,2.0,0.0),1.0,LIGHT_MATERIAL);

fn generate_random_spheres(world : &mut HittableList) {
    for a in -2..2 {
        for b in -2..2 {
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

fn scene_1(world : &mut HittableList) {
    const LEFT_MATERIAL : Material = Material::Lambertian { albedo: Color::new(0.4,0.2,0.5) };
    const CENTER_MATERIAL : Material = Material::Metal { albedo: Color::new(0.7,0.6,0.5), fuzz: 0.0 };
    const RIGHT_MATERIAL_1 : Material = Material::Dielectric { refraction_index: 1.5 };
    const RIGHT_MATERIAL_2 : Material = Material::Dielectric { refraction_index: 1.0/1.5 };
    

    
    const S2 : Sphere = Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0, LEFT_MATERIAL);
    const S3 : Sphere = Sphere::new(Vec3::new(4.0,1.0,0.0),1.0, RIGHT_MATERIAL_1);
    const S4 : Sphere = Sphere::new(Vec3::new(4.0,1.0,0.0),0.8, RIGHT_MATERIAL_2);
    generate_random_spheres(world);
    println!("Generating random spheres done.");

    world.add(Box::new(GROUND_SPHERE));
    world.add(Box::new(LIGHT));
    world.add(Box::new(S2));
    world.add(Box::new(S3));
    world.add(Box::new(S4));
}

fn scene_2(world : &mut HittableList) {
     
    let t1 : Triangle = Triangle::new(
        Vec3::new(-2.0,-2.0,0.0),
        Vec3::new(-2.0,2.0,0.0),
        Vec3::new(2.0,-2.0,0.0),
        Material::Lambertian { albedo: GRAY }
    );
    let t2 : Triangle = Triangle::new(
        Vec3::new(2.0,2.0,0.0),
        Vec3::new(-2.0,2.0,0.0),
        Vec3::new(2.0,-2.0,0.0),
        Material::Lambertian { albedo: GRAY }
    );

    let t3 : Triangle = Triangle::new(
        Vec3::new(-2.0,-2.0,0.0),
        Vec3::new(-2.0,2.0,0.0),
        Vec3::new(-2.0,-2.0,2.0),
        Material::Lambertian { albedo: GREEN}
    );
    
    let t4 : Triangle = Triangle::new(
        Vec3::new(-2.0,2.0,0.0),
        Vec3::new(-2.0,2.0,2.0),
        Vec3::new(-2.0,-2.0,2.0),
        Material::Lambertian { albedo: GREEN}
    );

    let t5 : Triangle = Triangle::new(
        Vec3::new(2.0,-2.0,0.0),
        Vec3::new(2.0,-2.0,2.0),
        Vec3::new(2.0,2.0,2.0),
        Material::Lambertian { albedo: RED}
    );

    let t6 : Triangle = Triangle::new(
        Vec3::new(2.0,-2.0,0.0),
        Vec3::new(2.0,2.0,2.0),
        Vec3::new(2.0,2.0,0.0),
        Material::Lambertian { albedo: RED}
    );

    let t7 : Triangle = Triangle::new(
        Vec3::new(-2.0,-2.0,0.0),
        Vec3::new(-2.0,-2.0,2.0),
        Vec3::new(2.0,-2.0,0.0),
        Material::Lambertian { albedo: GRAY}
    );

    let t8 : Triangle = Triangle::new(
        Vec3::new(2.0,-2.0,0.0),
        Vec3::new(-2.0,-2.0,2.0),
        Vec3::new(2.0,-2.0,2.0),
        Material::Lambertian { albedo: GRAY}
    );

    let t9 : Triangle = Triangle::new(
        Vec3::new(-2.0,2.0,0.0),
        Vec3::new(-2.0,2.0,2.0),
        Vec3::new(2.0,2.0,0.0),
        Material::Lambertian { albedo: GRAY}
    );

    let t10 : Triangle = Triangle::new(
        Vec3::new(2.0,2.0,0.0),
        Vec3::new(-2.0,2.0,2.0),
        Vec3::new(2.0,2.0,2.0),
        Material::Lambertian { albedo: GRAY}
    );
 
    let sphere : Sphere = Sphere::new(Vec3::new(0.0,1.8,1.0), 0.2 , Material::LightSource { intensity: Color::new(5.0,5.0,5.0) });

    //world.add(Box::new(GROUND_SPHERE));
    world.add(Box::new(t1));
    world.add(Box::new(t2));
    world.add(Box::new(t3));
    world.add(Box::new(t4));
    world.add(Box::new(t5));
    world.add(Box::new(t6));
    world.add(Box::new(t7));
    world.add(Box::new(t8));
    world.add(Box::new(t9));
    world.add(Box::new(t10));
    world.add(Box::new(sphere));
    //world.add(Box::new(LIGHT));
}

fn main() {
    // -- CAMERA --
    let camera : Camera = Camera::new(PPM_CONFIG,WIDTH,ASPECT_RATIO,SAMPLES_PER_PIXEL,MAX_DEPTH,VFOV, CAMERA_ORIGIN, CAMERA_DIRECTION);

    // -- WORLD --
    let mut world : HittableList = HittableList::new();    
    scene_2(&mut world);
    
    // -- RENDER --
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    camera.render(OUTPUT_PATH, &world);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let elapsed = (end.as_millis() - start.as_millis()) as f64 /1000.0;
    println!("Rendering took {elapsed} secs.")
}
