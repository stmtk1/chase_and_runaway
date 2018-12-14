use rand::prelude::*;
use pvector::PVector;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone)]
pub struct Animal {
    pub x: f64,
    pub y: f64,
    velocity: f64,
    vx: f64,
    vy: f64,
    //chasing: &Animal,
    //chased: Vec<&Animal>,
}

impl Animal{
    pub fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * WIDTH;
        let y: f64 = rng.gen::<f64>() * HEIGHT;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 0.5;
        Animal{ 
            x: x, 
            y: y, 
            velocity: velocity, 
            vx: theta.cos() * velocity, 
            vy: theta.sin() * velocity,
        }
    }
    
    pub fn offset(&self, other:Animal) -> PVector {
        let self_vec = PVector { x: self.x, y: self.y };
        let other_vec = PVector { x: other.x, y: other.y };
        self_vec.offset(other_vec)
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
            vy: self.vy,
        }
    }
    
    fn collect_near_pvectors(&self, animals: Vec<Animal>) -> Vec<Animal> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self.clone(), 10.0))
            .collect()
    }
    
    fn calculate_direction(&self, animals: Vec<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.offset(self.clone()))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    pub fn chase(&self, preyers: Vec<Animal>) -> Animal {
        let near_preyer = self.collect_near_pvectors(preyers);
        
        if near_preyer.len() <= 0 {
            return self.clone();
        }
        let next_velocity = self
            .calculate_direction(near_preyer)
            .mult(self.velocity * -1.0);
        
        self.apply_velocity(next_velocity)
    }
    
    pub fn run_away(&self, preyers: Vec<Animal>) -> Animal {
        let near_preyer = self.collect_near_pvectors(preyers);
        
        if near_preyer.len() <= 0 {
            return self.clone();
        }
        let next_velocity = self
            .calculate_direction(near_preyer)
            .mult(self.velocity);
        
        self.apply_velocity(next_velocity)
    }
    
    fn apply_velocity(&self, pvector: PVector) -> Animal {
        let mut ret = self.clone();
        ret.vx = pvector.x;
        ret.vy = pvector.y;
        ret
    }
    
    pub fn eat(&self, rats: Vec<Animal>) -> Vec<Animal> {
     rats
            .into_iter()
            .filter(|rat| !self.is_within(rat.clone(), 1.0) )
            .collect()
    }
    
    pub fn is_within(&self, other: Animal, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
}
