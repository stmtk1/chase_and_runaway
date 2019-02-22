use pvector::PVector;
use animal::{Animal, Rat, Cat};
use consts::{WIDTH, HEIGHT};
use std::collections::LinkedList;
use rand::prelude::*;

const ENERGY_MAX: u64 = 1000;

impl Animal for Rat {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 0.5;
        Rat {
            x: rng.gen::<f64>() * WIDTH, 
            y: rng.gen::<f64>() * HEIGHT,
            velocity: velocity, 
            vx: theta.cos() * velocity, 
            vy: theta.sin() * velocity,
            energy: ENERGY_MAX,
            id: rng.gen::<u64>(),
        }
    }

    fn next_states(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Self> {
        let alive_rats = Rat::delete_eaten(cats, rats);
        let ret = alive_rats
            .into_iter()
            .map(|rat| rat.run_away(cats))
            .collect();
        <Rat as Animal>::life_manage(&ret)
    }
    
    fn move_self(&self) -> Rat {
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
    
    fn position(&self) -> PVector{
        PVector{
            x: self.x,
            y: self.y,
        }
    }
    
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(other_vec)
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
        let mut ret = Rat::new();
        ret.velocity = self.velocity;
        ret
    }
    
    fn id(&self) -> u64 {
        self.id
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
}

impl std::cmp::PartialEq for Rat {
    fn eq(&self, other: &Rat) -> bool {
        self.id == other.id
    }
}

impl Rat {
    fn run_away(&self, cats: &LinkedList<Cat>) -> Rat {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(cats))
            .normalize()
            .mult(self.velocity);
        self
            .apply_velocity(next_velocity)
            .move_self()
    }
    
    fn run_away_vector(&self, cats: &LinkedList<Cat>) -> PVector {
        let near_cats = self.collect_near_pvectors(cats, 10.0);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_cats)
            .mult(-1.0)
    }
    
    fn eaten(&self, cats: &LinkedList<Cat>) -> bool{
        cats
            .into_iter()
            .any(|cat| self.is_within(cat, 1.0))
    }
    
    fn delete_eaten(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Rat> {
        rats
            .into_iter()
            .filter(|rat| !rat.eaten(cats))
            .map(|rat| rat.clone())
            .collect()
    }
}
