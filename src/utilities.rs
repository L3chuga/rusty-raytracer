use std::f64::{consts::PI, INFINITY};
use crate::vec3::*;

pub const EMPTY : Interval = Interval::new(INFINITY,-INFINITY);
pub const WHOLE : Interval = Interval::new(-INFINITY,INFINITY);
pub const UNIT : Interval = Interval::new(0.0, 1.0);

pub const ZERO : Vec3 = Vec3::new(0.0, 0.0, 0.0);

pub const WHITE : Color = Color::new(Vec3::new(1.0,1.0,1.0));
pub const BLACK : Color = Color::new(Vec3::new(0.0,0.0,0.0));
pub const SKY_BLUE : Color = Color::new(Vec3::new(0.5,0.7,1.0));
pub const RED : Color = Color::new(Vec3::new(1.0,0.0,0.0));
pub const BLUE : Color = Color::new(Vec3::new(0.0,0.0,1.0));
pub const GRAY : Color = Color::new(Vec3::new(0.5,0.5,0.5));


pub fn linear_to_gamma(x : f64) -> f64 {
    if x > 0.0 {return f64::sqrt(x)}
    else {return 0.0};
}
pub struct Interval {
    min : f64,
    max : f64
}

impl Interval {
    pub const fn new(a : f64, b : f64) -> Self {
        Interval {min:a,max:b}
    }

    pub const fn max(&self) -> f64 {self.max}
    pub const fn min(&self) -> f64 {self.min} 
    pub const fn size(&self) -> f64 {self.max-self.min}
    pub const fn contains(&self, x : f64) -> bool {self.min<=x && x<=self.max}
    pub const fn surrounds(&self, x : f64) -> bool {self.min<x && x<self.max}
    pub const fn clamp(&self, x : f64) -> f64 {
        if x<self.min {return self.min}
        if self.max<x {return self.max}
        return x
    }
}

pub const fn degrees_to_radians(degrees : f64) -> f64 {
    return degrees * PI/180.0;
}
