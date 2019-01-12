use rand::prelude::*;
use pvector::PVector;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone)]
pub struct Animal {
    pub x: f64,
    pub y: f64,
    velocity: f64,
    pub vx: f64,
    pub vy: f64,
}

impl Animal{
    pub fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * WIDTH;
        let y: f64 = rng.gen::<f64>() * HEIGHT;
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 1.0;
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
    
    fn collect_near_pvectors(&self, animals: Vec<Animal>, radious: f64) -> Vec<Animal> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self.clone(), radious))
            .collect()
    }
    
    fn calculate_direction(&self, animals: Vec<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.offset(self.clone()))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    pub fn chase(&self, rats: Vec<Animal>, cats: Vec<Animal>) -> Animal {
        let next_velocity = self
            .as_velocity()
            .add(self.chase_vector(rats.clone()))
            .add(self.separate_same(cats.clone()))
            .add(self.align(cats.clone()))
            .add(self.cohension(cats.clone()))
            .normalize()
            .mult(self.velocity);
        self.apply_velocity(next_velocity)
    }
    
    fn chase_vector(&self, preyers: Vec<Animal>) -> PVector {
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
            .mult(-1.0)
    }
    
    fn separate_same(&self, same_kind: Vec<Animal>) -> PVector {
        let near_animal = self.collect_near_pvectors(same_kind, 5.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animal.len() <= 1 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
    }
    
    fn align(&self, same_kind: Vec<Animal>) -> PVector{
        let near_animals = self.collect_near_pvectors(same_kind, 10.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animals.len() <= 1 {
            return PVector::zero();
        }
        self
            .add_velocity(near_animals)
    }
    
    fn add_velocity(&self, animals: Vec<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn cohension(&self, same_kind: Vec<Animal>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, 15.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animals.len() <= 1 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(-1.0)
    }
    
    pub fn run_away(&self, preyers: Vec<Animal>) -> Animal {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(preyers))
            .normalize()
            .mult(self.velocity);
        self.apply_velocity(next_velocity)
    }
    
    fn run_away_vector(&self, preyers: Vec<Animal>) -> PVector {
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
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
    
    fn as_velocity(&self) -> PVector {
        PVector {
            x: self.vx,
            y: self.vy,
        }
    }
}
