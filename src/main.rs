extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

mod animal;
mod pvector;
mod app;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
//use graphics::polygon;
use animal::Animal;
use app::App;


const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

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
        
        if let Some(_) = e.update_args() {
            app.update();
        }
    }
}

#[cfg(test)]
mod tests{
    use super::Animal;
    use super::PVector;
    
    #[test]
    fn distant_calculation_test(){
        let a1 = Animal { x: 0.0, y: 0.0, velocity: 0.0, vx: 0.0, vy: 0.0 };
        let a2 = Animal { x: 3.0, y: 4.0, velocity: 0.0, vx: 0.0, vy: 0.0 };
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
    
    #[test]
    fn mult_test(){
        let v1 = PVector { x: 1.0, y: 2.0 };
        let v2 = v1.mult(3.0);
        assert_eq!(v2.x, 3.0);
        assert_eq!(v2.y, 6.0);
    }

    #[test]
    fn chase_test(){
        let cat = Animal{ x: 0.0, y: 0.0, velocity: 1.0, vx: 0.0, vy: 0.0 };
        let rat = Animal{ x: 1.0, y: 0.0, velocity: 0.0, vx: 0.0, vy: 0.0 };
        //let after_chase = cat.chase(vec![Animal]);
        //assert_eq!(v2.x, 3.0);
        //assert_eq!(v2.y, 6.0);
    }
}
