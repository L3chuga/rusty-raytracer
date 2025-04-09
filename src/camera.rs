use std::{f64::INFINITY, fs::File, io::{BufWriter, Write}};
use crate::hittable::*;
use crate::vec3::*;
use crate::utilities::*;

fn ray_color(r : &Ray, world : &HittableList) -> Color {
    let hr = world.hit(&r, Interval::new(0.0,INFINITY));
    if hr.has_hit() {
        return Color::new((hr.normal()+1.0)*0.5)
    }

    let lamda : f64 = 0.5*(r.dir().y() + 1.0);
    return Color::new(WHITE.values()*(1.0-lamda)+BLUE.values()*lamda);
}

fn write_to_file(output_buffer : &mut BufWriter<File>, c : &Color){
    output_buffer.write(&format!("{} {} {}\n",c.r(),c.g(),c.b()).as_bytes()).ok();
}

pub struct Camera {
    image_width : i32,
    aspect_ratio : f64,
    ppm_config : String,

    viewport_height : f64,
    viewport_width : f64,
    focal_lenght : f64,

    image_height : i32,
    origin : Vec3,
    pixel_du : Vec3,
    pixel_dv : Vec3,    
    pixel_origin : Vec3
}

impl Camera {
    pub fn new(ppm_config : &str, image_width : i32, aspect_ratio : f64) -> Self {
        let mut camera : Camera = Camera {
            image_width, // Initial Values
            aspect_ratio,
            ppm_config: ppm_config.to_string(),
            viewport_height: 2.0,
            focal_lenght: 1.0,
            origin: ZERO,

            viewport_width: 0.0, // Derived values
            image_height: 0,
            pixel_du: ZERO,
            pixel_dv: ZERO, 
            pixel_origin: ZERO
        };

        camera.image_height = ((camera.image_width as f64)/camera.aspect_ratio) as i32;

        camera.origin = Vec3::new(0.0,0.0,0.0);
        camera.viewport_width = camera.viewport_height*((camera.image_width as f64)/(camera.image_height as f64));
        
        let viewport_u: Vec3 = Vec3::new(camera.viewport_width,0.0,0.0);
        let viewport_v: Vec3 = Vec3::new(0.0,-camera.viewport_height,0.0);

        camera.pixel_du = viewport_u/(camera.image_width as f64);
        camera.pixel_dv = viewport_v/(camera.image_height as f64);
        
        camera.pixel_origin = camera.origin
            - Vec3::new(0.0,0.0,camera.focal_lenght)
            - viewport_u/2.0
            - viewport_v/2.0
            + camera.pixel_du/2.0
            + camera.pixel_dv/2.0;

        return camera;
    }

    pub fn render(&self, output_buffer : &mut BufWriter<File>, world : &HittableList) {
        output_buffer.write(self.ppm_config.as_bytes()).ok();
        dbg!(self.image_width,self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel_origin + self.pixel_du*(i as f64) + self.pixel_dv*(j as f64); 
                let ray_dir: Vec3 = (pixel_center-self.origin).normalized();
                let r = Ray::new(self.origin, ray_dir);

                let color: Color = ray_color(&r,&world);
                write_to_file(output_buffer, &color);
            }
        }
    }
}