use rand::prelude::*;
use pvector::PVector;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone)]
pub struct Animal {
    pub x: f64,
    pub y: f64,
    pub velocity: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Animal{
    pub fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * WIDTH;
        let y: f64 = rng.gen::<f64>() * HEIGHT;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 0.5;
        Animal{ x: x, y: y, velocity: velocity, vx: theta.cos() * velocity, vy: theta.sin() * velocity }
    }
    
    pub fn new2() -> Animal{
        //let rng = rand::thread_rng();
        let theta: f64 = 0.0;//rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 0.5;
        Animal{ x: WIDTH - 1.0, y: HEIGHT / 2.0, velocity: velocity, vx: theta.cos() * velocity, vy: theta.sin() * velocity }
    }
    
    pub fn new3() -> Animal{
        //let rng = rand::thread_rng();
        let theta: f64 = 0.0; //rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 0.5;
        Animal{ x: 1.0, y: HEIGHT / 2.0, velocity: velocity, vx: theta.cos() * velocity, vy: theta.sin() * velocity }
    }
    
    pub fn offset(&self, other:Animal) -> PVector {
        let self_vec = PVector { x: self.x, y: self.y };
        let other_vec = PVector { x: other.x, y: other.y };
        self_vec.offset(other_vec)
    }
        
    
    pub fn dist(&self, other: Animal) -> f64 {
        self.offset(other).len()
    }
    
    pub fn move_self(&self) -> Animal {
        let mut new_x = self.x + self.vx;
        let mut new_y = self.y + self.vy;
        
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
            vx: self.vx,
            vy: self.vy
        }
    }
    
    pub fn as_pvector(&self) -> PVector {
        PVector{
            x: self.x,
            y: self.y,
        }
    }
    
    pub fn to_pvectors(animals: Vec<Animal>) -> Vec<PVector> {
        let mut animal_vec: Vec<PVector> = Vec::with_capacity(animals.len());
        for animal in animals {
            animal_vec.push(animal.as_pvector());
        }
        animal_vec
    }
    
    pub fn chase(&self, preyers: Vec<Animal>) -> Animal {
        let near_preyer = self
            .as_pvector()
            .find_near(Animal::to_pvectors(preyers), 10.0);
        if near_preyer.len() <= 0 {
            return self.clone();
        }
        let a = PVector::add_all(near_preyer.clone()).normalize().mult(self.velocity);
        println!("{}, {}", a.x, a.y);
        let next_velocity = PVector::add_all(near_preyer)
                                    .normalize()
                                    .mult(self.velocity);
        //println!("{}, {}", next_velocity.x, next_velocity.y);
        Animal {
            x: self.x,
            y: self.y,
            velocity: self.velocity,
            vx: next_velocity.x,
            vy: next_velocity.y
            //direction: PVector::add_all(near_preyer).direction(),
        }
    }
}
