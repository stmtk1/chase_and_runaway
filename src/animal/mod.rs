mod cat;
mod rat;

use pvector::PVector;
use std::collections::LinkedList;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct Cat{
    pub x: f64,
    pub y: f64,
    velocity: f64,
    vx: f64,
    vy: f64,
    pub chase_weight: f64,
    pub separate_weight: f64,
    pub align_weight: f64,
    pub cohension_weight: f64,
    ate: u32,
    energy: u64,
    id: u64,
}

#[derive(Clone)]
pub struct Rat{
    pub x: f64,
    pub y: f64,
    velocity: f64,
    vx: f64,
    vy: f64,
    energy: u64,
    id: u64,
}

pub trait Animal : Clone {
    fn new() -> Self;
    fn next_states(cats: &LinkedList<Cat>, rats: &LinkedList<Rat>) -> LinkedList<Self> where Self: std::marker::Sized;
    fn move_self(&self) -> Self;
    fn as_velocity(&self) -> PVector;
    fn apply_velocity(&self, PVector) -> Self;
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool;
    fn offset<T: Animal>(&self, other: &T) -> PVector;
    fn collect_near_pvectors<T: Animal>(&self, animals: &LinkedList<T>, radious: f64) -> LinkedList<T>;
    fn position(&self) -> PVector;
    fn calculate_direction<T: Animal>(&self, animals: LinkedList<T>) -> PVector;
    fn descendant(&self) -> Self;
    fn life_manage(animals: &LinkedList<Self>) -> LinkedList<Self>;
    fn is_same<T: Animal>(&self, other: &T) -> bool;
    fn id(&self) -> u64;
}

impl Cat{
    fn collect_servive(cats: &LinkedList<Cat>) -> LinkedList<Cat> {
        let len = cats.len();
        let mut ret: Vec<Cat> = Vec::from_iter(cats.into_iter().map(|a| a.clone()));
        
        ret.sort_by(Cat::sort_by_ate);
         ret
            .into_iter()
            .take(len / 10)
            .collect()
    }
    
    fn sort_by_ate(a1: &Cat, a2: &Cat) -> std::cmp::Ordering {
        if a1.ate < a2.ate {
            std::cmp::Ordering::Less
        } else if a1.ate > a2.ate {
            std::cmp::Ordering::Greater
        }else{
            std::cmp::Ordering::Equal
        }
    }
    
    pub fn next_generation(cats: &LinkedList<Cat>) -> LinkedList<Cat>{
        let mut ret: LinkedList<Cat> = LinkedList::new();
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
