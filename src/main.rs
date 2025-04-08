use std::{fs::{File,OpenOptions}, io::{self, BufWriter, Write}};
use const_format::formatcp;

mod vec3;
mod hittable;
use crate::vec3::*;
use crate::hittable::*;

const OUTPUT_PATH : &str = "image.ppm";
const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH : i32 = 400;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;
const VIEWPORT_HEIGHT : f64 = 2.0;
const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT*((WIDTH as f64)/(HEIGHT as f64));
const ORIGIN : Vec3 = Vec3::new(0.0, 0.0, 0.0);
const FOCAL_LENGHT : f64 = 1.0;

pub const RGB_MAX : i32 = 255;
const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");


fn write_to_file(output_buffer : &mut BufWriter<File>, c : &Color){
    output_buffer.write(&format!("{} {} {}\n",c.r(),c.g(),c.b()).as_bytes()).ok();
}



const WHITE : Color = Color::new(Vec3::new(1.0,1.0,1.0));
const BLUE : Color = Color::new(Vec3::new(0.5,0.7,1.0));
const RED : Color = Color::new(Vec3::new(1.0,0.0,0.0));





const TEST_SPHERE : Sphere = Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5);

fn ray_color(r : Ray) -> Color {
    let hr = TEST_SPHERE.hit(&r, 0.0, 100000.0);
    if hr.has_hit() {
        return Color::new((hr.normal()+1.0)*0.5)
    }

    let lamda : f64 = 0.5*(r.dir().y() + 1.0);
    return Color::new(WHITE.values()*(1.0-lamda)+BLUE.values()*lamda);
}

fn main() {

    // -- OUTPUT --
    let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);


    // -- CAMERA --
    let viewport_u: Vec3 = Vec3::new(VIEWPORT_WIDTH,0.0,0.0);
    let viewport_v: Vec3 = Vec3::new(0.0,-VIEWPORT_HEIGHT,0.0);

    let pixel_du: Vec3 = viewport_u/(WIDTH as f64);
    let pixel_dv: Vec3 = viewport_v/(HEIGHT as f64);
    
    let pixel_origin: Vec3 = ORIGIN 
        - Vec3::new(0.0,0.0,FOCAL_LENGHT)
        - viewport_u/2.0
        - viewport_v/2.0
        + pixel_du/2.0
        + pixel_dv/2.0;

    // -- WORLD --
    let mut world : HittableList = HittableList::new();
    world.add(&TEST_SPHERE);

    // -- RENDER --
    output_buffer.write(PPM_CONFIG.as_bytes()).ok();
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let pixel_center: Vec3 = pixel_origin + pixel_du*(i as f64) + pixel_dv*(j as f64); 
            let ray_dir: Vec3 = (pixel_center-ORIGIN).normalized();
            let r = Ray::new(ORIGIN, ray_dir);

            let color: Color = ray_color(r);
            write_to_file(&mut output_buffer, &color);
        }
    }
}
