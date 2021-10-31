use crate::Vec3;
use crate::Camera;
use crate::Color;

#[derive(Debug, Default, Clone)]
pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub color: Color
}

#[derive(Debug, Default)]
pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray{
    pub fn from_point_vec(point: &Vec3, dirc: &Vec3) -> Ray{
        Ray{
            origin: point.clone(),
            direction: dirc.clone()
        }
    }
    /// Calculates points of T where ray crosses sphere
    fn intersect_sphere(&self, sphere: &Sphere, camera: &Camera) -> Option<(f32, f32)> {
        println!("Inter Sphr: {:?}\t D: {:?}", &sphere, &self.direction);
        let r = sphere.radius;
        let co = sphere.center.sub(&camera.pos);    // ray CO is vector from sphere center to camera position
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * co.dot(&self.direction);
        let c = co.dot(&co) - r*r;
        //println!("a: {:?}\tb: {:?}\tc: {:?}", &a, &b, &c);

        let discriminant = b*b - 4.0*a*c;
    
        if discriminant < 0.0{
            return None
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        //println!("t1: {:?}\tt2: {:?}", &t1, &t2);
        Some((t1,t2))
    }

    pub fn trace_ray(&self,camera: &Camera, spheres: &Vec<Sphere>, t_min: f32, _t_max: f32) -> Color{
        let mut closest_t = f32::INFINITY;
        let mut closest_shere: Option<&Sphere> = None; 
        for sphere in spheres.iter(){
            let (t1,t2) = match self.intersect_sphere(sphere, camera){
                Some((x,y)) => {
                    (x,y)},
                None        => {
                    (f32::INFINITY, f32::INFINITY)
                }
            };
            if t1 >= t_min && t1 < closest_t{
                closest_t = t1;
                closest_shere = Some(sphere);
                //println!("Found closest_shere");
            }
           if  t2 >= t_min && t2 < closest_t {
                closest_t = t2;
                closest_shere = Some(sphere);
                //println!("Found closest_shere");
            }
        }
        let color = match closest_shere{
                Some(x)     => {
                    x.color.clone()
                }
                ,
                None        => Color::default()
        };
        color
    }
}
