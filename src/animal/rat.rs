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
        let velocity = 1.0;
        Rat {
            x: rng.gen::<f64>() * WIDTH, 
            y: rng.gen::<f64>() * HEIGHT,
            velocity: velocity, 
            vx: theta.cos() * velocity, 
            vy: theta.sin() * velocity,
            energy: ENERGY_MAX,
        }
    }

    fn next_states(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Self> {
        //let mut ret = Animal::eat_rats(cats, rats);
        let ret = rats
            .into_iter()
            .map(|rat| rat.run_away(cats))
            .collect();
        ret
        //Animal::life_manage(&ret)
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
    
    /*
    fn collect_near_pvectors(&self, animals: &LinkedList<Cat>, radious: f64) -> LinkedList<Cat> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .filter(|animal| !(animal.x == self.x && animal.y == self.y))
            .map(|animal| animal.clone())
            .collect()
    }
    */
}

impl Rat {
    fn run_away(&self, cats: &LinkedList<Cat>) -> Rat {
        let next_velocity = self
            .as_velocity()
            //.add(self.run_away_vector(preyers))
            .normalize()
            .mult(self.velocity);
        self
            .apply_velocity(next_velocity)
            .move_self()
    }
    
    fn run_away_vector(&self, cats: &LinkedList<Cat>) -> PVector {
        /*
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
            */
        PVector::zero()
    }
}

/*
impl Animal {
    
    pub fn after_eat(&self, rats: LinkedList<Animal>) -> LinkedList<Animal> {
     rats
            .into_iter()
            .filter(|rat| !self.is_within(rat, 1.0))
            .collect()
    }
    
    
*/
