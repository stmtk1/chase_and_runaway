mod test;

use animal::Animal;
use pvector::PVector;
use consts::*;
use std::collections::LinkedList;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH_LIMIT: f64 = 10.0;

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    x:      f64,
    y:      f64,
    width:  f64,
    height: f64,
}

#[derive(Debug, Clone)]
pub struct QuadTree<T: Animal> {
    rectangle:  Rectangle,
    children:   Option<Vec<Rc<RefCell<QuadTree<T>>>>>,
    animals:    Option<LinkedList<T>>,
}

impl<T: Animal> QuadTree<T> {
    fn new_tree(rect: &Rectangle) -> QuadTree<T> {
        if rect.width >= WIDTH_LIMIT {
            let mut children: Vec<Rc<RefCell<QuadTree<T>>>> = Vec::with_capacity(4);
            for n in 0..4 {
                children.push(QuadTree::optimize(rect.child(n)));
            }
            QuadTree {
                rectangle:  rect.clone(),
                animals:    None,
                children:   Some(children),
            }
        } else {
            QuadTree {
                rectangle:  rect.clone(),
                animals:    Some(LinkedList::new()),
                children:   None,
            }
        }
    }
    
    fn optimize(rect: Rectangle) -> Rc<RefCell<QuadTree<T>>> {
        Rc::new(RefCell::new(QuadTree::new_tree(&rect)))
    }
    
    fn append(&mut self, animal: &T) {
        if let Some(ref mut animals)  = self.animals {
            animals.push_back(animal.clone());
        } else if let Some(ref mut children) = self.children {
            for child in children {
                let mut tree = child.borrow_mut();
                if tree.rectangle.is_inside(&animal.position()) {
                    tree.append(animal);
                }
            }
        } else {
            panic!("both None");
        }
    }
    
    pub fn remove(&mut self, animal: &T) {
        if let Some(ref mut children) = self.children {
            for child in children {
                let mut tree = child.borrow_mut();
                if tree.rectangle.is_inside(&animal.position()) {
                    tree.remove(animal);
                    break;
                }
            }
        } else {
            let animals = self.animals
                .clone()
                .unwrap()
                .into_iter()
                .filter(|other| !animal.is_same(other))
                .collect();
            self.animals = Some(animals);
        }
    }
    
    pub fn new(animals: &Vec<T>) -> QuadTree<T> {
        let mut tree = QuadTree::new_tree(&Rectangle::whole_screen());
        for animal in animals {
            tree.append(animal);
        }
        tree
    }

    pub fn search<S: Animal>(&self, animal: &S, radious: f64) -> LinkedList<T> {
        if self.rectangle.min_dist(animal) > radious {
            return LinkedList::new();
        } else if let Some(ref animals) = self.animals {
            return animals
                .into_iter()
                .filter(|other| other.is_within(animal, radious))
                .map(|animal| animal.clone())
                .collect()
        }

        if let Some(ref children) = self.children {
            let mut ret = LinkedList::new();
            for child in children {
                let tree = child.borrow();
                if tree.rectangle.min_dist(animal) < radious {
                    let mut animals = tree.search(animal, radious);
                    ret.append(&mut animals);
                }
            }
            ret
        } else {
            panic!("both none");
        }
    }
    
    pub fn is_move_tree(&self, animal: &T) -> bool {
        self.rectangle.get_index((0, 0), &animal.position()) !=
            self.rectangle.get_index((0, 0), &animal.move_self().position())
    }
}
    
impl Rectangle {
    fn child(&self, num: u8) -> Rectangle{
        let x = if num % 2 == 0 {
            self.x
        } else {
            self.x + self.width / 2.0
        };
        let y = if num / 2 == 0 {
            self.y
        } else {
            self.y + self.height / 2.0
        };
        Rectangle {
            x, y,
            width:  self.width / 2.0,
            height: self.height / 2.0,
        }
    }
    
    fn is_inside(&self, vector: &PVector) -> bool{
        let PVector{x, y} = vector;
        self.x < *x 
            && *x < self.x + self.width
            && self.y < *y
            && *y < self.y + self.height
    }

    fn point(&self, n: u8) -> PVector{
        let x = if n % 2 == 0 {
            self.x
        } else {
            self.x + self.width
        };
        let y = if n % 2 == 0 {
            self.y
        } else {
            self.y + self.height
        };

        PVector{
            x, y
        }
    }

    fn min_dist<T: Animal>(&self, animal: &T) -> f64 {
        let position = animal.position();
        let PVector{x, y} = position;
        let x_contain = self.x < x && x < self.x + self.width;
        let y_contain = self.y < y && y < self.y + self.height;
        if x_contain && y_contain {
            0.0
        } else if x_contain {
            min(
                PVector{x, y: self.y}.offset(&position).len(),
                PVector{x, y: self.y + self.height}.offset(&position).len()
            )
        } else if y_contain {
            min(
                PVector{x: self.x, y}.offset(&position).len(),
                PVector{x: self.x + self.width, y}.offset(&position).len()
            )
        }else {
            (0..4)
                .map(|n| self.point(n).offset(&animal.position()).len())
                .fold(WIDTH + 100.0, |a, b| if a < b { a } else { b })
        }
    }
    
    pub fn whole_screen() -> Rectangle {
        Rectangle{
            x: 0.0,
            y: 0.0,
            width: WIDTH,
            height: HEIGHT,
        }
    }
    pub fn min_rectangle(&self) -> Rectangle {
        Rectangle::whole_screen().split_rectangle()
    }
    
    fn split_rectangle(&self) -> Rectangle {
        if self.width < WIDTH_LIMIT {
            self.clone()
        } else {
            Rectangle {
                width: self.width / 2.0,
                height: self.height / 2.0,
                x: self.x,
                y: self.y,
            }
        }
    }
    
    
    fn get_index(&self, index: (usize, usize), vector: &PVector) -> (usize, usize) {
        println!("{:?}", index);
        if self.width < WIDTH_LIMIT {
            return index;
        }
        
        for i in 0..4 {
            if self.child(i).is_inside(vector) {
                let (x, y) = index;
                let new_x = if i % 2 == 0 { 0 } else { 1 } + x * 2;
                let new_y = if i / 2 == 0 { 0 } else { 1 } + y * 2;
                return self.child(i).get_index((new_x, new_y), vector);
            }
        }
        panic!("can not find index");
    }
}
