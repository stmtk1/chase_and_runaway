extern crate rand;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate graphics;

mod animal;
mod pvector;
mod app;
mod consts;
mod quad_tree;
//mod kd_tree;

use app::App;

fn main(){
        print!("gen,");
        print!("chase,");
        print!("align,");
        print!("cohension,");
        print!("separate");
        println!("");
    let mut app = App::new();
    for i in 1..100 {
        app.show_window();
        app.print_params(i);
        app.next_generation();
    }
}
