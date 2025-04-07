use std::{fs, io::{self, BufWriter, Write}};
use const_format::formatcp;

const OUTPUT_PATH : &str = "image.ppm";
const WIDTH : i32 = 10;
const HEIGHT : i32 = 10;
const RGB_MAX : i32 = 255;
const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");

fn write_to_file(mut output_buffer : BufWriter<fs::File>){
    output_buffer.write(PPM_CONFIG.as_bytes()).ok();
    for _ in 0..HEIGHT {
        let mut pixel_row_rgb : String = String::new();
        for _ in 0..WIDTH {
            let (r,g,b) = (255,255,255);
            pixel_row_rgb.push_str(&format!("{r} {g} {b} "));
        }
        pixel_row_rgb.push('\n');
        output_buffer.write(pixel_row_rgb.as_bytes()).ok();
    }
}
#[derive(Clone, Copy)]
struct Vec3 {
    x : f64,
    y : f64,
    z : f64
}

impl Vec3 {
    fn new(x : f64, y : f64, z : f64) -> Vec3 {
        Vec3 {x,y,z}
    }

    fn norm_squared(self) -> f64 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    fn norm(self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    fn normalized(self) -> Vec3 {
        let vector_normalized = self.clone();
        let norm : f64 = self.norm();
        vector_normalized*(1f64/norm)
    }

    fn dot(u : Vec3, v : Vec3) -> f64 {
        u.x*v.x+u.y*v.y+u.z*v.z
    }

    fn cross(u : Vec3, v : Vec3) -> Vec3 {
        Vec3 {
            x : u.y*v.z-u.z*u.y,
            y : u.z*v.x-u.x*v.z,
            z : u.x*v.y-u.y*v.x 
        }
    }
}


struct Ray {
    origin : Vec3,
    dir : Vec3
}

impl Ray {
    fn at(self, t : f64) -> Vec3 {
        self.origin+self.dir*t
    }
}

impl std::ops::Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, scalar : f64) -> Vec3 {
        Vec3 {
            x : self.x+scalar,
            y : self.y+scalar,
            z : self.z+scalar
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar : f64) -> Vec3 {
        Vec3 {
            x : self.x*scalar,
            y : self.y*scalar,
            z : self.z*scalar
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other : Vec3) -> Vec3 {
        Vec3 {
            x : self.x+other.x,
            y : self.y+other.y,
            z : self.z+other.z
        }
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other : Vec3) -> Vec3 {
        Vec3 {
            x : self.x*other.x,
            y : self.y*other.y,
            z : self.z*other.z
        }
    }
}

fn main() {
    let output_file = fs::OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let output_buffer = io::BufWriter::new(output_file);
    let test : Vec3 = Vec3::new(1.5f64, 0f64, 0f64);
    
    //write_to_file(output_buffer);
}
