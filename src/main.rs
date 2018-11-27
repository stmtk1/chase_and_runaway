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

struct Animal {
    x: f64,
    y: f64
}

struct Cat{
    x: f64,
    y: f64
}

struct Rat{
    x: f64,
    y: f64
}

trait AnimalTrait {
    fn as_animal(&self) -> Animal;
}

impl Animal{
    fn dist(a1: Animal, a2: Animal) -> f64 {
        (a1.x - a2.x) * (a1.x - a2.x) + (a1.y - a2.y) * (a1.y - a2.y)
    }
}

impl Cat{
    fn new(width: f64, height: f64) -> Cat{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * width;
        let y: f64 = rng.gen::<f64>() * height;
        Cat{ x: x, y: y }
    }
}

impl Rat{
    fn new(width: f64, height: f64) -> Rat {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * width;
        let y: f64 = rng.gen::<f64>() * height;
        Rat{ x: x, y: y }
    }
}

impl AnimalTrait for Cat {
    fn as_animal(&self) -> Animal {
        Animal {x: self.x, y: self.y }
    }
}

impl AnimalTrait for Rat {
    fn as_animal(&self) -> Animal {
        Animal {x: self.x, y: self.y }
    }
}

pub struct App {
    gl: GlGraphics,
    rotation: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;
        
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.width / 2.0, args.height / 2.0);

        self.gl.draw(args.viewport(), |c, gl|{
            clear(GREEN, gl);
            
            let transform = 
                c.transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;
    }
}

fn main(){
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
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
