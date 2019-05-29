mod test;

use pvector::PVector;
use animal::{Animal, Rat, Cat};
use consts::*;
use rand::prelude::*;
use quad_tree::{QuadTree, Rectangle};


impl Animal for Rat {
    // インスタンス初期化
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let theta: f64 = rng.gen::<f64>() * 2.0 * (std::f64::consts::PI);
        let x = rng.gen::<f64>() * WIDTH;
        let y = rng.gen::<f64>() * HEIGHT;
        let velocity = RAT_VELOCITY;
        Rat {
            position: PVector::new(x, y),
            velocity: PVector::new(theta.cos(), theta.sin()).mult(velocity),
            energy: ENERGY_MAX,
            id: rng.gen::<u64>(),
        }
    }
    
    // 次のフレームの計算
    fn next_states(rats: &Vec<Self>, cats_tree: &QuadTree<Cat>, _rats_tree: &QuadTree<Rat>) -> Vec<Self> {
        let alive_rats = Rat::delete_eaten(cats_tree, rats);
        let ret = alive_rats
            .into_iter()
            .map(|rat| rat.run_away(cats_tree))
            .collect();
        <Rat as Animal>::life_manage(&ret)
    }
    
    // 現在位置に速度ベクトルを足す
    fn move_self(&self) -> Rat {
        let mut ret = self.clone();
        let mut new_pos = ret.position.add(self.clone().velocity);
        
        // 画面からはみ出た時の操作
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
        
        ret.position = new_pos;
        ret
    }
    
    // 速度ベクトルを返す
    fn as_velocity(&self) -> PVector {
        self.velocity.clone()
    }
    
    // 速度ベクトルの変更
    fn apply_velocity(&self, pvector: &PVector) -> Self {
        let mut ret = self.clone();
        ret.velocity = pvector.clone();
        ret
    }
    
    // 一定半径にいるかどうか
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool {
        self.offset(other).len() < radious
    }
    
    // 現在位置を返す
    fn position(&self) -> PVector{
        self.position.clone()
    }
    
    // 相対位置の計算
    fn offset<T: Animal>(&self, other: &T) -> PVector {
        let self_vec = self.position();
        let other_vec = other.position();
        self_vec.offset(&other_vec)
    }
    
    // 近くにいる個体を集める
    fn collect_near_pvectors<T: Animal>(&self, animals: &QuadTree<T>, radious: f64) -> Vec<T> {
        animals
            .search(self, radious)
            .into_iter()
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
        let mut ret = Rat::new();
        ret.energy = ENERGY_MAX;
        ret
    }
    
    //個体識別のためのID
    fn id(&self) -> u64 {
        self.id
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
    
    // 二つの個体が同じか識別
    fn is_same<T: Animal>(&self, other: &T) -> bool{
        self.id() == other.id()
    }
}

impl Rat {
    // 1個体の次の状態
    fn run_away(&self, cats_tree: &QuadTree<Cat>) -> Rat {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(cats_tree))
            .normalize()
            .mult(self.velocity.len());
        self
            .apply_velocity(&next_velocity)
            //.move_self()
    }
    
    // 逃げる方向をか速度ベクトルにする
    fn run_away_vector(&self, cats_tree: &QuadTree<Cat>) -> PVector {
        let near_cats = self.collect_near_pvectors(cats_tree, RUNAWAY_RADIOUS);
        
        if near_cats.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_cats)
            .mult(-1.0)
    }
    
    // 食べられているかどうかを判定
    fn eaten(&self, cats_tree: &QuadTree<Cat>) -> bool{
        cats_tree.search(self, 1.0).len() > 0
    }
    
    // 食べられたらいなくなる
    fn delete_eaten(cats_tree: &QuadTree<Cat>, rats: &Vec<Rat>) -> Vec<Rat> {
        rats
            .into_iter()
            .filter(|rat| !rat.eaten(cats_tree))
            .map(|rat| rat.clone())
            .collect()
    }
}
