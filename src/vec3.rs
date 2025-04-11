use crate::utilities::UNIT;
#[derive(Clone, Copy)]
pub struct Vec3 {
    x : f64,
    y : f64,
    z : f64
}

impl Vec3 {
    pub const fn new(x : f64, y : f64, z : f64) -> Self {
        Vec3 {x,y,z}
    }
    
    pub fn random_vec() -> Self {
        return Vec3 {x:rand::random::<f64>(),y:rand::random::<f64>(),z:rand::random::<f64>()}
    }

    pub fn random_outwards(normal : &Vec3) -> Vec3 {
        let v = Vec3::random_vec();
        if Vec3::dot(&v,normal)>0.0 {return v}
        else {return v*(-1.0)}
    }

    pub fn x(&self) -> f64 {self.x}
    pub fn y(&self) -> f64 {self.y}
    pub fn z(&self) -> f64 {self.z}

    pub fn norm_squared(&self) -> f64 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    pub fn norm(&self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    pub fn normalized(&self) -> Vec3 {
        self.clone()/self.norm()
    }

    pub const fn dot(u : &Vec3, v : &Vec3) -> f64 {
        u.x*v.x+u.y*v.y+u.z*v.z
    }

    pub const fn cross(u : &Vec3, v : &Vec3) -> Vec3 {
        Vec3 {
            x : u.y*v.z-u.z*u.y,
            y : u.z*v.x-u.x*v.z,
            z : u.x*v.y-u.y*v.x 
        }
    }

    pub fn reflect(&self, normal : &Vec3) -> Vec3 {
        return *self-(*normal)*2.0*Vec3::dot(self,normal);
    }

    pub fn refract(&self, normal : &Vec3, refractive_coefficient : f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&(*self*(-1.0)),normal),1.0);
        let r_out_perp = (*self+*normal*cos_theta)*refractive_coefficient;
        let r_out_para = *normal*(-f64::sqrt(f64::abs(1.0-r_out_perp.norm_squared())));
        return r_out_perp+r_out_para;
    }

    pub fn near_zero(&self) -> bool {
        let s:f64 = 1e-8;
        return f64::abs(self.x)<s && f64::abs(self.y)<s && f64::abs(self.z)<s;
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

#[derive(Clone, Copy)]
pub struct Ray {
    origin : Vec3,
    dir : Vec3
}

impl Ray {
    pub const fn new(origin : Vec3, direction : Vec3) -> Self {
        Ray {origin : origin, dir : direction}
    }

    pub fn at(&self, t : f64) -> Vec3 {
        self.origin+self.dir*t
    }

    pub const fn origin(&self) -> Vec3 {self.origin}
    pub const fn dir(&self) -> Vec3 {self.dir}
}


pub type Color = Vec3;
impl Color {
    pub fn r(&self) -> i16 {
        (255.999*UNIT.clamp(self.x)) as i16
    }
    pub fn g(&self) -> i16 {
        (255.999*UNIT.clamp(self.y)) as i16
    }
    pub fn b(&self) -> i16 {
        (255.999*UNIT.clamp(self.z)) as i16
    }
}