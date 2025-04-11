use crate::material;
use crate::vec3::*;
use crate::utilities::*;
use crate::material::Material;

const NOT_HIT : HitRecord = HitRecord {has_hit:false,point:Vec3::new(0.0,0.0,0.0),normal:Vec3::new(0.0,0.0,0.0),t:0.0,front_face:false, material: Material::Void};

pub struct HitRecord {
    has_hit : bool,
    point : Vec3,
    normal : Vec3,
    t : f64,
    front_face : bool,
    material : Material
}

impl HitRecord {
    fn set_face_normal(&mut self, r : &Ray, outward_normal : &Vec3) {
        self.front_face = Vec3::dot(&r.dir(),outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal*(-1.0);
        }
    }

    pub const fn has_hit(&self) -> bool {self.has_hit}
    pub const fn front_face(&self) -> bool {self.front_face}
    pub const fn point(&self) -> Vec3 {self.point}
    pub const fn normal(&self) -> Vec3 {self.normal}  
    pub const fn material(&self) -> Material {self.material}
}

pub trait Hittable {
    fn hit(&self, r : &Ray, ray_t : Interval) -> HitRecord;    
}

pub struct Sphere {
    center : Vec3,
    radius : f64,
    material : Material
}

impl Sphere {
    pub const fn new(center : Vec3, radius : f64, material : Material) -> Sphere {
        Sphere {center, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r : &Ray, ray_t : Interval) -> HitRecord {
        let oc = self.center-r.origin();
        let a : f64 = r.dir().norm_squared();
        let h : f64 = Vec3::dot(&r.dir(), &oc);
        let c : f64 = oc.norm_squared()-self.radius*self.radius;
        let dis : f64 = h*h - a*c;
        
        if dis<0.0 {
            return NOT_HIT
        } 
        
        let dis_sqrt: f64 = f64::sqrt(dis);
        let mut root: f64 = (h - dis_sqrt) / a;
        if !ray_t.surrounds(root) {
            root = (h + dis_sqrt) / a;
            if !ray_t.surrounds(root) {
                return NOT_HIT;
            }
        }

        let mut hr : HitRecord = HitRecord { has_hit: true, point: r.at(root), normal: Vec3::new(0.0,0.0,0.0), t: root, front_face: false , material : self.material};
        hr.t = root;
        hr.point = r.at(hr.t);
        let outward_normal = (hr.point-self.center)/self.radius;
        hr.set_face_normal(r, &outward_normal);
        return hr;
    }
}


pub struct HittableList {
    objects : Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub const fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, obj : Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}   

impl Hittable for HittableList {
    fn hit(&self, r : &Ray, ray_t : Interval) -> HitRecord {
        let mut hr : HitRecord = NOT_HIT;
        let mut hit_anything : bool = false;
        let mut closest_so_far : f64 = ray_t.max();

        for i in 0..self.objects.len() {
            let obj_hr = self.objects[i].hit(r, Interval::new(ray_t.min(),closest_so_far));
            if obj_hr.has_hit {
                hit_anything = true;
                closest_so_far = obj_hr.t;
                hr = obj_hr
            }
        }

        hr.has_hit = hit_anything;
        return hr;
    }
}