use std::{f64::consts::PI, fs::{File,OpenOptions}, io::{self, BufWriter, Write}};
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
const WIDTH : i32 = 400;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;
pub const RGB_MAX : i32 = 255;

const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

const GROUND_MATERIAL : &dyn Material = &Lambertian::new(Color::new(Vec3::new(0.8,0.8,0.0)));
const CENTER_MATERIAL : &dyn Material = &Lambertian::new(Color::new(Vec3::new(0.1,0.2,0.5)));
const LEFT_MATERIAL : &dyn Material = &Dielectric::new(1.5);
const BUBBLE_MATERIAL : &dyn Material = &Dielectric::new(1.0/1.5);
const RIGHT_MATERIAL : &dyn Material = &Metal::new(Color::new(Vec3::new(0.8,0.6,0.2)),0.0);

const SPHERE_GROUND : Sphere = Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0, GROUND_MATERIAL);
const SPHERE_1 : Sphere = Sphere::new(Vec3::new(0.0,0.0,-1.2),0.5, CENTER_MATERIAL);
const SPHERE_2 : Sphere = Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.5, LEFT_MATERIAL);
const SPHERE_3 : Sphere = Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.4, BUBBLE_MATERIAL);
const SPHERE_4 : Sphere = Sphere::new(Vec3::new(1.0,0.0,-1.0),0.5, RIGHT_MATERIAL);

fn main() {
    // -- OUTPUT --
    let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);

    // -- CAMERA --
    let camera : Camera = Camera::new(PPM_CONFIG,WIDTH,ASPECT_RATIO);

    // -- WORLD --
    let mut world : HittableList = HittableList::new();
    
    world.add(&SPHERE_GROUND);
    world.add(&SPHERE_1);
    world.add(&SPHERE_2);
    world.add(&SPHERE_3);
    world.add(&SPHERE_4);
    // -- RENDER --
    camera.render(&mut output_buffer, &world);
}
