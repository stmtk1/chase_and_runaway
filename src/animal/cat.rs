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
        }
    }
    
    fn next_states(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Self> {
        let ret: LinkedList<Cat> = cats
            .into_iter()
            .map(|cat| cat.chase(cats, rats))
            .collect();
        ret
        //Animal::life_manage(&ret)
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
        if(ret.energy <= 0){
            ret.energy = 0;
        }
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
            //.eat(rats)
            .move_self()
    }
    
    fn chase_vector(&self, rats: &LinkedList<Rat>) -> PVector {
        /*
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
            .mult(-1.0 * self.chase_weight)
            */
        PVector::zero()
    }
    
    fn separate_same(&self, same_kind: &LinkedList<Cat>) -> PVector {
        /*
        let near_animal = self.collect_near_pvectors(same_kind, 5.0);
        
        if near_animal.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(self.separate_weight)
            */
        PVector::zero()
    }
    
    fn align(&self, same_kind: &LinkedList<Cat>) -> PVector{
        /*
        let near_animals = self.collect_near_pvectors(same_kind, 10.0);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_animals)
            .mult(self.align_weight)
            */
        PVector::zero()
    }
    
    fn cohension(&self, same_kind: &LinkedList<Cat>) -> PVector {
        /*
        let near_animals = self.collect_near_pvectors(same_kind, 15.0);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(-1.0 * self.cohension_weight)
            */
        PVector::zero()
    }
}

/*
impl Animal {
    pub fn new_cat() -> Animal{
        let mut ret = Animal::new();
        ret.is_rat = false;
        ret
    }
    
    pub fn eat_rats(cats: &LinkedList<Animal>, rats: &LinkedList<Animal>) -> LinkedList<Animal> {
        let mut new_rats = rats.clone();
        for cat in cats {
            new_rats = cat.after_eat(new_rats);
        }
        new_rats
    }
    
    }
    
    fn add_velocity(&self, animals: &LinkedList<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn cohension(&self, same_kind: &LinkedList<Animal>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, 15.0);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(-1.0 * self.cohension_weight)
    }
    
    fn eat(&self, rats: &LinkedList<Animal>) -> Animal {
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
    
}
*/
