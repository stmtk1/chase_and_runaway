mod cat;
//mod test;
mod rat;

use pvector::PVector;

#[derive(Clone)]
pub struct Cat{
    position: PVector,
    velocity: PVector,
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
    position: PVector,
    velocity: PVector,
    energy: u64,
    id: u64,
}

pub trait Animal : Clone {
    fn new() -> Self;
    fn next_states(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Self> where Self: std::marker::Sized;
    fn move_self(&self) -> Self;
    fn as_velocity(&self) -> PVector;
    fn apply_velocity(&self, &PVector) -> Self;
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool;
    fn offset<T: Animal>(&self, other: &T) -> PVector;
    fn collect_near_pvectors<T: Animal>(&self, animals: &Vec<T>, radious: f64) -> Vec<T>;
    fn position(&self) -> PVector;
    fn calculate_direction<T: Animal>(&self, animals: Vec<T>) -> PVector;
    fn descendant(&self) -> Self;
    fn life_manage(animals: &Vec<Self>) -> Vec<Self>;
    fn is_same<T: Animal>(&self, other: &T) -> bool;
    fn id(&self) -> u64;
}
