use pvector::PVector;
use consts::{WIDTH, HEIGHT};
use animal::{Animal, Cat, Rat};
use std::collections::LinkedList;
use rand::prelude::*;

const CHASE_MAX: f64 = 480.0;
const SEPARATE_MAX: f64 = 480.0;
const ALIGN_MAX: f64 = 480.0;
const COHENSION_MAX: f64 = 480.0;
const ENERGY_MAX: u64 = 1000;

impl Animal for Cat {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 1.0;
        Cat {
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
            ate: 0,
            id: rng.gen::<u64>(),
        }
    }
    
    fn next_states(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Self> {
        let ret: LinkedList<Cat> = cats
            .into_iter()
            .map(|cat| cat.chase(cats, rats))
            .collect();
        <Cat as Animal>::life_manage(&ret)
    }
    
    fn move_self(&self) -> Cat {
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
    
    fn as_velocity(&self) -> PVector {
        PVector {
            x: self.vx,
            y: self.vy,
        }
    }
    
    fn apply_velocity(&self, pvector: PVector) -> Self {
        let mut ret = self.clone();
        ret.vx = pvector.x;
        ret.vy = pvector.y;
        ret
    }
    
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(other_vec)
    }
    
    
    fn position(&self) -> PVector{
        PVector{
            x: self.x,
            y: self.y,
        }
    }
    
    fn collect_near_pvectors<T: Animal>(&self, animals: &LinkedList<T>, radious: f64) -> LinkedList<T> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .filter(|animal| !animal.is_same(self))
            .map(|animal| animal.clone())
            .collect()
    }
    
    fn calculate_direction<T: Animal>(&self, animals: LinkedList<T>) -> PVector {
        animals
            .into_iter()
            .map(|animal| self.offset(&animal))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn descendant(&self) -> Self{
        let mut ret = Cat::new();
        ret.chase_weight = Cat::mutate(self.chase_weight, CHASE_MAX);
        ret.separate_weight = Cat::mutate(self.separate_weight, SEPARATE_MAX);
        ret.align_weight = Cat::mutate(self.align_weight, ALIGN_MAX);
        ret.cohension_weight = Cat::mutate(self.cohension_weight, COHENSION_MAX);
        ret.velocity = self.velocity;
        ret.ate = 0;
        ret
    }
    
    fn life_manage(animals: &LinkedList<Self>) -> LinkedList<Self> {
        let mut rng = rand::thread_rng();
        let mut ret: LinkedList<Self> = LinkedList::new();
        for animal in animals {
            if animal.energy <= 0{
                continue;
            }
            if rng.gen::<f32>() < 1.0 / (ENERGY_MAX as f32) {
                ret.push_back(animal.clone().descendant());
            }
            ret.push_back(animal.clone());
        }
        ret
    }
    
    fn is_same<T: Animal>(&self, other: &T) -> bool{
        self.id() == other.id()
    }
    
    fn id(&self) -> u64 {
        self.id
    }
}

impl Cat{
    pub fn chase(&self, cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> Cat{
        let next_velocity = self
            .as_velocity()
            .add(self.chase_vector(rats))
            .add(self.separate_same(cats))
            .add(self.align(cats))
            .add(self.cohension(cats))
            .normalize()
            .mult(self.velocity);
        
        self
            .apply_velocity(next_velocity)
            .eat(rats)
            .move_self()
    }
    
    fn chase_vector(&self, rats: &LinkedList<Rat>) -> PVector {
        let near_rats = self.collect_near_pvectors(rats, 10.0);
        
        if near_rats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_rats)
            .mult(self.chase_weight)
        //PVector::zero()
    }
    
    fn separate_same(&self, cats: &LinkedList<Cat>) -> PVector {
        let near_animal = self.collect_near_pvectors(cats, 5.0);
        
        if near_animal.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(-1.0 * self.separate_weight)
    }
    
    fn align(&self, cats: &LinkedList<Cat>) -> PVector{
        let near_cats = self.collect_near_pvectors(cats, 10.0);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_cats)
            .mult(self.align_weight)
    }
    
    fn cohension(&self, same_kind: &LinkedList<Cat>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, 15.0);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(self.cohension_weight)
    }
    
    fn eat(&self, rats: &LinkedList<Rat>) -> Cat {
        let mut ret = self.clone();
        let can_eat = rats
            .into_iter()
            .any(|rat| self.is_within(rat, 1.0));
        if can_eat {
            ret.energy += 300;
            ret.ate += 1;
        }
        ret
    }
    
    fn add_velocity(&self, animals: &LinkedList<Cat>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn mutate(value: f64, value_max: f64) -> f64{
        let mut rng = rand::thread_rng();
        (rng.gen::<f64>() * 20.0 - 10.0 + value).min(value_max).max(0.0)
    }
}
