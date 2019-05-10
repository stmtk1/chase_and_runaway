mod cat;
//mod test;
mod rat;

use pvector::PVector;

#[derive(Debug, Clone)]
pub struct Cat{
    position: PVector,
    velocity: PVector,
    pub chase_weight: f64,
    pub separate_weight: f64,
    pub align_weight: f64,
    pub cohension_weight: f64,
    ate: u32,
    energy: u64,
    id: u64,
}

#[derive(Clone)]
pub struct Rat{
    position: PVector,
    velocity: PVector,
    energy: u64,
    id: u64,
}

pub trait Animal : Clone {
    fn new() -> Self; // 初期化
    fn next_states(cats: &Vec<Cat>, rats: &Vec<Rat>) -> Vec<Self> where Self: std::marker::Sized; // １フレーム後の挙動
    fn move_self(&self) -> Self; // 速度ベクトル分だけ移動
    fn as_velocity(&self) -> PVector; // 速度ベクトルを返す
    fn apply_velocity(&self, &PVector) -> Self; // 速度ベクトルの変更
    fn is_within<T: Animal>(&self, other: &T, radious: f64) -> bool; // 一定半径以内にいるかどうか
    fn offset<T: Animal>(&self, other: &T) -> PVector; // ２匹の距離を計算
    fn collect_near_pvectors<T: Animal>(&self, animals: &Vec<T>, radious: f64) -> Vec<T>; // 一定半径以内にいるものを集める
    fn position(&self) -> PVector; // 現在の位置を返す
    fn calculate_direction<T: Animal>(&self, animals: Vec<T>) -> PVector; // 相対位置の平均を計算
    fn descendant(&self) -> Self; // 子孫。増殖のために使う
    fn life_manage(animals: &Vec<Self>) -> Vec<Self>; // 死んだ個体の削除、もしくは確率的に個体を増殖させる
    fn is_same<T: Animal>(&self, other: &T) -> bool; // 二つの個体が同じか
    fn id(&self) -> u64; // 個体の識別に使う
}
