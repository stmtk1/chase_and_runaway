#[cfg(test)]
mod tests{
    use animal::{Animal, Cat, Rat};
    use consts::*;
    use pvector::PVector;
    
    fn setpos(animal: &mut Rat, pos: &PVector){
        animal.position = pos.clone();
    }
    
    macro_rules! assert_float{
        (
            $x: expr ,$y: expr
        ) => {
            {
                assert!((($x - $y) / $x).abs() < 1.0e-9);
            }
        }
    }
    
    #[test]
    fn rat_new_test(){
        for _ in 0..100{
            let rat = <Rat as Animal>::new();
            let vel_size = rat.velocity.len();
            assert!(0.0 < rat.position.x && rat.position.x < WIDTH);
            assert!(0.0 < rat.position.y && rat.position.y < HEIGHT);
            assert_float!(rat.velocity.len(), RAT_VELOCITY);
            assert_float!(vel_size, RAT_VELOCITY);
            assert_eq!(rat.energy, ENERGY_MAX);
        }
    }
    
    #[test]
    fn rat_move_self_test(){
        let position = PVector::new(50.0, 100.0);
        let velocity = PVector::new(4.0, 2.0);
        let x = 50.0;
        let y = 100.0;
        let vx = 4.0;
        let vy = 2.0;
        let energy = 100;
        let mut rat = <Rat as Animal>::new();
        
        setpos(&mut rat, &position);
        rat = rat.apply_velocity(&velocity);
        
        rat.energy = energy;
        
        rat = rat.move_self();
        
        // 画面の外にはみ出さない場合
        assert_eq!(x + vx, rat.position.x);
        assert_eq!(y + vy, rat.position.y);
        
        // 画面の右側にはみ出す場合
       rat.position.x = WIDTH - vx + 1.0;
       assert_eq!(1.0, rat.move_self().position.x);
       
        // 画面の左側にはみ出す場合
       rat.velocity.x *= -1.0;
       rat.position.x = 1.0;
       assert_eq!(WIDTH - vx + 1.0, rat.move_self().position.x);
       
        // 画面の下側にはみ出す場合
        rat.position.y = HEIGHT - vy + 1.0;
        assert_eq!(1.0, rat.move_self().position.y);
        
        // 画面の上側にはみ出す場合
       rat.velocity.y *= -1.0;
       rat.position.y = 1.0;
       assert_eq!(HEIGHT - vy + 1.0, rat.move_self().position.y);
    }
    
    #[test]
    fn rat_as_velocity_test(){
        let rat = <Rat as Animal>::new();
        let vel = rat.as_velocity();
        assert_eq!(rat.velocity.x, vel.x);
        assert_eq!(rat.velocity.y, vel.y);
    }
    
    #[test]
    fn rat_apply_velocity_test(){
        let vel = PVector{x: 100.0, y: 200.0};
        let rat = <Rat as Animal>::new().apply_velocity(&vel);
        assert_eq!(rat.velocity.x, vel.x);
        assert_eq!(rat.velocity.y, vel.y);
    }
    
    #[test]
    fn rat_within_test(){
        let rat1 = <Rat as Animal>::new();
        let mut rat2 = <Rat as Animal>::new();
        let diff = 1.0;
        let offset = PVector::new(diff, 0.0);
        rat2.position = rat1.position.add(offset);
        assert!(rat1.is_within(&rat2, diff + 0.5));
        assert!(!rat1.is_within(&rat2, diff - 0.5));
    }

    #[test]
    fn rat_offset_test(){
        let rat1 = <Rat as Animal>::new();
        let mut rat2 = <Rat as Animal>::new();
        let dx = 1.0;
        let dy = 2.0;
        let offset = PVector::new(dx, dy);
        setpos(&mut rat2, &rat1.position.add(offset));
        let offset = rat1.offset(&rat2);
        assert_eq!(dx, offset.x);
        assert_eq!(dy, offset.y);
    }
    
    #[test]
    fn rat_postion_test(){
        let rat = <Rat as Animal>::new();
        let pos = rat.position();
        assert_eq!(rat.position.x, pos.x);
        assert_eq!(rat.position.y, pos.y);
    }
    
    #[test]
    fn rat_id_test(){
        let rat = <Rat as Animal>::new();
        assert_eq!(rat.id, rat.id());
    }
    
    #[test]
    fn rat_is_same_test(){
        let rat1 = <Rat as Animal>::new();
        let rat2 = <Rat as Animal>::new();
        assert!(rat1.is_same(&rat1));
        assert!(!rat1.is_same(&rat2));
    }
    
    #[test]
    fn rat_descendant_test(){
        let parent = <Rat as Animal>::new();
        for _ in 0..100 {
            let child = parent.descendant();
            assert_float!(parent.velocity.len(), child.velocity.len());
            // TODO vxとvyのテスト
            assert_ne!(parent.position.x, child.position.x);
            assert_ne!(parent.position.y, child.position.y);
        }
    }
    
    #[test]
    fn rat_life_manage_test(){
        // 全員死んでいるので、消される
        let mut dead = <Rat as Animal>::new();
        dead.energy = 0;
        let mut dead_rats: Vec<Rat> = Vec::with_capacity(100);
        for _ in 0..100{
            dead_rats.push(dead.clone());
        }
        let expect_none = Rat::life_manage(&dead_rats);
        assert_eq!(expect_none.len(), 0);
        
        // 一定の確率で子孫が誕生するので元々の数より多くなる
        let mut all_alive: Vec<Rat> = Vec::with_capacity(100000);
        for _ in 0..100000{
            all_alive.push(<Rat as Animal>::new());
        }
        let more_than_handret = Rat::life_manage(&all_alive);
        assert!(more_than_handret.len() > 100000);
    }
    
    #[test]
    fn rat_calclate_direction_test(){
        let dx = 0.6;
        let dy = 0.8;
        let rat = <Rat as Animal>::new();
        let mut other = <Rat as Animal>::new();
        let mut offset = PVector::new(dx, dy);
        setpos(&mut other, &rat.position.add(offset));
        let mut arg = Vec::with_capacity(100);
        for _ in 0..100{
            arg.push(other.clone());
        }
        let result = rat.calculate_direction(arg);
        assert_float!(dx, result.x);
        assert_float!(dy, result.y);
    }
    
    
    #[test]
    fn rat_collect_near_pvectors_test(){
        let rat = <Rat as Animal>::new();
        let diff = 1.0;
        // 近くにいない場合
        let mut other = <Rat as Animal>::new();
        let offset = PVector::new(diff, diff);
        setpos(&mut other, &rat.position.add(offset));
        let mut rats: Vec<Rat> = Vec::with_capacity(100);
        for _ in 0..100{
            rats.push(other.clone());
        }
        let expect_none = rat.collect_near_pvectors(&rats, 1.0);
        assert_eq!(expect_none.len(), 0);
        
        //全部一定半径内にいる場合 
        let not_dicrease = rat.collect_near_pvectors(&rats, 2.0);
        assert!(not_dicrease.len() == 100);
    }
    
    #[test]
    fn rat_runaway_vector_test(){
        let mut rat = <Rat as Animal>::new();
        let cat = <Cat as Animal>::new();
        let runaway_diff = RUNAWAY_RADIOUS / 2.0;
        let x = 0.6;
        let y = 0.8;
        setpos(&mut rat, &PVector::new(runaway_diff * x, runaway_diff * y).add(cat.position()));
        let mut cats: Vec<Cat> = Vec::with_capacity(100);
        for _ in 0..100 {
            cats.push(cat.clone());
        }
        let result = rat.run_away_vector(&cats);
        
        assert_float!(x, result.x);
        assert_float!(y, result.y);
        
        let not_chase_diff = CHASE_RADIOUS;
        setpos(&mut rat, &PVector::new(not_chase_diff, not_chase_diff).add(cat.position()));
        
        let not_chase = rat.run_away_vector(&cats);
        assert_eq!(not_chase.x, 0.0);
        assert_eq!(not_chase.y, 0.0);
    }
    
    #[test]
    fn rat_eaten_test(){
        let mut rat = <Rat as Animal>::new();
        let cat = <Cat as Animal>::new();
        let eaten_diff = EATEN_RADIOUS / 2.0;
        setpos(&mut rat, &PVector::new(eaten_diff, eaten_diff).add(cat.position()));
        let mut cats: Vec<Cat> = Vec::with_capacity(1);
        cats.push(cat.clone());
        
        let not_eaten_diff = EATEN_RADIOUS;
        setpos(&mut rat, &PVector::new(not_eaten_diff, not_eaten_diff).add(cat.position()));
        assert!(!rat.eaten(&cats));
    }
    
    #[test]
    fn rat_delete_eaten_test(){
        let mut rat = <Rat as Animal>::new();
        let cat = <Cat as Animal>::new();
        let eaten_diff = EATEN_RADIOUS / 2.0;
        setpos(&mut rat, &PVector::new(eaten_diff, eaten_diff).add(cat.position()));
        let mut cats: Vec<Cat> = Vec::with_capacity(1);
        cats.push(cat.clone());
        let mut rats: Vec<Rat> = Vec::with_capacity(100);
        for _ in 0..100 {
            rats.push(rat.clone());
        }
        assert_eq!(Rat::delete_eaten(&cats, &rats).len(), 0);
        
        let not_eaten_diff = EATEN_RADIOUS;
        setpos(&mut rat, &PVector::new(not_eaten_diff, not_eaten_diff).add(cat.position()));
        rats.clear();
        for _ in 0..100 {
            rats.push(rat.clone());
        }
        
        assert_eq!(Rat::delete_eaten(&cats, &rats).len(), 100);
    }
}
