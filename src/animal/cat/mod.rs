mod test;

use pvector::PVector;
use consts::*;
use animal::{Animal, Cat, Rat};
use rand::prelude::*;

impl Animal for Cat {
    // 初期化
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let velocity = PVector::new(theta.cos(), theta.sin()).mult(CAT_VELOCITY);
        let x = rng.gen::<f64>() * WIDTH;
        let y = rng.gen::<f64>() * HEIGHT;
        Cat {
            position: PVector::new(x, y),
            velocity: velocity, 
            chase_weight: rng.gen::<f64>() * CHASE_MAX,
            separate_weight: rng.gen::<f64>() * SEPARATE_MAX,
            align_weight: rng.gen::<f64>() * ALIGN_MAX,
            cohension_weight: rng.gen::<f64>() * COHENSION_MAX,
            energy: ENERGY_MAX,
            ate: 0,
            id: rng.gen::<u64>(),
        }
    }
    
    // １フレーム後の状態を返す
    fn next_states(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Self> {
        let ret: Vec<Cat> = cats
            .into_iter()
            .map(|cat| cat.chase(cats, rats))
            .collect();
        <Cat as Animal>::life_manage(&ret)
    }
    
    // 速度ベクトル分だけ動く
    fn move_self(&self) -> Cat {
        let mut new_pos = self.position().add(self.as_velocity());
        let mut ret = self.clone();
        
        if new_pos.x > WIDTH {
            new_pos.x -= WIDTH;
        }
        
        if new_pos.x < 0.0 {
            new_pos.x += WIDTH;
        }
        
        if new_pos.y > HEIGHT {
            new_pos.y -= HEIGHT;
        }
        
        if new_pos.y < 0.0 {
            new_pos.y += HEIGHT;
        }
        
        ret.energy -= 1;
        
        ret.position = new_pos;
        ret
    }
    
    // 速度ベクトルを返す
    fn as_velocity(&self) -> PVector {
        self.velocity.clone()
    }
    
    //速度ベクトルの変更
    fn apply_velocity(&self, pvector: &PVector) -> Self {
        let mut ret = self.clone();
        ret.velocity = pvector.clone();
        ret
    }
    
    // 一定半径以内にいるかどうか
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    // 相対位置の計算
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(&other_vec)
    }
    
    // 現在の位置
    fn position(&self) -> PVector{
        self.position.clone()
    }
    
    // 一定半径以内にいる個体を集める
    fn collect_near_pvectors<T: Animal>(&self, animals: &Vec<T>, radious: f64) -> Vec<T> {
        animals
            .into_iter()
            .filter(|animal| animal.is_within(self, radious))
            .filter(|animal| !animal.is_same(self))
            .map(|animal| animal.clone())
            .collect()
    }
    
    // 相対位置の平均を計算
    fn calculate_direction<T: Animal>(&self, animals: Vec<T>) -> PVector {
        animals
            .into_iter()
            .map(|animal| self.offset(&animal))
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    // 子孫
    fn descendant(&self) -> Self{
        let mut ret = Cat::new();
        ret.chase_weight = Cat::mutate(self.chase_weight, CHASE_MAX);
        ret.separate_weight = Cat::mutate(self.separate_weight, SEPARATE_MAX);
        ret.align_weight = Cat::mutate(self.align_weight, ALIGN_MAX);
        ret.cohension_weight = Cat::mutate(self.cohension_weight, COHENSION_MAX);
        ret.ate = 0;
        ret
    }
    
    // 死んだ個体の削除、および確率的に子孫を作成
    fn life_manage(animals: &Vec<Self>) -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut ret: Vec<Self> = Vec::new();
        for animal in animals {
            if animal.energy <= 0{
                continue;
            }
            if rng.gen::<f32>() < 1.0 / (ENERGY_MAX as f32) {
                ret.push(animal.clone().descendant());
            }
            ret.push(animal.clone());
        }
        ret
    }
    
     // 二つの個体が同じかどうかを判定
    fn is_same<T: Animal>(&self, other: &T) -> bool{
        self.id() == other.id()
    }
    
    // 個体の識別用
    fn id(&self) -> u64 {
        self.id
    }
}

impl Cat{
    // 加速度ベクトルを計算し、速度ベクトルに足す
    pub fn chase(&self, cats: &Vec<Cat>, rats: &Vec<Rat>) -> Cat{
        let next_velocity = self
            .as_velocity()
            .add(self.chase_vector(rats))
            .add(self.separate_same(cats))
            .add(self.align(cats))
            .add(self.cohension(cats))
            .normalize()
            .mult(self.velocity.len());
        
        self
            .apply_velocity(&next_velocity)
            .eat(rats)
            .move_self()
    }
    
    // 追いかける方向の計算
    fn chase_vector(&self, rats: &Vec<Rat>) -> PVector {
        let near_rats = self.collect_near_pvectors(rats, CHASE_RADIOUS);
        
        if near_rats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_rats)
            .mult(self.chase_weight)
    }
    
    // BOIDの個体同士を引き離す操作
    fn separate_same(&self, cats: &Vec<Cat>) -> PVector {
        let near_animal = self.collect_near_pvectors(cats, SEPARATE_RADIOUS);
        
        if near_animal.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animal)
            .mult(-1.0 * self.separate_weight)
    }
    
    // BOIDの整列処理
    fn align(&self, cats: &Vec<Cat>) -> PVector{
        let near_cats = self.collect_near_pvectors(cats, ALIGN_RADIOUS);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        self
            .add_velocity(&near_cats)
            .mult(self.align_weight)
    }
    
    // BOIDの個体が多い場所に行く操作
    fn cohension(&self, same_kind: &Vec<Cat>) -> PVector {
        let near_animals = self.collect_near_pvectors(same_kind, COHENSION_RADIOUS);
        
        if near_animals.len() <= 0 {
            return PVector::zero();
        }
        self
            .calculate_direction(near_animals)
            .mult(self.cohension_weight)
    }
    
    // 一定半径以内にいるなら食べる
    fn eat(&self, rats: &Vec<Rat>) -> Cat {
        let mut ret = self.clone();
        let can_eat = rats
            .into_iter()
            .any(|rat| self.is_within(rat, EATEN_RADIOUS));
        if can_eat {
            ret.energy += EAT_ENERGY;
            ret.ate += 1;
        }
        ret
    }
    
    // 整列処理のために近くにいる個体の速度ベクトルの平均をとる
    fn add_velocity(&self, animals: &Vec<Cat>) -> PVector {
        animals
            .into_iter()
            .map(|animal| animal.as_velocity())
            .fold(PVector::zero(), |folded, vector| vector.add(folded))
            .normalize()
    }
    
    // 子孫を残す時にパラメータを少し変化させる
    fn mutate(value: f64, value_max: f64) -> f64{
        let mut rng = rand::thread_rng();
        (rng.gen::<f64>() * MUTATE_ABS * 2.0 - MUTATE_ABS + value).min(value_max).max(0.0)
    }
    
    // 遺伝的アルゴリズムでたくさん食べた個体だけが次の世代で生き残る
    fn collect_servive(cats: &Vec<Cat>) -> Vec<Cat> {
        let len = cats.len();
        let mut ret = cats.clone();
        ret.sort_by(Cat::comp_by_ate);
         ret
            .into_iter()
            .take(len / 10)
            .collect()
    }
    
    // 次の世代に行く時に食べた順にソートする
    fn comp_by_ate(a1: &Cat, a2: &Cat) -> std::cmp::Ordering {
        if a1.ate < a2.ate {
            std::cmp::Ordering::Less
        } else if a1.ate > a2.ate {
            std::cmp::Ordering::Greater
        }else{
            std::cmp::Ordering::Equal
        }
    }
    
    // 生き残った個体を複製し、次の世代にする
    pub fn next_generation(cats: &Vec<Cat>) -> Vec<Cat>{
        let mut ret: Vec<Cat> = Vec::new();
        let superior = Cat::collect_servive(cats);
        while ret.len() < 20 {
            let mut appended = superior
                .clone()
                .into_iter()
                .map(|animal| animal.descendant())
                .collect();
            
            ret.append(&mut appended);
        }
        
        ret
            .into_iter()
            .take(20)
            .collect()
    }
}
