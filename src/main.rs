extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

mod animal;
mod pvector;
mod app;
mod consts;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::collections::LinkedList;
//use graphics::polygon;
use animal::Animal;
use consts::{WIDTH, HEIGHT};
use app::App;

fn main(){
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "spinning-square",
        [WIDTH as u32, HEIGHT as u32]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut cats: LinkedList<Animal> = LinkedList::new();
    for _ in 0..10 {
        cats.push_back(Animal::new_cat());
    }
    
    let mut rats: LinkedList<Animal> = LinkedList::new();
    for _ in 0..200 {
        rats.push_back(Animal::new_rat());
    }
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cats: cats,
        rats: rats,
    };
    
    let mut events = Events::new(EventSettings::new());
    
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args(){ 
            app.render(&r);
        }
        
        if let Some(_) = e.update_args() {
            app.update();
        }
    }
}

