use crate::RGB_MAX;

#[derive(Clone, Copy)]
pub struct Vec3 {
    x : f64,
    y : f64,
    z : f64
}

impl Vec3 {
    pub const fn new(x : f64, y : f64, z : f64) -> Vec3 {
        Vec3 {x,y,z}
    }

    pub fn x(self) -> f64 {self.x}
    pub fn y(self) -> f64 {self.y}
    pub fn z(self) -> f64 {self.z}

    pub fn norm_squared(self) -> f64 {
        self.x*self.x+self.y*self.y+self.z*self.z
    }

    pub fn norm(self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    pub fn normalized(self) -> Vec3 {
        self.clone()/self.norm()
    }

    pub const fn dot(u : Vec3, v : Vec3) -> f64 {
        u.x*v.x+u.y*v.y+u.z*v.z
    }

    pub const fn cross(u : Vec3, v : Vec3) -> Vec3 {
        Vec3 {
            x : u.y*v.z-u.z*u.y,
            y : u.z*v.x-u.x*v.z,
            z : u.x*v.y-u.y*v.x 
        }
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

    pub fn at(self, t : f64) -> Vec3 {
        self.origin+self.dir*t
    }

    pub const fn origin(self) -> Vec3 {self.origin}
    pub const fn dir(self) -> Vec3 {self.dir}
}

pub struct Color {
    values : Vec3
}

fn truncate_range(x : i16) -> i16 {
    i16::max(i16::min(x,RGB_MAX as i16),0)
}

impl Color {
    pub const fn new(v : Vec3) -> Self {
        Color {values : v}
    }
    pub fn r(&self) -> i16 {
        truncate_range((255.999*self.values.x()) as i16)
    }
    pub fn g(&self) -> i16 {
        truncate_range((255.999*self.values.y()) as i16)
    }
    pub fn b(&self) -> i16 {
        truncate_range((255.999*self.values.z()) as i16)
    }

    pub fn values(self) -> Vec3 {self.values}
}