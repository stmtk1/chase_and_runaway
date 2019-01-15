use rand::prelude::*;
use pvector::PVector;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const CHASE_MAX: f64 = 480.0;
const SEPARATE_MAX: f64 = 480.0;
const ALIGN_MAX: f64 = 480.0;
const COHENSION_MAX: f64 = 480.0;
const ENERGY_MAX: u64 = 1000;

#[derive(Clone)]
pub struct Animal {
    pub x: f64,
    pub y: f64,
    velocity: f64,
    vx: f64,
    vy: f64,
    chase_weight: f64,
    separate_weight: f64,
    align_weight: f64,
    cohension_weight: f64,
    energy: u64,
}

impl Animal{
    pub fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 1.0;
        Animal{ 
            x: rng.gen::<f64>() * WIDTH, 
            y: rng.gen::<f64>() * HEIGHT,
            velocity: velocity, 
            vx: theta.cos() * velocity, 
            vy: theta.sin() * velocity,
            chase_weight: rng.gen::<f64>() * CHASE_MAX,
            separate_weight: rng.gen::<f64>() * SEPARATE_MAX,
            align_weight: rng.gen::<f64>() * ALIGN_MAX,
            cohension_weight: rng.gen::<f64>() * COHENSION_MAX,
            energy: ENERGY_MAX,
        }
    }
    
    pub fn offset(&self, other:&Animal) -> PVector {
        let self_vec = PVector { x: self.x, y: self.y };
        let other_vec = PVector { x: other.x, y: other.y };
        self_vec.offset(other_vec)
    }
    
    pub fn move_self(&self) -> Animal {
        let mut new_x = self.x + self.vx;
        let mut new_y = self.y + self.vy;
        let mut ret = self.clone();
        
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
        
        ret.energy -= 1;
        ret.x = new_x;
        ret.y = new_y;
        ret
    }
    
    fn collect_near_pvectors(&self, animals: &Vec<Animal>, radious: f64) -> Vec<Animal> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .map(|animal| animal.clone())
            .collect()
    }
    
    pub fn delete_dead(animals: Vec<Animal>) -> Vec<Animal> {
        animals
            .into_iter()
            .filter(|animal| animal.energy > 0)
            .collect()
    }
    
    fn calculate_direction(&self, animals: Vec<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.offset(self))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    pub fn chase(&self, rats: &Vec<Animal>, cats: &Vec<Animal>) -> Animal {
        let next_velocity = self
            .as_velocity()
            .add(self.chase_vector(rats))
            .add(self.separate_same(cats))
            .add(self.align(cats))
            .add(self.cohension(cats))
            .normalize()
            .mult(self.velocity);
        self.apply_velocity(next_velocity)
    }
    
    fn chase_vector(&self, preyers: &Vec<Animal>) -> PVector {
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
            .mult(-1.0 * self.chase_weight)
    }
    
    fn separate_same(&self, same_kind: &Vec<Animal>) -> PVector {
        let near_animal = self.collect_near_pvectors(same_kind, 5.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animal.len() <= 1 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(self.separate_weight)
    }
    
    fn align(&self, same_kind: &Vec<Animal>) -> PVector{
        let near_animals = self.collect_near_pvectors(same_kind, 10.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animals.len() <= 1 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_animals)
            .mult(self.align_weight)
    }
    
    fn add_velocity(&self, animals: &Vec<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn cohension(&self, same_kind: &Vec<Animal>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, 15.0);
        
        // 自分自身もカウントされてしまうため1
        // TODO 自分自身がカウントされないようにする
        if near_animals.len() <= 1 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(-1.0 * self.cohension_weight)
    }
    
    pub fn run_away(&self, preyers: &Vec<Animal>) -> Animal {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(preyers))
            .normalize()
            .mult(self.velocity);
        self.apply_velocity(next_velocity)
    }
    
    fn run_away_vector(&self, preyers: &Vec<Animal>) -> PVector {
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
    
    fn mutate(value: f64, value_max: f64) -> f64{
        let mut rng = rand::thread_rng();
        (rng.gen::<f64>() * 2.0 - 1.0 + value).min(value_max).max(0.0)
    }
    
    fn descendant(&self) -> Animal{
        let mut ret = Animal::new();
        ret.chase_weight = Animal::mutate(self.chase_weight, CHASE_MAX);
        ret.separate_weight = Animal::mutate(self.separate_weight, SEPARATE_MAX);
        ret.align_weight = Animal::mutate(self.align_weight, ALIGN_MAX);
        ret.cohension_weight = Animal::mutate(self.cohension_weight, COHENSION_MAX);
        ret
    }
    
    pub fn life_manage(animals: Vec<Animal>) -> Vec<Animal> {
        let mut rng = rand::thread_rng();
        let mut ret: Vec<Animal> = Vec::with_capacity(animals.len() * 2);
        for animal in animals {
            // 複製する確率をきちんと決める
            if rng.gen::<f32>() < 0.0011 {
                ret.push(animal.clone().descendant());
            }
            ret.push(animal.clone());
        }
        ret
    }
    
    pub fn eat(&self, rats: Vec<Animal>) -> Vec<Animal> {
     rats
            .into_iter()
            .filter(|rat| !self.is_within(rat, 1.0) )
            .collect()
    }
    
    pub fn is_within(&self, other: &Animal, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    
    fn as_velocity(&self) -> PVector {
        PVector {
            x: self.vx,
            y: self.vy,
        }
    }
}
