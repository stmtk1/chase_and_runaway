use animal::{Animal, Cat, Rat};
use pvector::PVector;
use consts::*;
use std::collections::LinkedList;

pub enum QuadTree<T: Animal> {
    Leef{
        animals: LinkedList<T>,
    },
    Branch { 
        left_up: Box<QuadTree<T>>,
        right_up: Box<QuadTree<T>>,
        left_down: Box<QuadTree<T>>,
        right_down: Box<QuadTree<T>>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }
}

fn new_tree<T: Animal>(x: f64, y: f64, width: f64, height: f64) -> QuadTree<T> {
    if width <= CELL_WIDTH {
        QuadTree::Leef{ 
            animals: LinkedList::new(),
        }
    }else{
        let new_width = width / 2.0;
        let new_height = height / 2.0;
        let left_up = new_tree(x, y, new_width, new_height);
        let right_up = new_tree(x + new_width, y, new_width, new_height);
        let left_down = new_tree(x, y + new_height, new_width, new_height);
        let right_down = new_tree(x + new_width, y + new_height, new_width, new_height);
        QuadTree::Branch{
            left_up: Box::new(left_up),
            left_down: Box::new(left_down),
            right_up: Box::new(right_up),
            right_down: Box::new(right_down),
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }
}

fn append<T: Animal>(tree: &mut QuadTree<T>, animal: &T){
    match tree {
        QuadTree::Leef { animals } =>
            animals.push_back(animal.clone()),
        QuadTree::Branch {
            x,
            y,
            width,
            height,
            left_up,
            left_down,
            right_up,
            right_down,
        } => {
            let child_width = *width / 2.0;
            let child_height = *height / 2.0;
            let offset = PVector::new(*x, *y).mult(-1.0).add(animal.position());
            if offset.x < child_width && offset.y < child_height{
                append(left_up, animal);
            }else if offset.x < child_width {
                append(left_down, animal);
            }else if offset.y < child_height {
                append(right_up, animal);
            }else {
                append(right_down, animal);
            }
        }
    }
}

pub fn new<T: Animal>(animals : &Vec<T>) -> QuadTree<T> {
    let mut ret = new_tree::<T>(0.0, 0.0, WIDTH, HEIGHT);
    for animal in animals {
        append(&mut ret, animal);
    }
    ret
}

pub fn search<T: Animal>(tree: &QuadTree<T>, animal: T, radious: f64){
    
}

fn dist_square<T: Animal>(x: f64, y: f64, width: f64, height: f64, animal: &T) -> f64 {
    let PVector{x: animal_x, y: animal_y} = animal.position();
    if x < animal_x && animal_x < x + width && y < animal_y  && animal_y < y + height {
        return 0.0;
    }
    let mut ret = max(WIDTH, HEIGHT) + 1.0;
    ret = min(ret, PVector::new(x, y).mult(-1.0).add(animal.position()).len());
    ret = min(ret, PVector::new(x + width, y).mult(-1.0).add(animal.position()).len());
    ret = min(ret, PVector::new(x, y + height).mult(-1.0).add(animal.position()).len());
    ret = min(ret, PVector::new(x + width, y + height).mult(-1.0).add(animal.position()).len());
    
    if x < animal_x && animal_x < x + width {
        ret = min(ret, PVector::new(0.0, y).mult(-1.0).add(animal.position()).len());
        ret = min(ret, PVector::new(0.0, y + height).mult(-1.0).add(animal.position()).len());
    }
    
    if y < animal_y && animal_y < y + height {
        ret = min(ret, PVector::new(x, 0.0).mult(-1.0).add(animal.position()).len());
        ret = min(ret, PVector::new(x + width, 0.0).mult(-1.0).add(animal.position()).len());
    }
    ret
}

fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
