#[cfg(test)]
mod tests{
    use animal::{Animal, Cat, Rat};
    use consts::*;
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
    
    fn setpos(animal: &mut Cat, pos: &PVector){
        animal.position = pos.clone();
    }
    
    #[test]
    fn cat_new_test(){
        for _ in 0..100{
            let cat = <Cat as Animal>::new();
            let vel_size = cat.velocity.len();
            assert!(0.0 < cat.position().x && cat.position().x < WIDTH);
            assert!(0.0 < cat.position().y && cat.position().y < HEIGHT);
            assert_float!(cat.velocity.len(), CAT_VELOCITY);
            assert_float!(vel_size, CAT_VELOCITY);
            assert!(0.0 < cat.chase_weight && cat.chase_weight <  CHASE_MAX);
            assert!(0.0 < cat.separate_weight && cat.separate_weight <  SEPARATE_MAX);
            assert!(0.0 < cat.align_weight && cat.align_weight <  ALIGN_MAX);
            assert!(0.0 < cat.cohension_weight && cat.cohension_weight <  COHENSION_MAX);
            assert_eq!(cat.energy, ENERGY_MAX);
            assert_eq!(cat.ate, 0);
        }
    }
    
    #[test]
    fn cat_move_self_test(){
        let x = 50.0;
        let y = 100.0;
        let vx = 4.0;
        let vy = 2.0;
        let energy = 100;
        let mut cat = <Cat as Animal>::new();
        
        setpos(&mut cat, &PVector{x: x, y: y});
        cat = cat.apply_velocity(&PVector{x: vx, y: vy});
        
        cat.energy = energy;
        
        cat = cat.move_self();
        
        assert_eq!(energy - 1, cat.energy);
        // 画面の外にはみ出さない場合
        assert_eq!(cat.position(), PVector::new(x + vx, y + vy));
        
        // 画面の右側にはみ出す場合
       cat.position.x = WIDTH - vx + 1.0;
       assert_eq!(1.0, cat.move_self().position().x);
       
        // 画面の左側にはみ出す場合
       cat.velocity.x *= -1.0;
       cat.position.x = 1.0;
       assert_eq!(WIDTH - vx + 1.0, cat.move_self().position().x);
       
        // 画面の下側にはみ出す場合
        cat.position.y = HEIGHT - vy + 1.0;
        assert_eq!(1.0, cat.move_self().position().y);
        
        // 画面の上側にはみ出す場合
       cat.velocity.y *= -1.0;
       cat.position.y = 1.0;
       assert_eq!(HEIGHT - vy + 1.0, cat.move_self().position().y);
    }
    
    #[test]
    fn cat_as_velocity_test(){
        let cat = <Cat as Animal>::new();
        let vel = cat.as_velocity();
        assert_eq!(cat.velocity.x, vel.x);
        assert_eq!(cat.velocity.y, vel.y);
    }
    
    #[test]
    fn cat_apply_velocity_test(){
        let vel = PVector{x: 100.0, y: 200.0};
        let cat = <Cat as Animal>::new().apply_velocity(&vel);
        assert_eq!(cat.velocity.x, vel.x);
        assert_eq!(cat.velocity.y, vel.y);
    }
    
    #[test]
    fn cat_within_test(){
        let cat1 = <Cat as Animal>::new();
        let mut cat2 = <Cat as Animal>::new();
        let diff = 1.0;
        setpos(&mut cat2, &cat1.position().add(PVector::new(diff, 0.0)));
        assert!(cat1.is_within(&cat2, diff + 0.5));
        assert!(!cat1.is_within(&cat2, diff - 0.5));
    }

    #[test]
    fn cat_offset_test(){
        let cat1 = <Cat as Animal>::new();
        let mut cat2 = <Cat as Animal>::new();
        let diff = PVector::new(1.0, 2.0);
        setpos(&mut cat2, &cat1.position().add(diff));
        let offset = cat1.offset(&cat2);
        assert_float!(offset.x, offset.x);
        assert_float!(offset.y, offset.y);
    }
    
    #[test]
    fn cat_postion_test(){
        let cat = <Cat as Animal>::new();
        let pos = cat.position();
        assert_eq!(cat.position(), pos);
    }
    
    #[test]
    fn cat_id_test(){
        let cat = <Cat as Animal>::new();
        assert_eq!(cat.id, cat.id());
    }
    
    #[test]
    fn cat_is_same_test(){
        let cat1 = <Cat as Animal>::new();
        let cat2 = <Cat as Animal>::new();
        assert!(cat1.is_same(&cat1));
        assert!(!cat1.is_same(&cat2));
    }
    
    #[test]
    fn cat_descendant_test(){
        let parent = <Cat as Animal>::new();
        for _ in 0..100 {
            let child = parent.descendant();
            assert!((parent.chase_weight - child.chase_weight).abs() < MUTATE_ABS);
            assert!((parent.separate_weight - child.separate_weight).abs() < MUTATE_ABS);
            assert!((parent.align_weight - child.align_weight).abs() < MUTATE_ABS);
            assert!((parent.cohension_weight - child.cohension_weight).abs() < MUTATE_ABS);
        }
    }
    
    #[test]
    fn cat_life_manage_test(){
        // 全員死んでいるので、消される
        let mut dead = <Cat as Animal>::new();
        dead.energy = 0;
        let mut dead_cats: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100{
            dead_cats.push(dead.clone());
        }
        let expect_none = Cat::life_manage(&dead_cats);
        assert_eq!(expect_none.len(), 0);
        
        // 一定の確率で子孫が誕生するので元々の数より多くなる
        let mut all_alive: Vec<Cat> = Vec::with_capacity(100000);
        for _ in 0..100000{
            all_alive.push(<Cat as Animal>::new());
        }
        let more_than_handret = Cat::life_manage(&all_alive);
        assert!(more_than_handret.len() > 100000);
    }
    
    #[test]
    fn cat_calclate_direction_test(){
        let dx = 0.6;
        let dy = 0.8;
        let cat = <Cat as Animal>::new();
        let mut other = <Cat as Animal>::new();
        setpos(&mut other, &PVector::new(dx, dy).add(cat.position()));
        let mut arg = Vec::with_capacity(100);
        for _ in 0..100{
            arg.push(other.clone());
        }
        let result = cat.calculate_direction(arg);
        assert_float!(dx, result.x);
        assert_float!(dy, result.y);
    }
    
    
    #[test]
    fn cat_collect_near_pvectors_test(){
        let cat = <Cat as Animal>::new();
        let diff = 1.0;
        // 近くにいない場合
        let mut other = <Cat as Animal>::new();
        setpos(&mut other, &PVector::new(diff,diff).add(cat.position()));
        let mut cats: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100{
            cats.push(other.clone());
        }
        let expect_none = cat.collect_near_pvectors(&cats, 1.0);
        assert_eq!(expect_none.len(), 0);
        
        //全部一定半径内にいる場合 
        let not_dicrease = cat.collect_near_pvectors(&cats, 2.0);
        assert!(not_dicrease.len() == 100);
    }
    
    #[test]
    fn add_velocity(){
        let cat = <Cat as Animal>::new();
        let mut other = <Cat as Animal>::new();
        let x = 0.6;
        let y = 0.8;
        other = other.apply_velocity(&PVector::new(x, y));
        let mut cats: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100 {
            cats.push(other.clone());
        }
        
        let ret = cat.add_velocity(&cats);
        assert_float!(x, ret.x);
        assert_float!(y, ret.y);
    }
    
    #[test]
    fn cat_eat_test(){
        let mut cat = <Cat as Animal>::new();
        let rat = <Rat as Animal>::new();
        let eaten_diff = EATEN_RADIOUS / 2.0;
        let mut offset = PVector::new(eaten_diff, eaten_diff);
        setpos(&mut cat, &rat.position.add(offset));
        let mut eaten: Vec<Rat> = Vec::with_capacity(100);
        for _ in 0..100 {
            eaten.push(rat.clone());
        }
        let result = cat.eat(&eaten);
        assert_eq!(cat.energy + EAT_ENERGY, result.energy);
        assert_eq!(result.ate, 1);
        
        let not_eat_diff = EATEN_RADIOUS;
        offset = PVector::new(not_eat_diff, not_eat_diff);
        setpos(&mut cat, &rat.position.add(offset));
        
        let not_eat = cat.eat(&eaten);
        assert_eq!(cat.energy, not_eat.energy);
        assert_eq!(not_eat.ate, 0);
    }
    
    #[test]
    fn cat_chase_vector_test(){
        let mut cat = <Cat as Animal>::new();
        let rat = <Rat as Animal>::new();
        let chased_diff = CHASE_RADIOUS / 2.0;
        let dx = 0.6;
        let dy = 0.8;
        let mut offset = PVector::new(-1.0 * dx * chased_diff, -1.0 * dy * chased_diff);
        setpos(&mut cat, &rat.position().add(offset));
        let mut chased: Vec<Rat> = Vec::with_capacity(100);
        for _ in 0..100 {
            chased.push(rat.clone());
        }
        let result = cat.chase_vector(&chased);
        
        assert_float!(dx * cat.chase_weight, result.x);
        assert_float!(dy * cat.chase_weight, result.y);
        
        let not_chase_diff = CHASE_RADIOUS;
        offset = PVector::new(-0.6 * not_chase_diff, -0.8 * not_chase_diff);
        setpos(&mut cat, &rat.position().add(offset));
        
        let not_chase = cat.chase_vector(&chased);
        assert_eq!(not_chase.x, 0.0);
        assert_eq!(not_chase.y, 0.0);
    }
    
    #[test]
    fn cat_separate_same_test(){
        let mut cat = <Cat as Animal>::new();
        let other = <Cat as Animal>::new();
        let separate_diff = SEPARATE_RADIOUS / 2.0;
        let x = 0.6;
        let y = 0.8;
        setpos(&mut cat, &PVector::new(separate_diff * x, separate_diff * y).add(other.position()));
        let mut others: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100 {
            others.push(other.clone());
        }
        let result = cat.separate_same(&others);
        
        assert_float!(x * cat.separate_weight, result.x);
        assert_float!(y * cat.separate_weight, result.y);
        
        let not_separate_diff = SEPARATE_RADIOUS;
        setpos(&mut cat, &PVector::new(not_separate_diff, not_separate_diff).add(other.position()));
        
        let not_separate = cat.separate_same(&others);
        assert_eq!(not_separate.x, 0.0);
        assert_eq!(not_separate.y, 0.0);
    }
    
    #[test]
    fn cat_align_test(){
        let mut cat = <Cat as Animal>::new();
        let align_diff = ALIGN_RADIOUS / 2.0;
        let x = 0.6;
        let y = 0.8;
        let other = <Cat as Animal>::new().apply_velocity(&PVector{x: x, y: y});
        setpos(&mut cat, &PVector::new(-align_diff, -align_diff).add(other.position()));
        let mut others: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100 {
            others.push(other.clone());
        }
        let result = cat.align(&others);
        
        assert_float!(x * cat.align_weight, result.x);
        assert_float!(y * cat.align_weight, result.y);
        
        let not_align_diff = ALIGN_RADIOUS;
        setpos(&mut cat, &PVector::new(-not_align_diff, -not_align_diff).add(other.position()));
        
        let not_align = cat.align(&others);
        assert_eq!(not_align.x, 0.0);
        assert_eq!(not_align.y, 0.0);
    }
    
    #[test]
    fn cat_cohension_test(){
        let mut cat = <Cat as Animal>::new();
        let other = <Cat as Animal>::new();
        let cohension_diff = COHENSION_RADIOUS / 2.0;
        let x = 0.6;
        let y = 0.8;
        setpos(&mut cat, &PVector::new(-cohension_diff * x, -cohension_diff * y).add(other.position()));
        let mut others: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100 {
            others.push(other.clone());
        }
        let result = cat.cohension(&others);
        
        assert_float!(x * cat.cohension_weight, result.x);
        assert_float!(y * cat.cohension_weight, result.y);
        
        let not_cohension_diff = COHENSION_RADIOUS;
        setpos(&mut cat, &PVector::new(-not_cohension_diff, -not_cohension_diff).add(other.position()));
        
        let not_cohension = cat.cohension(&others);
        assert_eq!(not_cohension.x, 0.0);
        assert_eq!(not_cohension.y, 0.0);
    }
}
