use colorful::{RGB, Colorful};
mod sphere;
use sphere::{Sphere};
use image::*;


#[derive(Debug , Clone)]
pub struct Color(u8,u8,u8);

impl Color{
    pub const WHITE: Color = Color(255,255,255);
    pub fn new() -> Color{
        Color::default()
    }
    pub fn with_intensity(&self, k: u8) -> Color{
        let x = match self.0.checked_mul(k){
            Some(e) => e,
            None    => u8::MAX
        };
         let y = match self.1.checked_mul(k){
            Some(e) => e,
            None    => u8::MAX
        };
         let z = match self.2.checked_mul(k){
            Some(e) => e,
            None    => u8::MAX
        };
       Color(x,y,z) 
    }

    pub fn to_ascii(&self)->colorful::core::color_string::CString{
       //"#".red()
        " ".bg_color(RGB::new(self.0, self.1, self.2)) 
    }

}

impl Default for Color{
    fn default()->Color{
        Color(0,0,0)
    }
}

#[derive(Debug, Default)]
pub struct Vec2(f32,f32);

impl Vec2{
    pub fn norm(&self) -> f32{
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }
}

#[derive(Debug,Default,Clone)]
/// x and z are horizontal and y is vertical
pub struct Vec3(f32,f32,f32);

impl Vec3{
    pub fn norm(&self)->f32{
        (self.0.powf(2.0) + self.1.powf(2.0) + self.2.powf(2.0)).sqrt()
    }
    pub fn distance(&self, other: &Vec3) -> f32{
        let d = (
            (self.0 - other.0).powi(2) +    // powi is method for raising float to int. rust docs say its faster
            (self.1 - other.1).powi(2) +
            (self.2 - other.2).powi(2)
            ).sqrt();
        d
    }
    pub fn sub(&self, other: &Vec3) -> Vec3{
       Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2) 
    }
    pub fn dot(&self, other: &Vec3) -> f32{
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }
}

#[derive(Debug, Default)]
pub struct Camera{
    pos: Vec3,
}

#[derive(Debug)]
pub struct Frame{
    distance: f32,
    height: f32,
    width: f32
}
impl Frame {
    fn new()->Frame{
        Frame{
            distance: 1.0,
            height: 1.0,
            width: 1.0
        }
    }

    fn new_accurate(dis: f32, height: f32, width: f32)->Frame{
        Frame{
            distance: dis,
            height: height,
            width: width
        }
    }
    /// Allows us to convert Canvas coords to viewport coords
    fn canvas_to_viewport_coords(&self,canvas_coord: &Vec2, canvas: &Canvas) -> Vec3{
        let v_x = canvas_coord.0 * (self.width / canvas.width as f32);
        let v_y = canvas_coord.1 * (self.height / canvas.height as f32);
        let v_z = self.distance;
        Vec3(v_x, v_y, v_z)

    }
}
impl Default for Frame{
    fn default()->Frame{
        Frame::new()
    }
}

#[derive(Debug)]
/// Represents the screen
pub struct Canvas{
    buffer: Vec<Vec<Color>>,
    image: image::DynamicImage,
    height: usize,
    width: usize,
    camera: Camera,
    viewport: Frame
}

impl Canvas{
    pub fn new(x: usize, y: usize, cam: Camera, view: Frame) -> Canvas{
        let mut v = Vec::with_capacity(y);
        for _ in 0..y{
            let mut row = Vec::with_capacity(x);
            for _ in 0..x{
                row.push(Color::default());
            }
            v.push(row);
        }
        let mut im = image::DynamicImage::new_rgb8(x as u32, y as u32);
        Canvas{
            buffer: v,
            image: im,
            height: y,
            width: x,
            camera: cam,
            viewport: view
        }
    }

    /// Implements raytracing. with reverse photon tracking.
    pub fn draw(&mut self, spheres: &Vec<Sphere>){
        let heighthalf = self.height as f32 / 2.0;
        let widthhalf = self.width as f32 / 2.0;
        println!("hh: {:?}\twh: {:?}", &heighthalf, &widthhalf);

       for x in (-(self.width as f32 / 2.0) as usize)..((self.width as f32/ 2.0) as usize){
           println!("x: {:?}", x);
        for y in (-(self.height as f32 / 2.0) as usize)..((self.height
                                                           as f32/ 2.0) as usize){
           // point in viewport corresponding to Canvas , vector from camera to point in viewport
           // that corresponds to canvas coords
           let d: Vec3 = self.viewport.canvas_to_viewport_coords(
               &Vec2(x as f32,y as f32),
               self 
               );
           let o = &self.camera.pos;
           let color = trace_ray(spheres, o, &d, 1.0, f32::INFINITY);
           self.put_pixel(x.clone(),y.clone(),color.clone());

        }
       } 
    }

    pub fn flush(&self){
        // for row in self.buffer.iter(){
        //     // let mut line= colorful::core::color_string::CString::new("");
        //     for color in row.iter(){
        //         let s = color.to_ascii(); 
        //         print!("{}", s);
        //     }
        //     println!("")
        // }
         match self.image.save(format!("images/test{:?}image.png", std::time::SystemTime::now())){
             Ok(_)  => (),
             Err(_) => (),
         }
    }
    /// takes in canvas coordinates to place color
    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color){
        let cx = (1.0 / x as f32) * (self.viewport.width as f32 / self.width as f32);
        let cy = (1.0 / y as f32) * (self.viewport.height as f32/ self.height as f32);

       self.image.put_pixel(cx as u32, cy as u32, image::Pixel::from_channels(color.0, color.1, color.2, 1));
    }
    /// convert system/human coordinates to Canvas/screen coordinates
    fn _convert(&self, c: Vec2)->(Option<usize>, Option<usize>){
        ((c.0 as usize).checked_sub(self.width / 2), (c.1 as usize).checked_sub(self.height / 2)) 
    }
}


fn main() {
    let frame = Frame::new_accurate(1.0, 1.0, 1.0);
    let camera = Camera{
        pos: Vec3(0.0, 0.0, -1.0)
    };
    let sphere1 = Sphere{center: Vec3(0.0,-1.0,3.0),radius:1.0,color:Color(255,0,0)};
    let sphere2 = Sphere{center: Vec3(2.0,0.0,4.0),radius:1.0,color:Color(0,0,255)};
    let sphere3 = Sphere{center: Vec3(-2.0,0.0,4.0),radius:1.0,color:Color(0,255,0)};
    let v = vec![sphere1, sphere2, sphere3];
    let mut canvas = Canvas::new(800,600, camera, frame);

    for i in -1..=1{
        println!("i: {:?}", &i);

        let newcamera = Camera{pos: Vec3(i as f32, 0.0,0.0)};
        canvas.camera = newcamera;
        canvas.draw(&v);
        canvas.flush();
    }
}

fn intersect_ray_sphere(o: &Vec3, d: &Vec3, sphere: &Sphere)->(f32,f32){
    let r = sphere.radius;
    let co = o.sub(&sphere.center);

    let a = d.dot(d);
    let b = 2.0*co.dot(d);
    let c = co.dot(&co) - r*r;

    let discriminant = b*b - 4.0 * a * c;
    if discriminant < 0.0{
        return (f32::INFINITY, f32::INFINITY)
    }
    let t1 = ((0.0-b) + discriminant.sqrt()) / (2.0 * a);
    let t2 = ((0.0-b) - discriminant.sqrt()) / (2.0 * a);
    (t1,t2)
}

fn trace_ray(spheres: &Vec<Sphere>, o: &Vec3, d: &Vec3, t_min: f32, _t_max: f32)->Color{
    let mut closest_t = f32::INFINITY;
    let mut closest_sphere: Option<&Sphere> = None;
    for sphere in spheres.iter(){
        let (t1, t2) = intersect_ray_sphere(o, d, sphere);
        if t1 > t_min && t1 < closest_t{
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if t2 > t_min && t2 < closest_t{
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere{
        Some(x)     => x.color.clone(),
        None        => Color(0, 0, 0)
    }
}
