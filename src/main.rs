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
        self.x * self.x + self.y * self.y
    }

    fn offset_x(self_x: f64, other_x: f64) -> f64 {
        let dist_x = other_x - self_x;
        // self_x < other_x => dist_x > 0
        if self_x < other_x && dist_x < WIDTH - dist_x {
            dist_x
        } else if self_x < other_x {
            dist_x - WIDTH
        } else if WIDTH - dist_x < dist_x {
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
        } else if HEIGHT - dist_y < dist_y {
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
}

impl Animal{
    fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * WIDTH;
        let y: f64 = rng.gen::<f64>() * HEIGHT;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        Animal{ x: x, y: y, velocity: 0.5, direction: theta }
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
    
    
    
    
    fn find_near(&self, others: Vec<Animal>){
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
            new_cats.push(cat.move_self());
        }
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
    for _ in 0..100 {
        cats.push(Animal::new());
    }
    
    let mut rats: Vec<Animal> = Vec::with_capacity(100);
    for _ in 0..100 {
        rats.push(Animal::new());
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
}

#[cfg(test)]
mod tests{
    use super::Animal;
    
    #[test]
    fn distant_calculation_test(){
        let a1 = Animal { x: 0.0, y: 0.0, velocity: 0.0, direction: 0.0 };
        let a2 = Animal { x: 3.0, y: 4.0, velocity: 0.0, direction: 0.0 };
        assert_eq!(a1.dist(a2), 25.0);
    }
}
