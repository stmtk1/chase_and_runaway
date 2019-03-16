mod test;

use pvector::PVector;
use animal::{Animal, Rat, Cat};
use consts::*;
use rand::prelude::*;


impl Animal for Rat {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let x = rng.gen::<f64>() * WIDTH;
        let y = rng.gen::<f64>() * HEIGHT;
        let velocity = RAT_VELOCITY;
        Rat {
            position: PVector::new(x, y),
            velocity: PVector::new(theta.cos(), theta.sin()).mult(velocity),
            energy: ENERGY_MAX,
            id: rng.gen::<u64>(),
        }
    }

    fn next_states(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Self> {
        let alive_rats = Rat::delete_eaten(cats, rats);
        let ret = alive_rats
            .into_iter()
            .map(|rat| rat.run_away(cats))
            .collect();
        <Rat as Animal>::life_manage(&ret)
    }
    
    fn move_self(&self) -> Rat {
        let mut ret = self.clone();
        let mut new_pos = ret.position.add(self.clone().velocity);
        
        if new_pos.x > WIDTH {
            new_pos.x -= WIDTH;
        }
        
        if new_pos.x < 0.0 {
            new_pos.x += WIDTH;
        }
        
        if new_pos.y > HEIGHT {
            new_pos.y -= HEIGHT;
        }
        
        if new_pos.y < 0.0 {
            new_pos.y += HEIGHT;
        }
        
        //ret.energy -= 1;
        ret.position = new_pos;
        ret
    }
    
    fn as_velocity(&self) -> PVector {
        self.velocity.clone()
    }
    
    fn apply_velocity(&self, pvector: &PVector) -> Self {
        let mut ret = self.clone();
        ret.velocity = pvector.clone();
        ret
    }
    
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    
    fn position(&self) -> PVector{
        self.position.clone()
    }
    
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(&other_vec)
    }
    
    fn collect_near_pvectors<T: Animal>(&self, animals: &Vec<T>, radious: f64) -> Vec<T> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .filter(|animal| !animal.is_same(self))
            .map(|animal| animal.clone())
            .collect()
    }
    
    fn calculate_direction<T: Animal>(&self, animals: Vec<T>) -> PVector {
        animals
            .into_iter()
            .map(|animal| self.offset(&animal))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn descendant(&self) -> Self{
        let mut ret = Rat::new();
        ret.energy = ENERGY_MAX;
        ret
    }
    
    fn id(&self) -> u64 {
        self.id
    }
    
    fn life_manage(animals: &Vec<Self>) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut ret: Vec<Self> = Vec::new();
        for animal in animals {
            if animal.energy <= 0{
                continue;
            }
            if rng.gen::<f32>() < 1.0 / (ENERGY_MAX as f32) {
                ret.push(animal.clone().descendant());
            }
            ret.push(animal.clone());
        }
        ret
    }
    
    fn is_same<T: Animal>(&self, other: &T) -> bool{
        self.id() == other.id()
    }
}

impl Rat {
    fn run_away(&self, cats: &Vec<Cat>) -> Rat {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(cats))
            .normalize()
            .mult(self.velocity.len());
        self
            .apply_velocity(&next_velocity)
            .move_self()
    }
    
    fn run_away_vector(&self, cats: &Vec<Cat>) -> PVector {
        let near_cats = self.collect_near_pvectors(cats, RUNAWAY_RADIOUS);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_cats)
            .mult(-1.0)
    }
    
    fn eaten(&self, cats: &Vec<Cat>) -> bool{
        cats
            .into_iter()
            .any(|cat| self.is_within(cat, 1.0))
    }
    
    fn delete_eaten(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Rat> {
        rats
            .into_iter()
            .filter(|rat| !rat.eaten(cats))
            .map(|rat| rat.clone())
            .collect()
    }
}
