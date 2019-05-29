#[cfg(test)]
mod rectangle_tests{
    use quad_tree::{Rectangle, QuadTree};
    use consts::*;
    use animal::{Cat, Animal};
    use pvector::PVector;
    use std::collections::LinkedList;
    
    macro_rules! assert_float{
        (
            $x: expr ,$y: expr
        ) => {
            {
                assert!((($x - $y) / $x).abs() < 1.0e-9);
            }
        }
    }
    
    fn get_all_animals(tree: &QuadTree<Cat>) -> LinkedList<Cat> {
        if let Some(ref children) = tree.children {
            let mut ret = LinkedList::new();
            for child in children {
                let mut animals = get_all_animals(&child.borrow());
                ret.append(&mut animals);
            }
            ret
        } else {
            tree.animals.clone().unwrap()
        }
    }
    
    fn sized_rect(width: f64, height: f64) -> Rectangle{
        Rectangle {
            x: 0.0,
            y: 0.0,
            width, height
        }
    }
    
    fn positioned_cat(x: f64, y: f64) -> Cat {
        let ret = Cat::new().set_position(&PVector{x, y});
        ret
    }
    
    fn tree_depth(tree: &QuadTree<Cat>) -> usize {
        if let Some(ref children) = tree.children {
            tree_depth(&children[0].borrow_mut()) + 1
        } else {
            0
        }
    }
    
    fn tree_minsq(tree: &QuadTree<Cat>) -> Rectangle {
        if let Some(ref children) = tree.children {
            tree_minsq(&children[0].borrow_mut())
        } else {
           tree.rectangle.clone()
        }
    }
    
    fn tree_parse(tree: &QuadTree<Cat>) {
        if let Some(ref children) = tree.children {
            if let Some(_) = tree.animals {
                panic!("Both Variables have value!");
            }
            for child in children {
                let child_tree = &child.borrow();
                tree_parse(child_tree);
            }
        } else {
            match tree.animals {
                None => panic!("Nor Variable have value!"),
                _ => ()
            }
        }
    }
    
    fn tree_animals_size(tree: &QuadTree<Cat>, index: usize) -> usize {
        if let Some(ref children) = tree.children {
            tree_animals_size(&children[index].borrow(), index)
        } else {
            match tree.animals.clone() {
                Some(animals) => { 
                    animals.len() 
                },
                _ => 1939291 // random number
            }
        }
    }
    
    #[test]
    fn whole_screen_test(){
        let Rectangle{ width, height, .. } = Rectangle::whole_screen();
        assert_eq!(WIDTH, width);
        assert_eq!(HEIGHT, height);
    }
    
    #[test]
    fn new_tree_test(){
        let rect = sized_rect(1000.0, 2000.0);
        let tree = QuadTree::new_tree(&rect);
        assert_eq!(tree_depth(&tree), 7);
        let Rectangle{width, height, .. } = tree_minsq(&tree);
        assert_float!(7.8125, width);
        assert_float!(15.625, height);
    }
    
    #[test]
    fn new_tree_child_test(){
        tree_parse(&QuadTree::new_tree(&sized_rect(1000.0, 2000.0)));
    }
    
    #[test]
    fn append_test_left_up(){
        let rect = sized_rect(1000.0, 2000.0);
        let mut tree = QuadTree::new_tree(&rect);
        let cat = positioned_cat(5.0, 10.0);
        
        for _ in 0..10 {
            tree.append(&cat);
        }
        assert_eq!(tree_animals_size(&tree, 0), 10);
        assert_eq!(tree_animals_size(&tree, 1), 0);
        assert_eq!(tree_animals_size(&tree, 2), 0);
        assert_eq!(tree_animals_size(&tree, 3), 0);
    }
    
    #[test]
    fn append_test_right_up(){
        let rect = sized_rect(1000.0, 2000.0);
        let mut tree = QuadTree::new_tree(&rect);
        let cat = positioned_cat(995.0, 10.0);
        
        for _ in 0..10 {
            tree.append(&cat);
        }
        assert_eq!(tree_animals_size(&tree, 0), 0);
        assert_eq!(tree_animals_size(&tree, 1), 10);
        assert_eq!(tree_animals_size(&tree, 2), 0);
        assert_eq!(tree_animals_size(&tree, 3), 0);
    }
    
    #[test]
    fn append_test_left_down(){
        let rect = sized_rect(1000.0, 2000.0);
        let mut tree = QuadTree::new_tree(&rect);
        let cat = positioned_cat(5.0, 1990.0);
        
        for _ in 0..10 {
            tree.append(&cat);
        }
        assert_eq!(tree_animals_size(&tree, 0), 0);
        assert_eq!(tree_animals_size(&tree, 1), 0);
        assert_eq!(tree_animals_size(&tree, 2), 10);
        assert_eq!(tree_animals_size(&tree, 3), 0);
    }
    
    #[test]
    fn append_test_right_down(){
        let rect = sized_rect(1000.0, 2000.0);
        let mut tree = QuadTree::new_tree(&rect);
        let cat = positioned_cat(995.0, 1990.0);
        
        for _ in 0..10 {
            tree.append(&cat);
        }
        assert_eq!(tree_animals_size(&tree, 0), 0);
        assert_eq!(tree_animals_size(&tree, 1), 0);
        assert_eq!(tree_animals_size(&tree, 2), 0);
        assert_eq!(tree_animals_size(&tree, 3), 10);
    }
    
    #[test]
    fn new_test(){
        let mut animals = Vec::with_capacity(128 * 128);
        let width = 5.0;
        let height = 3.75;
        for i in 0..128 {
            for j in 0..128 {
                animals.push(positioned_cat((i as f64 + 0.5) * width, (j as f64 + 0.5) * height));
            } 
        } 
        let tree = QuadTree::new(&animals);
        let animals = get_all_animals(&tree);
        assert_eq!(animals.len(), 128 * 128);
    }
    
    /*
     * TODO テストに通す
    #[test]
    fn new_corner_test(){
        let mut animals = Vec::with_capacity(129 * 129);
        let width = 5.0;
        let height = 3.75;
        for i in 0..129 {
            for j in 0..129 {
                animals.push(positioned_cat((i as f64) * width, (j as f64) * height));
            } 
        } 
        let tree = QuadTree::new(&animals);
        let animals = get_all_animals(&tree);
        assert_eq!(animals.len(), 129 * 129);
    }
    */
    
    #[test]
    fn remove_center_test(){
        let mut animals = Vec::with_capacity(128 * 128);
        let width = 5.0;
        let height = 3.75;
        for i in 0..128 {
            for j in 0..128 {
                animals.push(positioned_cat((i as f64 + 0.5) * width, (j as f64 + 0.5) * height));
            } 
        }
        let mut tree = QuadTree::new(&animals);
        for cat in &animals {
            tree.remove(cat);
        }
        let animals = get_all_animals(&tree);
        assert_eq!(animals.len(), 0);
    }
    
    #[test]
    fn remove_corner_test(){
        let mut animals = Vec::with_capacity(129 * 129);
        let width = 5.0;
        let height = 3.75;
        for i in 0..129 {
            for j in 0..129 {
                animals.push(positioned_cat((i as f64) * width, (j as f64) * height));
            } 
        }
        let mut tree = QuadTree::new(&animals);
        for cat in &animals {
            tree.remove(cat);
        }
        let animals = get_all_animals(&tree);
        assert_eq!(animals.len(), 0);
    }
    
    #[test]
    fn get_index_test(){
        let vector = PVector {
            x: 0.0 * 5.0 + 2.5,
            y: 0.0 * 3.75 + 1.0,
        };
        let rect = Rectangle::whole_screen();
        assert_eq!((0, 0), rect.get_index((0, 0), &vector));
    }
    
    #[test]
    fn get_index_test2(){
        let vector = PVector {
            x: 127.0 * 5.0 + 2.5,
            y: 127.0 * 3.75 + 1.0,
        };
        let rect = Rectangle::whole_screen();
        assert_eq!((127, 127), rect.get_index((0, 0), &vector));
    }
    
    #[test]
    fn get_index_test3(){
        let vector = PVector {
            x: 0.0 * 5.0 + 2.5,
            y: 127.0 * 3.75 + 1.0,
        };
        let rect = Rectangle::whole_screen();
        assert_eq!((0, 127), rect.get_index((0, 0), &vector));
    }
    
    #[test]
    fn get_index_test4(){
        let vector = PVector {
            x: 127.0 * 5.0 + 2.5,
            y: 0.0 * 3.75 + 1.0,
        };
        let rect = Rectangle::whole_screen();
        assert_eq!((127, 0), rect.get_index((0, 0), &vector));
    }
    
    #[test]
    fn get_index_test5(){
        let vector = PVector {
            x: 51.0 * 5.0 + 2.5,
            y: 112.0 * 3.75 + 1.0,
        };
        let rect = Rectangle::whole_screen();
        assert_eq!((51, 112), rect.get_index((0, 0), &vector));
    }
    
    #[test]
    fn is_move_tree_test() {
        let x = 78;
        let y = 67;
        let origin_cat = positioned_cat((x as f64 + 0.5) * 5.0, (y as f64 + 0.5) * 3.75);
        let tree = QuadTree::new(&Vec::new());
        assert!(!tree.is_move_tree(&origin_cat.apply_velocity(&PVector{ x: 0.0, y: 0.0 })));
        for i in 0..8 {
            let vec = PVector {x: (i as f64 * 45.0).cos() * 5.0, y: (i as f64 * 45.0).sin() * 3.75 };
            assert!(tree.is_move_tree(&origin_cat.apply_velocity(&vec)));
        }
    }
}
