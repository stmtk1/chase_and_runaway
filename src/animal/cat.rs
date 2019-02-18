use pvector::PVector;
use animal::Animal;
use std::collections::LinkedList;

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
    
    pub fn chase(&self, rats: &LinkedList<Animal>, cats: &LinkedList<Animal>) -> Animal {
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
    
    fn chase_vector(&self, preyers: &LinkedList<Animal>) -> PVector {
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
            .mult(-1.0 * self.chase_weight)
    }
    
    fn separate_same(&self, same_kind: &LinkedList<Animal>) -> PVector {
        let near_animal = self.collect_near_pvectors(same_kind, 5.0);
        
        if near_animal.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(self.separate_weight)
    }
    
    fn align(&self, same_kind: &LinkedList<Animal>) -> PVector{
        let near_animals = self.collect_near_pvectors(same_kind, 10.0);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_animals)
            .mult(self.align_weight)
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
        }
        ret
    }
    
    pub fn next_states_cats(cats: &LinkedList<Animal>, rats: &LinkedList<Animal>) -> LinkedList<Animal> {
        let ret: LinkedList<Animal> = cats
            .into_iter()
            .map(|cat| cat.chase(rats, cats))
            .collect();
        Animal::life_manage(&ret)
    }
}
