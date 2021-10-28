

#[derive(Debug, Default)]
pub struct Color(u8,u8,u8);

impl Color{
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
}

#[derive(Debug)]
pub struct Coord2D(f32,f32);

#[derive(Debug,Default)]
/// x and z are horizontal and y is vertical
pub struct Coord3D(f32,f32,f32);

#[derive(Debug, Default)]
pub struct Canvas{
    data: Vec<Vec<Color>>,
    height: usize,
    width: usize
}

impl Canvas{
    pub fn new(x: usize, y: usize) -> Canvas{
       let mut v = Vec::with_capacity(x);
       for _ in x..y{
        v.push(Vec::<Color>::with_capacity(y));
       }
       Canvas{
        data: v,
        height: y,
        width: x,
        }
    }

    pub fn put_pixel(&self, c: Coord2D, color: Color){
       let pixel = self.convert(c);
       let pixel_Coord2D = (
           match pixel.0 {
            Some(x) => x,
            None => todo!("implement if point will or will not be projected or shown")
           },
           match pixel.1 {
            Some(x) => x,
            None => todo!("implement if point will or will not be projected or shown")
           }
           );

       todo!()
    }
    fn convert(&self, c: Coord2D)->(Option<usize>, Option<usize>){
        //(
        //    c.0 as usize - (self.width / 2),     //convert human Coord2D to canvas Coord2D
        //   c.1 as usize - (self.height / 2)     
        //)
        ((c.0 as usize).checked_sub(self.width / 2), (c.1 as usize).checked_sub(self.height / 2)) 
    }
}
fn main() {
   let c = Canvas::new(100,100); 
   println!("Canvas info: {:?}", c);
   let color = Color::new();
   println!("Color info {:?}", color);
}
