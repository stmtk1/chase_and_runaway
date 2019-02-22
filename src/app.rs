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
use std::collections::LinkedList;
use piston::event_loop::*;
use piston::input::*;

// #[derive(Clone)]
pub struct App {
    pub gl: GlGraphics,
    pub window: Window,
    pub cats: LinkedList<Cat>,
    pub rats: LinkedList<Rat>,
}

impl App {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const ANIMAL_SIZE: f64 = 5.0;
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window = App::new_window(opengl);
        App {
            gl: GlGraphics::new(opengl),
            window: window,
            cats: App::new_cats(),
            rats: App::new_rats(),
        }
    }
    
    fn new_cats() -> LinkedList<Cat> {
        let mut ret: LinkedList<Cat> = LinkedList::new();
        for _ in 0..10 {
            ret.push_back(<Cat as Animal>::new());
        }
        ret
    }
    
    fn new_rats() -> LinkedList<Rat> {
        let mut ret: LinkedList<Rat> = LinkedList::new();
        for _ in 0..200 {
            ret.push_back(<Rat as Animal>::new());
        }
        ret
    }
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
    
    pub fn next_generation(&mut self){
        self.cats = Cat::next_generation(&self.cats);
        self.rats = App::new_rats();
        /*
        App {
            gl: self.gl,
            window: self.window,
            cats: Cat::next_generation(&self.cats),
            rats: App::new_rats(),
        }
        */
    }
    
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
    

    fn draw_cat(c: &Context, gl: &mut GlGraphics, cats: &LinkedList<Cat>, square: graphics::types::Rectangle) {
        for cat in cats {
            let transform = c.transform
                .trans(cat.x, cat.y);
            rectangle(App::RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    fn draw_rat(c: &Context, gl: &mut GlGraphics, rats: &LinkedList<Rat>, square: graphics::types::Rectangle) {
        for rat in rats {
            let transform = c.transform
                .trans(rat.x, rat.y);
            rectangle(App::BLUE, square, transform, gl);
            //polygon(BLUE, &TRIANGLE, transform, gl);
        }
    }

    fn is_finished(rats: &LinkedList<Rat>) -> bool {
        rats.len() == 0
    }
    
    fn chase_average(animals: &LinkedList<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.chase_weight)
            / animals.len() as f64
    }
    
    fn align_average(animals: &LinkedList<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.align_weight)
            / animals.len() as f64
    }
    
    fn separate_average(animals: &LinkedList<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.separate_weight)
            / animals.len() as f64
    }
    
    fn cohension_average(animals: &LinkedList<Cat>) -> f64 {
        animals
            .into_iter()
            .fold(0.0, |a, b| a + b.cohension_weight) 
            / animals.len() as f64
    }

    pub fn update(&mut self)  -> bool {
        let cats = self.cats.clone();
        let rats = self.rats.clone();
        self.cats = <Cat as Animal>::next_states(&cats, &rats);
        self.rats = <Rat as Animal>::next_states(&cats, &rats);
        App::is_finished(&self.rats)
    }
    
    fn max(a: f64, b: f64) -> f64 {
        if a < b {
            a
        }else{
            b
        }
    }
    
    pub fn print_params(&self, ord: i32) {
        print!("{},", ord);
        print!("{},", App::chase_average(&self.cats));
        print!("{},", App::align_average(&self.cats));
        print!("{},", App::cohension_average(&self.cats));
        print!("{}",  App::separate_average(&self.cats));
        println!("");
    }
}
