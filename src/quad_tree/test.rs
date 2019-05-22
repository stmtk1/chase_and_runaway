#[cfg(test)]
mod rectangle_tests{
    use quad_tree::{Rectangle, QuadTree};
    use consts::*;
    use animal::{Cat, Animal};
    use pvector::PVector;
    
    macro_rules! assert_float{
        (
            $x: expr ,$y: expr
        ) => {
            {
                assert!((($x - $y) / $x).abs() < 1.0e-9);
            }
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
    fn append_test(){
        let rect = sized_rect(1000.0, 2000.0);
        let mut tree = QuadTree::new_tree(&rect);
        let cat = positioned_cat(5.0, 10.0);
        tree.append(&cat);
    }
}
