mod test;

use pvector::PVector;
use consts::*;
use animal::{Animal, Cat, Rat};
use rand::prelude::*;

impl Animal for Cat {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = PVector::new(theta.cos(), theta.sin()).mult(CAT_VELOCITY);
        let x = rng.gen::<f64>() * WIDTH;
        let y = rng.gen::<f64>() * HEIGHT;
        Cat {
            position: PVector::new(x, y),
            velocity: velocity, 
            chase_weight: rng.gen::<f64>() * CHASE_MAX,
            separate_weight: rng.gen::<f64>() * SEPARATE_MAX,
            align_weight: rng.gen::<f64>() * ALIGN_MAX,
            cohension_weight: rng.gen::<f64>() * COHENSION_MAX,
            energy: ENERGY_MAX,
            ate: 0,
            id: rng.gen::<u64>(),
        }
    }
    
    fn next_states(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Self> {
        let ret: Vec<Cat> = cats
            .into_iter()
            .map(|cat| cat.chase(cats, rats))
            .collect();
        <Cat as Animal>::life_manage(&ret)
    }
    
    fn move_self(&self) -> Cat {
        let mut new_pos = self.position().add(self.as_velocity());
        let mut ret = self.clone();
        
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
        
        ret.energy -= 1;
        
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
    
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(&other_vec)
    }
    
    fn position(&self) -> PVector{
        self.position.clone()
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
        let mut ret = Cat::new();
        ret.chase_weight = Cat::mutate(self.chase_weight, CHASE_MAX);
        ret.separate_weight = Cat::mutate(self.separate_weight, SEPARATE_MAX);
        ret.align_weight = Cat::mutate(self.align_weight, ALIGN_MAX);
        ret.cohension_weight = Cat::mutate(self.cohension_weight, COHENSION_MAX);
        ret.ate = 0;
        ret
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
    
    fn id(&self) -> u64 {
        self.id
    }
}

impl Cat{
    pub fn chase(&self, cats: &Vec<Cat>, rats: &Vec<Rat>) -> Cat{
        let next_velocity = self
            .as_velocity()
            .add(self.chase_vector(rats))
            .add(self.separate_same(cats))
            .add(self.align(cats))
            .add(self.cohension(cats))
            .normalize()
            .mult(self.velocity.len());
        
        self
            .apply_velocity(&next_velocity)
            .eat(rats)
            .move_self()
    }
    
    fn chase_vector(&self, rats: &Vec<Rat>) -> PVector {
        let near_rats = self.collect_near_pvectors(rats, CHASE_RADIOUS);
        
        if near_rats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_rats)
            .mult(self.chase_weight)
    }
    
    fn separate_same(&self, cats: &Vec<Cat>) -> PVector {
        let near_animal = self.collect_near_pvectors(cats, SEPARATE_RADIOUS);
        
        if near_animal.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(-1.0 * self.separate_weight)
    }
    
    fn align(&self, cats: &Vec<Cat>) -> PVector{
        let near_cats = self.collect_near_pvectors(cats, ALIGN_RADIOUS);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_cats)
            .mult(self.align_weight)
    }
    
    fn cohension(&self, same_kind: &Vec<Cat>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, COHENSION_RADIOUS);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(self.cohension_weight)
    }
    
    fn eat(&self, rats: &Vec<Rat>) -> Cat {
        let mut ret = self.clone();
        let can_eat = rats
            .into_iter()
            .any(|rat| self.is_within(rat, EATEN_RADIOUS));
        if can_eat {
            ret.energy += EAT_ENERGY;
            ret.ate += 1;
        }
        ret
    }
    
    fn add_velocity(&self, animals: &Vec<Cat>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    fn mutate(value: f64, value_max: f64) -> f64{
        let mut rng = rand::thread_rng();
        (rng.gen::<f64>() * MUTATE_ABS * 2.0 - MUTATE_ABS + value).min(value_max).max(0.0)
    }
    
    fn collect_servive(cats: &Vec<Cat>) -> Vec<Cat> {
        let len = cats.len();
        let mut ret = cats.clone();
        ret.sort_by(Cat::comp_by_ate);
         ret
            .into_iter()
            .take(len / 10)
            .collect()
    }
    
    fn comp_by_ate(a1: &Cat, a2: &Cat) -> std::cmp::Ordering {
        if a1.ate < a2.ate {
            std::cmp::Ordering::Less
        } else if a1.ate > a2.ate {
            std::cmp::Ordering::Greater
        }else{
            std::cmp::Ordering::Equal
        }
    }
    
    pub fn next_generation(cats: &Vec<Cat>) -> Vec<Cat>{
        let mut ret: Vec<Cat> = Vec::new();
        let superior = Cat::collect_servive(cats);
        while ret.len() < 20 {
            let mut appended = superior
                .clone()
                .into_iter()
                .map(|animal| animal.descendant())
                .collect();
            
            ret.append(&mut appended);
        }
        
        ret
            .into_iter()
            .take(20)
            .collect()
    }
}

