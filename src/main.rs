extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

mod animal;
mod pvector;
mod app;
mod consts;

use app::App;

fn main(){
    let mut app = App::new();
    app.show_window();
    
}
