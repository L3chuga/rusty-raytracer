use crate::{utilities::WHITE, vec3::Vec3, Color, HitRecord, Ray};

pub trait Material {
    fn scatter(&self, r : &Ray, hr: &HitRecord) -> (Ray,Color);
}
pub struct VoidMaterial;
impl VoidMaterial{
    pub const fn new() -> Self {
        VoidMaterial {}
    }
}
impl Material for VoidMaterial {
    fn scatter(&self, r : &Ray, _: &HitRecord) -> (Ray,Color) {
        return (*r, WHITE);
    }
}

pub struct Lambertian {
    albedo : Color
}

impl Lambertian {
    pub const fn new(albedo : Color) -> Self {
        Lambertian { albedo }
    }
}
impl Material for Lambertian{
    fn scatter(&self, _ : &Ray, hr: &HitRecord) -> (Ray, Color) {
        let mut scatter_direction = (hr.normal()+Vec3::random_vec()).normalized();
        if scatter_direction.near_zero() {scatter_direction = hr.normal();}
        return (Ray::new(hr.point(), scatter_direction), self.albedo);
    }
}

pub struct Metal {
    albedo : Color,
    fuzz : f64
}

impl Metal {
    pub const fn new(albedo : Color, fuzz : f64) -> Self {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal{
    fn scatter(&self, r : &Ray, hr: &HitRecord) -> (Ray, Color) {
        let reflect_dir = r.dir().reflect(&hr.normal());
        return (Ray::new(hr.point(), reflect_dir.normalized()+Vec3::random_vec().normalized()*self.fuzz), self.albedo);
    }
}

pub struct Dielectric {
    refraction_index : f64
}
impl Dielectric {
    pub const fn new(refraction_index : f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cos : f64, refraction_index : f64) -> f64 {
        let mut r0 = (1.0-refraction_index)/(1.0+refraction_index);
        r0 = r0*r0;
        return r0 + (1.0-r0)*f64::powi(1.0-cos, 5);
    }
}
impl Material for Dielectric {
    fn scatter(&self, r : &Ray, hr: &HitRecord) -> (Ray,Color) {
        let ri : f64;
        if hr.front_face() {ri = 1.0/self.refraction_index}
        else {ri = self.refraction_index}

        let cos_theta: f64 = f64::min(Vec3::dot(&(r.dir()*(-1.0)),&hr.normal()),1.0);
        let sin_theta: f64 = f64::sqrt(1.0-cos_theta*cos_theta);
        let cannot_refract = ri*sin_theta > 1.0;
        let refract_dir: Vec3;

        if cannot_refract || Dielectric::reflectance(cos_theta, ri)>rand::random::<f64>() {refract_dir = r.dir().reflect(&hr.normal())}
        else {refract_dir = r.dir().refract(&hr.normal(), ri)}

        return (Ray::new(hr.point(), refract_dir), WHITE);
    }
}
