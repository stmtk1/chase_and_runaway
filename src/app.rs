use animal::Animal;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
//use graphics::*;
use graphics::{rectangle, clear};
use graphics::rectangle::square;
use graphics::Transformed;
use graphics::context::Context;

// #[derive(Clone)]
pub struct App {
    pub gl: GlGraphics,
    pub cats: Vec<Animal>,
    pub rats: Vec<Animal>,
}

impl App {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    const ANIMAL_SIZE: f64 = 5.0;
    
    pub fn render(&mut self, args: &RenderArgs){
        let square: [f64; 4] = square(0.0, 0.0, App::ANIMAL_SIZE);
        
        //const TRIANGLE:   &[[f32; 2]; 3] = &[[1.0, 0.0], [0.0, 1.732], [2.0, 1.732]];

        let cats = &self.cats;
        let rats = &self.rats;

        self.gl.draw(args.viewport(), |c, gl|{
            clear(App::GREEN, gl);
            
            for rat in rats {
                let transform = c.transform
                    .trans(rat.x, rat.y);
                rectangle(App::BLUE, square, transform, gl);
                //polygon(BLUE, &TRIANGLE, transform, gl);
            }
        });
    }
    
    fn draw_cat(c: &Context, gl: &mut GlGraphics, cats: &Vec<Animal>){
        for cat in cats{
            let transform = c.transform
                .trans(cat.x, cat.y);
            rectangle(App::RED, square, transform, gl);
            //polygon(RED, &TRIANGLE, transform, gl);
        }
    }
    

    pub fn update(&mut self) {
        let cats = self.cats.clone();
        let rats = self.rats.clone();
        self.cats = Animal::next_states_cats(&cats, &rats);
        self.rats = Animal::next_states_rats(&cats, &rats);
        //self.rats = App::eat_rats(self.cats.clone(), self.rats.clone());
    }
}
