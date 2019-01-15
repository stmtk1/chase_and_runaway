use animal::Animal;
use piston::input::RenderArgs;
use opengl_graphics::GlGraphics;
//use graphics::*;
use graphics::{rectangle, clear};
use graphics::rectangle::square;
use graphics::Transformed;

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
            
            for cat in cats{
                let transform = c.transform
                    .trans(cat.x, cat.y);
                rectangle(App::RED, square, transform, gl);
                //polygon(RED, &TRIANGLE, transform, gl);
            }
            
            for rat in rats {
                let transform = c.transform
                    .trans(rat.x, rat.y);
                rectangle(App::BLUE, square, transform, gl);
                //polygon(BLUE, &TRIANGLE, transform, gl);
            }
        });
    }
    
    fn move_cats(cats: Vec<Animal>, rats: Vec<Animal>) -> Vec<Animal>{
        let ret = cats.clone()
            .into_iter()
            .map(|cat| cat.chase(&rats, &cats).move_self())
            .collect();
        Animal::life_manage(Animal::delete_dead(ret))
    }
    
    fn move_rats(rats: Vec<Animal>, cats: Vec<Animal>) -> Vec<Animal> {
        let ret = rats
            .into_iter()
            .map(|rat| rat.run_away(&cats).move_self())
            .collect();
        Animal::life_manage(Animal::delete_dead(ret))
    }
    
    fn eat_rats(cats: Vec<Animal>, rats: Vec<Animal>) -> Vec<Animal> {
        let mut new_rats = rats.clone();
        
        for cat in &cats {
            new_rats = cat.eat(new_rats);
        }
        new_rats
    }

    pub fn update(&mut self) {
        self.cats = App::move_cats(self.cats.clone(), self.rats.clone());
        self.rats = App::move_rats(self.rats.clone(), self.cats.clone());
        
        self.rats = App::eat_rats(self.cats.clone(), self.rats.clone());
    }
}
