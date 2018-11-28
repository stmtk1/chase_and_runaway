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

#[derive(Clone)]
struct Animal {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone)]
struct Cat{
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone)]
struct Rat{
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

trait AnimalTrait {
    fn as_animal(&self) -> Animal;
}

impl Animal{
    fn dist(a1: Animal, a2: Animal) -> f64 {
        (a1.x - a2.x) * (a1.x - a2.x) + (a1.y - a2.y) * (a1.y - a2.y)
    }
    
    fn move_self(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
    
    fn as_cat(&self) -> Cat {
        Cat{x: self.x, y: self.y, vx: self.vx, vy: self.vy }
    }
    
    fn as_rat(&self) -> Rat {
        Rat{x: self.x, y: self.y, vx: self.vx, vy: self.vy }
    }
}

impl Cat{
    fn new(width: f64, height: f64) -> Cat{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * width;
        let y: f64 = rng.gen::<f64>() * height;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let vx: f64 = theta.cos();
        let vy: f64 = theta.sin();
        Cat{ x: x, y: y, vx: vx, vy: vy }
    }
}

impl Rat{
    fn new(width: f64, height: f64) -> Rat {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * width;
        let y: f64 = rng.gen::<f64>() * height;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let vx: f64 = theta.cos();
        let vy: f64 = theta.sin();
        Rat{ x: x, y: y, vx: vx, vy: vy }
    }
}

impl AnimalTrait for Cat {
    fn as_animal(&self) -> Animal {
        Animal {x: self.x, y: self.y, vx: self.vx, vy: self.vy }
    }
}

impl AnimalTrait for Rat {
    fn as_animal(&self) -> Animal {
        Animal {x: self.x, y: self.y, vx: self.vx, vy: self.vy }
    }
}

// #[derive(Clone)]
pub struct App {
    gl: GlGraphics,
    cats: Vec<Cat>,
    rats: Vec<Rat>,
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;
        
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];

        let square = rectangle::square(0.0, 0.0, 5.0);
        let (x, y) = (args.width / 2.0, args.height / 2.0);
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
            
            for rat in rats{
                let transform = c.transform
                    .trans(rat.x, rat.y);
                rectangle(BLUE, square, transform, gl);
                //polygon(BLUE, &TRIANGLE, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let cats = &self.cats.clone();
        let mut new_cats: Vec<Cat> = Vec::with_capacity(cats.len());
        for cat in cats {
            let mut animal = cat.as_animal();
            animal.move_self();
            new_cats.push(animal.as_cat());
        }
        self.cats = new_cats;
        
        let rats = &self.rats.clone();
        let mut new_rats: Vec<Rat> = Vec::with_capacity(rats.len());
        for rat in rats {
            let mut animal = rat.as_animal();
            animal.move_self();
            new_rats.push(animal.as_rat());
        }
        self.rats = new_rats;
    }
}

fn main(){
    let opengl = OpenGL::V3_2;
    let width: f64 = 640.0;
    let height: f64 = 480.0;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [width as u32, height as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut cats: Vec<Cat> = Vec::with_capacity(100);
    for _ in 0..100 {
        cats.push(Cat::new(width, height));
    }
    
    let mut rats: Vec<Rat> = Vec::with_capacity(100);
    for _ in 0..100 {
        rats.push(Rat::new(width, height));
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
            app.update(&u);
        }
    }
}
