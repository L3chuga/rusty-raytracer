use std::{f64::consts::PI, fs::{File,OpenOptions}, io::{self, BufWriter, Write}, sync::Arc};
use const_format::formatcp;
use material::Lambertian;

mod vec3;
mod hittable;
mod camera;
mod utilities;
mod material;

use crate::vec3::*;
use crate::hittable::*;
use crate::camera::*;
use crate::utilities::*;
use crate::material::*;

const OUTPUT_PATH : &str = "image.ppm";
const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH : i32 = 1200;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;
pub const RGB_MAX : i32 = 255;

const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

const GROUND_MATERIAL : &dyn Material = &Lambertian::new(Color::new(Vec3::new(0.5,0.5,0.5)));
const CENTER_MATERIAL : &dyn Material = &Lambertian::new(Color::new(Vec3::new(0.1,0.2,0.5)));
const LEFT_MATERIAL :   &dyn Material = &Dielectric::new(1.5);
const BUBBLE_MATERIAL : &dyn Material = &Dielectric::new(1.0/1.5);
const RIGHT_MATERIAL :  &dyn Material = &Metal::new(Color::new(Vec3::new(0.8,0.6,0.2)),0.0);

const S1 : Sphere = Sphere::new(Vec3::new(0.0,1.0,0.0),1.0, LEFT_MATERIAL);
const S2 : Sphere = Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0, &Lambertian::new(Color::new(Vec3::new(0.4,0.2,0.1))));
const S3 : Sphere = Sphere::new(Vec3::new(4.0,1.0,0.0),1.0, &Metal::new(Color::new(Vec3::new(0.7,0.6,0.5)), 0.0));

fn main() {
    // -- OUTPUT --
    let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);

    // -- CAMERA --
    let camera : Camera = Camera::new(PPM_CONFIG,WIDTH,ASPECT_RATIO);

    // -- WORLD --

    /* 
    let ground_material : Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(Vec3::new(0.5,0.5,0.5))));
    let center_material : Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(Vec3::new(0.1,0.2,0.5))));
    let left_material : Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let bubble_material : Arc<dyn Material> = Arc::new(Dielectric::new(1.0/1.5));
    let right_material : Arc<dyn Material> = Arc::new(Metal::new(Color::new(Vec3::new(0.8,0.6,0.2)),0.0));
    */

    let ground_sphere : Sphere = Sphere::new(Vec3::new(0.0,-1000.0,-1.0),1000.0, GROUND_MATERIAL);

    let mut world : HittableList = HittableList::new();
    world.add(Box::new(ground_sphere));

    
    for a in -2..2 {
        for b in -2..2 {
            let choose_material = rand::random::<f64>();
            let center : Vec3 = Vec3::new(a as f64+0.9*rand::random::<f64>(), 0.2, b as f64+0.9*rand::random::<f64>());
            if (center-Vec3::new(4.0,0.2,-1.0)).norm_squared() > 0.9 {
                if choose_material < 0.8 {
                    world.add(Box::new(Sphere::new(center, 0.2, CENTER_MATERIAL)));
                } else if choose_material < 0.95 {
                    world.add(Box::new(Sphere::new(center, 0.2, LEFT_MATERIAL)));
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, RIGHT_MATERIAL)));
                }
            }
        }
    }
    println!("Generating random spheres done.");

    world.add(Box::new(S1));
    world.add(Box::new(S2));
    world.add(Box::new(S3));
    
    // -- RENDER --
    camera.render(&mut output_buffer, &world);
}
