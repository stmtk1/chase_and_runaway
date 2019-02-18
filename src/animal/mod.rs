mod cat;
mod rat;

use rand::prelude::*;
use pvector::PVector;
use consts::{WIDTH, HEIGHT};
use std::collections::LinkedList;
//use std::collections::linked_list::Iter;
use std::iter::Iterator;
use std::iter::FromIterator;

const CHASE_MAX: f64 = 480.0;
const SEPARATE_MAX: f64 = 480.0;
const ALIGN_MAX: f64 = 480.0;
const COHENSION_MAX: f64 = 480.0;
const ENERGY_MAX: u64 = 1000;

#[derive(Clone)]
pub struct Animal {
    is_rat: bool,
    pub x: f64,
    pub y: f64,
    velocity: f64,
    vx: f64,
    vy: f64,
    chase_weight: f64,
    separate_weight: f64,
    align_weight: f64,
    cohension_weight: f64,
    ate: u32,
    energy: u64,
}

impl Animal{
    fn new() -> Animal{
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = 1.0;
        Animal{
            is_rat: true,
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
    
    pub fn position(&self) -> PVector {
        PVector {
            x: self.x,
            y: self.y,
        }
    }
    
    pub fn offset(&self, other:&Animal) -> PVector {
        let self_vec = PVector { x: self.x, y: self.y };
        let other_vec = PVector { x: other.x, y: other.y };
        self_vec.offset(other_vec)
    }
    
    fn move_self(&self) -> Animal {
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
    
    
    fn collect_near_pvectors(&self, animals: &LinkedList<Animal>, radious: f64) -> LinkedList<Animal> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .filter(|animal| !(animal.x == self.x && animal.y == self.y))
            .map(|animal| animal.clone())
            .collect()
    }
    
    fn calculate_direction(&self, animals: LinkedList<Animal>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.offset(self))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
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
        ret.velocity = self.velocity;
        ret.ate = 0;
        ret
    }
    
    fn life_manage(animals: &LinkedList<Animal>) -> LinkedList<Animal> {
        let mut rng = rand::thread_rng();
        let mut ret: LinkedList<Animal> = LinkedList::new();
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
    
    fn is_within(&self, other: &Animal, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    
    fn as_velocity(&self) -> PVector {
        PVector {
            x: self.vx,
            y: self.vy,
        }
    }
    
    fn collect_servive(cats: &LinkedList<Animal>) -> LinkedList<Animal> {
        let len = cats.len();
        let mut ret: Vec<Animal> = Vec::from_iter(cats.into_iter().map(|a| a.clone()));
        
        ret.sort_by(Animal::sort_by_ate);
         ret
            .into_iter()
            .take(len / 10)
            .collect()
    }
    
    fn sort_by_ate(a1: &Animal, a2: &Animal) -> std::cmp::Ordering {
        if a1.ate < a2.ate {
            std::cmp::Ordering::Less
        } else if a1.ate > a2.ate {
            std::cmp::Ordering::Greater
        }else{
            std::cmp::Ordering::Equal
        }
    }
    
    pub fn next_generation(cats: &LinkedList<Animal>) -> LinkedList<Animal>{
        let mut ret: LinkedList<Animal> = LinkedList::new();
        let mut superior = Animal::collect_servive(cats);
        while ret.len() < 100 {
            /*
            let mut next = &superior
                .into_iter()
                .map(|animal| animal.descendant())
                .collect();
                */
            
            ret.append(&mut superior);
        }
        ret
            .into_iter()
            .take(100)
            .collect()
    }
}
