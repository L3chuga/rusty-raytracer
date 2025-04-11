use crate::{utilities::WHITE, vec3::Vec3, Color, HitRecord, Ray};


#[derive(Clone, Copy)]
pub enum Material {
    Void,
    Lambertian {albedo : Color},
    Metal {albedo : Color, fuzz : f64},
    Dielectric {refraction_index : f64},
}

impl Material {
    pub fn scatter(&self, r : &Ray, hr: &HitRecord) -> (Ray, Color) {
        match self {
            Material::Void => (*r, WHITE),

            Material::Lambertian {albedo} => {
                let mut scatter_direction = (hr.normal()+Vec3::random_outwards(&hr.normal())).normalized();
                if scatter_direction.near_zero() {scatter_direction = hr.normal();}
                (Ray::new(hr.point(), scatter_direction), *albedo)
            },

            Material::Metal {albedo, fuzz} => {
                let reflect_dir = r.dir().reflect(&hr.normal());
                (Ray::new(hr.point(), reflect_dir.normalized()+Vec3::random_vec().normalized()*(*fuzz)), *albedo)
            },

            Material::Dielectric {refraction_index} => {
                let ri : f64;
                if hr.front_face() {ri = 1.0/refraction_index}
                else {ri = *refraction_index}

                let cos_theta: f64 = f64::min(Vec3::dot(&(r.dir()*(-1.0)),&hr.normal()),1.0);
                let sin_theta: f64 = f64::sqrt(1.0-cos_theta*cos_theta);
                let cannot_refract = ri*sin_theta > 1.0;
                let refract_dir: Vec3;

                if cannot_refract || reflectance(cos_theta, ri)>rand::random::<f64>() {refract_dir = r.dir().reflect(&hr.normal())}
                else {refract_dir = r.dir().refract(&hr.normal(), ri)}

                (Ray::new(hr.point(), refract_dir), WHITE)
            }
        }
    }
}

fn reflectance(cos : f64, refraction_index : f64) -> f64 {
    let mut r0 = (1.0-refraction_index)/(1.0+refraction_index);
    r0 = r0*r0;
    return r0 + (1.0-r0)*f64::powi(1.0-cos, 5);
}