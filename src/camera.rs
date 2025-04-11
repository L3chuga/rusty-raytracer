use std::{f64::INFINITY, fs::{File, OpenOptions}, io::{self, BufWriter, Write}};
use crate::hittable::*;
use crate::vec3::*;
use crate::utilities::*;


fn random_unit_square() -> Vec3 {
    return Vec3::new(rand::random::<f64>()-0.5,rand::random::<f64>()-0.5,0.0);
}

pub struct Camera {
    image_width : i32,
    aspect_ratio : f64,
    ppm_config : String,
    samples_per_pixel : i32,
    max_depth : i32,
    vfov : f64,
    look_at : Vec3,
    vup : Vec3,

    viewport_height : f64,
    viewport_width : f64,
    focal_lenght : f64,

    image_height : i32,
    origin : Vec3,
    pixel_du : Vec3,
    pixel_dv : Vec3,    
    pixel_origin : Vec3,
    pixel_sample_scale : f64,
    u : Vec3,
    v : Vec3,
    w : Vec3,
}

impl Camera {
    pub fn new(ppm_config : &str, image_width : i32, aspect_ratio : f64, samples_per_pixel : i32, max_depth : i32, vfov : f64, origin : Vec3, look_at : Vec3) -> Self {
        let mut camera : Camera = Camera {
            image_width, // Initial Values
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            vfov,
            ppm_config: ppm_config.to_string(),
            origin,
            look_at,
            vup: Vec3::new(0.0,1.0,0.0),

            focal_lenght: 0.0, // Derived Values
            viewport_height: 0.0,
            viewport_width: 0.0,
            image_height: 0,
            pixel_du: ZERO,
            pixel_dv: ZERO, 
            pixel_origin: ZERO,
            pixel_sample_scale : 0.0,
            u: ZERO,
            v: ZERO,
            w: ZERO 
        };

        camera.focal_lenght = (camera.origin-camera.look_at).norm();
        camera.image_height = ((camera.image_width as f64)/camera.aspect_ratio) as i32;

        let theta: f64 = degrees_to_radians(camera.vfov);
        let h = f64::tan(theta/2.0);
        camera.viewport_height = 2.0*h*camera.focal_lenght;
        camera.viewport_width = camera.viewport_height*((camera.image_width as f64)/(camera.image_height as f64));
        
        camera.w = (camera.origin-camera.look_at).normalized();
        camera.u = Vec3::cross(&camera.vup,&camera.w).normalized();
        camera.v = Vec3::cross(&camera.w,&camera.u);

        let viewport_u: Vec3 = camera.u*camera.viewport_width;
        let viewport_v: Vec3 = camera.v*camera.viewport_height*(-1.0);

        camera.pixel_du = viewport_u/(camera.image_width as f64);
        camera.pixel_dv = viewport_v/(camera.image_height as f64);
        
        let viewport_upper_left: Vec3 = camera.origin
            - camera.w*(camera.focal_lenght)
            - viewport_u/2.0
            - viewport_v/2.0;
        camera.pixel_origin = viewport_upper_left+(camera.pixel_du+camera.pixel_dv)/2.0;

        camera.pixel_sample_scale = 1.0/(camera.samples_per_pixel as f64);

        return camera;
    }

    pub fn render(&self, output_path : &str, world : &HittableList) {
        
        //output_buffer.write(self.ppm_config.as_bytes()).ok();
        let mut pixel_data : String = String::from("");
        let mut k = 1; let ten_percent = self.image_height/10;
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Color::new(0.0,0.0,0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i,j);
                    color = color+self.ray_color(&r,&world, self.max_depth);
                }
                color = color*self.pixel_sample_scale;
                color = Vec3::new(linear_to_gamma(color.x()),linear_to_gamma(color.y()),linear_to_gamma(color.z()));
                pixel_data += &format!("{} {} {} ",color.r(),color.g(),color.b());
            }
            pixel_data += "\n";

            if j >= ten_percent*k || j == self.image_height-1 {
                let output_file: File = OpenOptions::new().write(true).truncate(true).create(true).open(output_path).expect("Unable to open file.");
                let mut output_buffer: BufWriter<File> = io::BufWriter::new(output_file);
                output_buffer.write(self.ppm_config.as_bytes()).ok();
                output_buffer.write(pixel_data.as_bytes()).ok();
                output_buffer.write( ("0 0 0 ").repeat((self.image_width*(self.image_height-j-1)) as usize).as_bytes()).ok();

                let percent = k*10;
                println!("{percent}% completed.");
                k+=1;
            }
        }
        
    }

    fn get_ray(&self, i : i32, j : i32) -> Ray {
        let offset = random_unit_square();
        let pixel_sample: Vec3 = self.pixel_origin 
            + self.pixel_du*(i as f64 + offset.x()) 
            + self.pixel_dv*(j as f64 + offset.y());
        let ray_dir: Vec3 = (pixel_sample-self.origin).normalized();
        return Ray::new(self.origin, ray_dir);
    }

    fn ray_color(&self, r : &Ray, world : &HittableList, depth : i32) -> Color {
        if depth<=0 {return BLACK}

        let hr = world.hit(&r, Interval::new(0.001,INFINITY));
        if hr.has_hit() {
            let (scattered, attenuation) = hr.material().scatter(r, &hr);
            return self.ray_color(&scattered, world, depth-1)*attenuation;
        } else {
            let lamda : f64 = 0.5*(r.dir().y() + 1.0);
            return WHITE*(1.0-lamda)+SKY_BLUE*lamda;
        }
    }
}