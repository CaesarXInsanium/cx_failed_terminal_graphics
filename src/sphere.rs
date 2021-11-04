use crate::Vec3;
use crate::Color;

#[derive(Debug, Default, Clone)]
pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
    pub specular: f32
}
