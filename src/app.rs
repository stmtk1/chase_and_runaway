use animal::Animal;
use piston::input::RenderArgs;
use consts::{WIDTH, HEIGHT};
use glutin_window::GlutinWindow as Window;
//use opengl_graphics::GlGraphics;
//use graphics::*;
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
    pub cats: LinkedList<Animal>,
    pub rats: LinkedList<Animal>,
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
    
    fn new_cats() -> LinkedList<Animal> {
        let mut ret: LinkedList<Animal> = LinkedList::new();
        for _ in 0..10 {
            ret.push_back(Animal::new_cat());
        }
        ret
    }
    
    fn new_rats() -> LinkedList<Animal> {
        let mut ret: LinkedList<Animal> = LinkedList::new();
        for _ in 0..200 {
            ret.push_back(Animal::new_rat());
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

    fn draw_cat(c: &Context, gl: &mut GlGraphics, cats: &LinkedList<Animal>, square: graphics::types::Rectangle) {
        for cat in cats {
            let transform = c.transform
                .trans(cat.x, cat.y);
            rectangle(App::RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    
    fn draw_rat(c: &Context, gl: &mut GlGraphics, rats: &LinkedList<Animal>, square: graphics::types::Rectangle) {
        for rat in rats {
            let transform = c.transform
                .trans(rat.x, rat.y);
            rectangle(App::BLUE, square, transform, gl);
            //polygon(BLUE, &TRIANGLE, transform, gl);
        }
    }

    fn is_finished(rats: &LinkedList<Animal>) -> bool {
        rats.len() == 0
    }

    pub fn update(&mut self)  -> bool {
        let cats = self.cats.clone();
        let rats = self.rats.clone();
        self.cats = Animal::next_states_cats(&cats, &rats);
        self.rats = Animal::next_states_rats(&cats, &rats);
        App::is_finished(&rats)
    }
}
