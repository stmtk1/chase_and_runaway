use animal::{Animal, Cat, Rat};
use pvector::PVector;
use consts::*;
use std::collections::LinkedList;
use std::cell::RefCell;
use std::rc::Rc;

const WIDTH_MIN: f64 = 10.0;

#[derive(Debug, Clone)]
struct Rectangle {
    x:      f64,
    y:      f64,
    width:  f64,
    height: f64,
}

#[derive(Debug, Clone)]
pub enum QuadTree<T: Animal> {
    Leef{
        animals:    LinkedList<T>,
        rectangle:  Rectangle,
    },
    Branch { 
        children: Vec<Option<Rc<RefCell<QuadTree<T>>>>>,
        rectangle:  Rectangle,
    }
}

impl<T: Animal> QuadTree<T> {
    fn new_tree(rect: &Rectangle) -> QuadTree<T> {
        if rect.width < WIDTH_MIN {
            QuadTree::Leef {
                animals:    LinkedList::new(),
                rectangle:  rect.clone(),
            }
        } else {
            let mut children: Vec<_> = Vec::new();
            for n in 0..4 {
                children.push(QuadTree::optimize(rect.child(n)));
            }
            
            println!("{:?}", children.len());
            QuadTree::Branch {
                children:   children,
                rectangle:  rect.clone(),
            }
        }
    }
    
    fn optimize(rect: Rectangle) -> Option<Rc<RefCell<QuadTree<T>>>> {
        Some(Rc::new(RefCell::new(QuadTree::new_tree(&rect))))
    }
}

impl Rectangle {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        Rectangle {
            x:      x,
            y:      y,
            width:  width,
            height: height,
        }
    }
    
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
            x:      x,
            y:      y,
            width:  self.width / 2.0,
            height: self.height / 2.0,
        }
    }
}
