#[cfg(test)]
mod tests{
    use pvector::PVector;
    fn is_near(a: f64, b: f64) -> bool {
        ((a - b) / b).abs() < 1.0e-9
    }
    
    #[test]
    fn new_test(){
        let x = 1.0;
        let y = 2.0;
        let vector = PVector::new(x, y);
        assert_eq!(vector.x, x);
        assert_eq!(vector.y, y);
    }
    
    #[test]
    fn len_test(){
        // 有名な直角間角形
        assert!(is_near(PVector::new(3.0, 4.0).len(), 5.0));
        // 直角二等辺三角形
        assert!(is_near(PVector::new(1.0, 1.0).len(), 2.0_f64.sqrt()));
        // 正三角形の半分
        assert!(is_near(PVector::new(1.0, 3.0_f64.sqrt()).len(), 2.0));
    }
    
    /*
     // TODO 画面を飛び越えて距離判定する
    #[test]
    fn offset_test(){
        let vec1 = PVector::new(1.0, 2.0);
        let vec2 = PVector::new(10.0, 20.0);
        assert_eq!(vec1.offset(&vec2), PVector::new(9.0, 18.0));
        assert_eq!(vec2.offset(&vec1), PVector::new(-9.0, -18.0));
        // 
    }
    */
    
    #[test]
    fn add_test(){
        let a = PVector::new(1.0, 2.0);
        let b = PVector::new(3.0, 4.0);
        assert_eq!(a.add(b), PVector::new(4.0, 6.0));
    }
    
    #[test]
    fn mult_test(){
        let v = PVector::new(1.0, 2.0);
        let scalar = 3.0;
        assert_eq!(PVector::new(3.0, 6.0), v.mult(scalar));
    }
    
    #[test]
    fn normalize_test(){
        // 有名な直角三角形
        let v = PVector::new(3.0, 4.0).normalize();
        assert!(is_near(0.6, v.x));
        assert!(is_near(0.8, v.y));
        let v = PVector::new(1.0, 1.0).normalize();
        assert!(is_near(0.5_f64.sqrt(), v.x));
        assert!(is_near(0.5_f64.sqrt(), v.y));
        let v = PVector::new(1.0, 3.0_f64.sqrt()).normalize();
        assert!(is_near(0.5_f64, v.x));
        assert!(is_near(0.75_f64.sqrt(), v.y));
    }
    
    #[test]
    fn zero_test() {
        let zero = PVector::zero();
        assert_eq!(zero.x, 0.0);
        assert_eq!(zero.y, 0.0);
    }
}
