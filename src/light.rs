use crate::{Vec3};
#[derive(Debug, Clone)]
pub enum Light{
    Ambient(AmbientLight),
    Point(PointLight),
    Directional(DirectionalLight)
}
impl Light{
    pub fn compute_lighting(&self, lights: &Vec<Self>, p: &Vec3, n: &Vec3)->f32{
        let i = 0.0;
        for light in lights.iter(){}
        match self{
            Light::Ambient(light) => {
                i + light.intensity
            },
            Light::Point(light) => {
                let l = light.point.sub(p);
                let n_dot_l = n.dot(&l);
                if n_dot_l > 0.0{
                    i + light.intensity * (n_dot_l / (n.norm() / l.norm())) 
                }else{
                    i
                }
            },
            Light::Directional(light) => {
                let l = &light.direction;
                let n_dot_l = n.dot(&l);
                if n_dot_l > 0.0{
                    i + light.intensity * (n_dot_l / (n.norm() / l.norm())) 
                }else{
                    i
                }
            },
        }
    }
}
impl Default for Light{
    fn default() -> Self {
        Light::Ambient(AmbientLight::default())
    }
}
#[derive(Debug, Clone, Default)]
pub struct AmbientLight {
    pub intensity: f32
}
#[derive(Debug, Clone, Default)]
pub struct PointLight {
    pub intensity: f32,
    pub point: Vec3
}
#[derive(Debug, Clone, Default)]
pub struct DirectionalLight {
    pub intensity: f32,
    pub direction: Vec3
}