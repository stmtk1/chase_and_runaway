extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

use rand::prelude::*;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::polygon;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone)]
struct Animal {
    x: f64,
    y: f64,
    velocity: f64,
    direction: f64,
}

#[derive(Clone)]
struct PVector {
    x: f64,
    y: f64,
}

impl PVector {
    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dist(&self, other: PVector) -> f64 {
        self.offset(other).len()
    }
    
    fn offset_x(self_x: f64, other_x: f64) -> f64 {
        let dist_x = other_x - self_x;
        // self_x < other_x => dist_x > 0
        if self_x < other_x && dist_x < WIDTH - dist_x {
            dist_x
        } else if self_x < other_x {
            dist_x - WIDTH
        } else if -WIDTH - dist_x < dist_x {
            dist_x
        } else {
            WIDTH + dist_x
        }
    }
    
    fn offset_y(self_y: f64, other_y: f64) -> f64 {
        let dist_y = other_y - self_y;
        
        if self_y < other_y && dist_y < HEIGHT - dist_y {
            dist_y
        } else if self_y < other_y {
            dist_y - HEIGHT
        } else if -HEIGHT - dist_y < dist_y {
            dist_y
        } else {
            HEIGHT + dist_y
        }
    }
    
    fn offset(&self, other: PVector) -> PVector {
        PVector {
            x: PVector::offset_x(self.x, other.x), 
            y: PVector::offset_y(self.y, other.y) 
        }
    }
    
    fn find_near(&self, others: Vec<PVector>, r: f64) -> Vec<PVector> {
        //println!("{}", others[0].(self));
        let mut ret: Vec<PVector>= Vec::with_capacity(others.len());
        for other in others {
            if self.dist(other.clone()) < r {
                ret.push(other);
            }
        }
        ret
    }

    fn add(&self, other: PVector) -> PVector {
        PVector{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    fn add_all(vectors: Vec<PVector>) -> PVector {
        let mut ret = PVector{ x: 0.0, y: 0.0 };
        for vec in vectors {
            ret = ret.add(vec);
        }
        ret
    }
    
    fn direction(&self) -> f64 {
            self.x.atan2(self.y)
    }
    
    /*
    
    fn normalize(&self) -> PVector {
        let size = self.len();
        PVector{
            x: self.x / size,
            y: self.y / size,
        }
    }
    
    fn mult(&self, scalar: f64) -> PVector {
        PVector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
    */
}

impl Animal{
    fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * WIDTH;
        let y: f64 = rng.gen::<f64>() * HEIGHT;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        Animal{ x: x, y: y, velocity: 0.5, direction: theta }
    }
    
    fn new2() -> Animal{
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        Animal{ x: WIDTH - 1.0, y: HEIGHT / 2.0, velocity: 0.5, direction: 0.0 }
    }
    
    fn new3() -> Animal{
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        Animal{ x: 1.0, y: HEIGHT / 2.0, velocity: 0.5, direction: theta }
    }
    
    fn offset(&self, other:Animal) -> PVector {
        let self_vec = PVector { x: self.x, y: self.y };
        let other_vec = PVector { x: other.x, y: other.y };
        self_vec.offset(other_vec)
    }
        
    
    fn dist(&self, other: Animal) -> f64 {
        self.offset(other).len()
    }
    
    fn move_self(&self) -> Animal {
        let mut new_x = self.x + self.velocity * self.direction.cos();
        let mut new_y = self.y + self.velocity * self.direction.sin();
        
        if new_x > WIDTH {
            new_x -= WIDTH;
        }
        
        if new_x < 0.0 {
            new_x += WIDTH;
        }
        
        if new_y > HEIGHT {
            new_y -= HEIGHT;
        }
        
        if new_y < 0.0 {
            new_y += HEIGHT;
        }
        
        Animal {
            x: new_x,
            y: new_y,
            velocity: self.velocity,
            direction: self.direction
        }
    }
    
    fn as_pvector(&self) -> PVector {
        PVector{
            x: self.x,
            y: self.y,
        }
    }
    
    fn to_pvectors(animals: Vec<Animal>) -> Vec<PVector> {
        let mut animal_vec: Vec<PVector> = Vec::with_capacity(animals.len());
        for animal in animals {
            animal_vec.push(animal.as_pvector());
        }
        animal_vec
    }
    
    fn chase(&self, preyers: Vec<Animal>) -> Animal {
        let near_preyer = self
            .as_pvector()
            .find_near(Animal::to_pvectors(preyers), 50.0);
        println!("{}", near_preyer.len());
        //self.direction = PVector::add_all(near_preyer).direction();
        Animal {
            x: self.x,
            y: self.y,
            velocity: self.velocity,
            direction: PVector::add_all(near_preyer).direction(),
        }
    }
}

// #[derive(Clone)]
pub struct App {
    gl: GlGraphics,
    cats: Vec<Animal>,
    rats: Vec<Animal>,
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;
        
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];

        let square = rectangle::square(0.0, 0.0, 5.0);
        let cats = &self.cats;
        let rats = &self.rats;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(GREEN, gl);
            
            for cat in cats{
                let transform = c.transform
                    .trans(cat.x, cat.y);
                rectangle(RED, square, transform, gl);
                //polygon(RED, &TRIANGLE, transform, gl);
            }
            
            for rat in rats {
                let transform = c.transform
                    .trans(rat.x, rat.y);
                rectangle(BLUE, square, transform, gl);
                //polygon(BLUE, &TRIANGLE, transform, gl);
            }
        });
    }

    fn update(&mut self) {
        let cats = &self.cats.clone();
        let mut new_cats: Vec<Animal> = Vec::with_capacity(cats.len());
        for cat in cats {
            new_cats.push(cat.chase(self.rats.clone()).move_self());
        }
        println!("{}", new_cats[0].direction);
        self.cats = new_cats;
        let rats = &self.rats.clone();
        
        let mut new_rats: Vec<Animal> = Vec::with_capacity(rats.len());
        for rat in rats {
            new_rats.push(rat.move_self());
        }
        self.rats = new_rats;
    }
}

fn main(){
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [WIDTH as u32, HEIGHT as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut cats: Vec<Animal> = Vec::with_capacity(100);
    for _ in 0..1 {
        cats.push(Animal::new3());
    }
    
    let mut rats: Vec<Animal> = Vec::with_capacity(100);
    for _ in 0..100 {
        rats.push(Animal::new2());
    }
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cats: cats, 
        rats: rats, 
    };
    
    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args(){ 
            app.render(&r);
        }
        
        if let Some(u) = e.update_args() {
            app.update();
        }
    }
     //println!("{}", Animal::new2().offset(Animal::new3()).x);
}

#[cfg(test)]
mod tests{
    use super::Animal;
    use super::PVector;
    
    #[test]
    fn distant_calculation_test(){
        let a1 = Animal { x: 0.0, y: 0.0, velocity: 0.0, direction: 0.0 };
        let a2 = Animal { x: 3.0, y: 4.0, velocity: 0.0, direction: 0.0 };
        assert_eq!(a1.dist(a2), 5.0);
    }
    
    #[test]
    fn find_near_test(){
        let v1 = PVector { x: 0.0, y: 0.0 };
        let v2 = PVector { x: 1.0, y: 2.0 };
        let v3 = PVector { x: 3.0, y: 3.0 };
        let mut vec: Vec<PVector> = Vec::with_capacity(100);
        for _ in 0..90 {
            vec.push(v2.clone());
        }
        
        for _ in 0..10 {
            vec.push(v3.clone());
        }
        assert_eq!(v1.find_near(vec, 3.0).len(), 90);
    }
    
    #[test]
    fn find_near_test2(){
        let v1 = PVector { x: 0.0, y: 0.0 };
        let v2 = PVector { x: -1.0, y: 2.0 };
        let v3 = PVector { x: -3.0, y: -3.0 };
        let mut vec: Vec<PVector> = Vec::with_capacity(100);
        for _ in 0..90 {
            vec.push(v2.clone());
        }
        
        for _ in 0..10 {
            vec.push(v3.clone());
        }
        assert_eq!(v1.find_near(vec, 3.0).len(), 90);
    }

    #[test]
    fn add_test(){
        let v1 = PVector { x: 1.0, y: 2.0 };
        let v2 = PVector { x: 3.0, y: 3.0 };
        let v3 = v1.add(v2);
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 5.0);
    }
    
    #[test]
    fn add_all_test(){
        let mut vec:Vec<PVector> = Vec::with_capacity(5);
        vec.push(PVector { x: 1.0, y: 1.0 });
        vec.push(PVector { x: 2.0, y: 1.0 });
        vec.push(PVector { x: 3.0, y: 2.0 });
        vec.push(PVector { x: 4.0, y: 3.0 });
        vec.push(PVector { x: 5.0, y: 5.0 });
        let ret = PVector::add_all(vec);
        assert_eq!(ret.x, 15.0);
        assert_eq!(ret.y, 12.0);
    }
    
    /*
    #[test]
    fn mult_test(){
        let v1 = PVector { x: 1.0, y: 2.0 };
        let v2 = v1.mult(3.0);
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 6.0);
    }
    */
}
