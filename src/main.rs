use std::{fs::{File,OpenOptions}, io::{self, BufWriter, Write}, mem::Discriminant};
use const_format::formatcp;

const OUTPUT_PATH : &str = "image.ppm";
const ASPECT_RATIO : f64 = 16.0/9.0;
const WIDTH : i32 = 400;
const HEIGHT : i32 = ((WIDTH as f64)/ASPECT_RATIO) as i32;
const VIEWPORT_HEIGHT : f64 = 2.0;
const VIEWPORT_WIDTH : f64 = VIEWPORT_HEIGHT*((WIDTH as f64)/(HEIGHT as f64));
const ORIGIN : Vec3 = Vec3 {x:0.0, y:0.0, z:0.0};
const FOCAL_LENGHT : f64 = 1.0;

const RGB_MAX : i32 = 255;
const PPM_CONFIG : &str = formatcp!("P3\n{WIDTH} {HEIGHT}\n{RGB_MAX}\n");


fn write_to_file(output_buffer : &mut BufWriter<File>, c : &Color){
    output_buffer.write(&format!("{} {} {}\n",c.r(),c.g(),c.b()).as_bytes()).ok();
}
#[derive(Clone, Copy)]
struct Vec3 {
    x : f64,
    y : f64,
    z : f64
}

impl Vec3 {
    const fn new(x : f64, y : f64, z : f64) -> Vec3 {
        Vec3 {x,y,z}
    }

    fn norm_squared(self) -> f64 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    fn norm(self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    fn normalized(self) -> Vec3 {
        self.clone()/self.norm()
    }

    const fn dot(u : Vec3, v : Vec3) -> f64 {
        u.x*v.x+u.y*v.y+u.z*v.z
    }

    const fn cross(u : Vec3, v : Vec3) -> Vec3 {
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

impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar : f64) -> Vec3 {
        self*(1.0/scalar)
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
impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other : Vec3) -> Vec3 {
        Vec3 {
            x : self.x-other.x,
            y : self.y-other.y,
            z : self.z-other.z
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

struct Color {
    values : Vec3
}

impl Color {
    const fn new(val : Vec3) -> Self {
        Color {values : val}
    }

    fn r(&self) -> i16 {
        (255.999*self.values.x) as i16
    }
    fn g(&self) -> i16 {
        (255.999*self.values.y) as i16
    }
    fn b(&self) -> i16 {
        (255.999*self.values.z) as i16
    }
}

const WHITE : Color = Color{values:Vec3::new(1.0,1.0,1.0)};
const BLUE : Color = Color{values:Vec3::new(0.5,0.7,1.0)};
const RED : Color = Color{values:Vec3::new(1.0,0.0,0.0)};

struct Sphere {
    center : Vec3,
    radius : f64
}

fn hit_sphere(s : &Sphere, ray : &Ray) -> bool {
    let a : f64 = Vec3::dot(ray.dir,ray.dir);
    let b : f64 = -2.0*Vec3::dot(ray.dir, s.center-ray.origin);
    let c : f64 = Vec3::dot(s.center-ray.origin,s.center-ray.origin)-s.radius*s.radius;
    let discriminant : f64 = b*b-4.0*a*c;
    return discriminant>=0.0
}

const TEST_SPHERE : Sphere = Sphere {center: Vec3::new(0.0,0.0,1.0), radius:0.5};

fn ray_color(r : Ray) -> Color {
    if hit_sphere(&TEST_SPHERE, &r) {
        return RED
    }

    let lamda : f64 = 0.5*(r.dir.y + 1.0);
    return Color::new(WHITE.values*(1.0-lamda)+BLUE.values*lamda);
}

fn main() {
    let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(OUTPUT_PATH).expect("Unable to open file.");
    let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);
    output_buffer.write(PPM_CONFIG.as_bytes()).ok();

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

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let pixel_center: Vec3 = pixel_origin + pixel_du*(i as f64) + pixel_dv*(j as f64); 
            let ray_dir: Vec3 = (pixel_center-ORIGIN).normalized();
            let r = Ray {origin : ORIGIN, dir : ray_dir};

            let color: Color = ray_color(r);
            write_to_file(&mut output_buffer, &color);
        }
    }
}
