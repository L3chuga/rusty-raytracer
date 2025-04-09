use std::{fs::{File,OpenOptions}, io::{self, BufWriter, Write}};
use const_format::formatcp;

mod vec3;
mod hittable;
mod camera;
mod utilities;

use crate::vec3::*;
use crate::hittable::*;
use crate::camera::*;

const OUTPUT_PATH : &str = "image.ppm";
const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH : i32 = 400;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;
pub const RGB_MAX : i32 = 255;

const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

const TEST_SPHERE_1 : Sphere = Sphere::new(Vec3::new(-1.0,0.0,-2.0), 0.5);
const TEST_SPHERE_2 : Sphere = Sphere::new(Vec3::new(1.0,0.0,-2.0), 0.5);
const GROUND : Sphere = Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0);


fn main() {

    // -- OUTPUT --
    let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);


    // -- CAMERA --
    let camera : Camera = Camera::new(PPM_CONFIG,WIDTH,ASPECT_RATIO);

    // -- WORLD --
    let mut world : HittableList = HittableList::new();
    world.add(&TEST_SPHERE_1);
    world.add(&TEST_SPHERE_2);
    world.add(&GROUND);

    // -- RENDER --
    camera.render(&mut output_buffer, &world);
}
