extern crate piston_window;
extern crate rand;

use rand::prelude::*;
use piston_window::*;

fn main(){
    for _ in 0..100{
        let width: f64 = 100.0;
        let height: f64 = 100.0;
        let cat = Cat::new(width, height);
        let rat = Rat::new(width, height);
        println!("{}", Animal::dist(cat.as_animal(), rat.as_animal()));
        World::new(width, height);
    }
}

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

struct World{
    window: PistonWindow,
    cats: Vec<Cat>,
    rats: Vec<Rat>,
    height: f64,
    width: f64
}

impl World {
    fn new(width: f64, height: f64) -> World {
        World {
            window: World::create_window(width, height),
            cats: vec![],
            rats: vec![],
            height: height,
            width: width
        }
    }

    fn create_window(width: f64, height: f64) -> PistonWindow {
        WindowSettings::new("chase and run away", [width as u32, height as u32])
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
    
    fn clear_window(window: PistonWindow){
    }
}
