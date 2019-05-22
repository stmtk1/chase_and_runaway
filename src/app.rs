use animal::{Animal, Cat, Rat};
use piston::input::RenderArgs;
use consts::{WIDTH, HEIGHT};
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{rectangle, clear};
use graphics::rectangle::square;
use graphics::Transformed;
use graphics::context::Context;
use piston::event_loop::*;
use piston::input::*;
use quad_tree::QuadTree;

// #[derive(Clone)]
pub struct App {
    pub gl: GlGraphics,
    pub window: Window,
    pub cats: Vec<Cat>,
    pub rats: Vec<Rat>,
    pub cats_tree: QuadTree<Cat>,
    pub rats_tree: QuadTree<Rat>,
}

impl App {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const ANIMAL_SIZE: f64 = 5.0;
    // 初期化
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window = App::new_window(opengl);
        let cats = App::new_cats();
        let rats = App::new_rats();
        let cats_tree = QuadTree::new(&cats);
        let rats_tree = QuadTree::new(&rats);
        
        App {
            gl: GlGraphics::new(opengl),
            window, cats, rats, cats_tree, rats_tree,
        }
    }
    
    // １世代目に捕食者の初期化
    fn new_cats() -> Vec<Cat> {
        let mut ret: Vec<Cat> = Vec::new();
        for _ in 0..10 {
            ret.push(<Cat as Animal>::new());
        }
        ret
    }
    
    // 世代の初めに捕食者の初期化
    fn new_rats() -> Vec<Rat> {
        let mut ret: Vec<Rat> = Vec::new();
        for _ in 0..200 {
            ret.push(<Rat as Animal>::new());
        }
        ret
    }
    
    // ウィンドウの初期化
    fn new_window(opengl: OpenGL) -> Window{
        WindowSettings::new(
                "spinning-square",
                [WIDTH as u32, HEIGHT as u32]
            )
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap()
    }
    
    // 優秀な捕食者だけを次の世代へ
    pub fn next_generation(&mut self){
        self.cats = Cat::next_generation(&self.cats);
        self.rats = App::new_rats();
    }
    
    // １世代終わるまでウインドウを表示
    pub fn show_window(&mut self){
        let mut events = Events::new(EventSettings::new());
        
        while let Some(e) = events.next(&mut self.window) {
            if let Some(r) = e.render_args(){ 
                self.render(&r);
            }
            
            if let Some(_) = e.update_args() {
                if self.update(){
                    break
                }
            }
        }
    }
    
    // 描画
    pub fn render(&mut self, args: &RenderArgs){
        
        let square: graphics::types::Rectangle = square(0.0, 0.0, App::ANIMAL_SIZE);

        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];

        let cats = &self.cats;
        let rats = &self.rats;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(App::GREEN, gl);
            
            App::draw_cat(&c, gl, cats, square);
            App::draw_rat(&c, gl, rats, square);

        });
    }
    
    // 捕食者の描画
    fn draw_cat(c: &Context, gl: &mut GlGraphics, cats: &Vec<Cat>, square: graphics::types::Rectangle) {
        for cat in cats {
            let transform = c.transform
                .trans(cat.position().x, cat.position().y);
            rectangle(App::RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    // 日捕食者の描画
    fn draw_rat(c: &Context, gl: &mut GlGraphics, rats: &Vec<Rat>, square: graphics::types::Rectangle) {
        for rat in rats {
            let transform = c.transform
                .trans(rat.position().x, rat.position().y);
            rectangle(App::BLUE, square, transform, gl);
            //polygon(BLUE, &TRIANGLE, transform, gl);
        }
    }
    
    // 非捕食者がいなくなったら世代が終わる
    fn is_finished(rats: &Vec<Rat>) -> bool {
        rats.len() == 0
    }
    
    // 捕食者のパラメータの平均の計算
    fn chase_average(animals: &Vec<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.chase_weight)
            / animals.len() as f64
    }
    
    // 捕食者のパラメータの平均の計算
    fn align_average(animals: &Vec<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.align_weight)
            / animals.len() as f64
    }
    
    // 捕食者のパラメータの平均の計算
    fn separate_average(animals: &Vec<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.separate_weight)
            / animals.len() as f64
    }
    
    // 捕食者のパラメータの平均の計算
    fn cohension_average(animals: &Vec<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.cohension_weight) 
            / animals.len() as f64
    }

    // １フレーム進める
    pub fn update(&mut self)  -> bool {
        let cats = self.cats.clone();
        let rats = self.rats.clone();
        self.cats = <Cat as Animal>::next_states(&cats, &self.cats_tree, &self.rats_tree);
        self.rats = <Rat as Animal>::next_states(&rats, &self.cats_tree ,&self.rats_tree);
        self.cats_tree = QuadTree::new(&self.cats);
        self.rats_tree = QuadTree::new(&self.rats);
        App::is_finished(&self.rats)
    }
    
    // 世代の最後にパラメータを標準出力へ
    pub fn print_params(&self, ord: i32) {
        print!("{},", ord);
        print!("{},", App::chase_average(&self.cats));
        print!("{},", App::align_average(&self.cats));
        print!("{},", App::cohension_average(&self.cats));
        print!("{}",  App::separate_average(&self.cats));
        println!("");
    }
}
