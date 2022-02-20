use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use rand::Rng;

struct Game {
    gl: GlGraphics,
    bird: Bird,
    pillar: Vec<Pillar>,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GRAY, gl);
        });

        self.bird.render(&mut self.gl, arg);
        for p in &mut self.pillar{
            p.render(&mut self.gl, arg);
        }
    }

    fn update(&mut self){
        self.bird.update();
        for p in &mut self.pillar {
            p.update();
        }

        for p in self.pillar.clone() {
            if (p.pos - 70.0) < 0.0 && (p.pos + 30.0) - 50.0 > 0.0{
                if !(self.bird.pos > (p.holepos - 42.0) && self.bird.pos < (p.holepos + 42.0)){
                    self.reset();
                }
            } 

        }

        if (self.bird.pos <= 0.0) || (self.bird.pos >= 250.0){
            self.reset();
        }

        if let Some(e) = self.pillar.first(){
            if e.pos < -50.0 {
                self.pillar.remove(0);
            }
        }

        match self.pillar.last(){
            Some(e) => 
                if e.pos < 350.0 {
                    self.pillar.push(Pillar::new(500.0));
                }
            None => {self.pillar.push(Pillar::new(500.0))}
        }
    }

    fn pressed(&mut self, btn: &Button){
        if let &Button::Keyboard(Key::Space) = btn {
            self.bird.velocity = -8.0;
        }

        if let &Button::Keyboard(Key::R) = btn {
            self.reset();
        }
    }

    fn reset(&mut self){
        self.pillar = vec![Pillar::new(500.0)];
        self.bird.pos = 140.0;
        self.bird.velocity = 0.0;
    }
}

struct Bird {
    pos: f64,
    velocity: f64
}

impl Bird {
    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs){
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = graphics::rectangle::square(50.0, self.pos, 20_f64);
        
        gl.draw(arg.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        })
    }
    fn update(&mut self){
        self.pos += self.velocity;
        if self.velocity < 10_f64{
            self.velocity += 1.3;
        }
        if self.pos > 280.0{
            self.pos = 280.0;
            self.velocity = 0.0;
        }
        
        if self.pos < 0.0{
            self.pos = 0.0;
            self.velocity = 0.0;
        }
    }
}

#[derive(Clone)]
struct Pillar {
    pos: f64,
    holepos: f64,
    space: f64,
}

impl Pillar {
    fn new(xpos: f64) -> Pillar{
        Pillar {
            pos: xpos,
            holepos: rand::thread_rng().gen_range(90.0..210.0),
            space: 42.0,
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs){
        use graphics;

        let ORANGE: [f32; 4] = [1.0, 0.64, 0.0, 1.0];

        let upper_pillar = graphics::rectangle::rectangle_by_corners(
            self.pos,
            0.0,
            self.pos + 30.0,
            self.holepos - self.space);

        let lower_pillar = graphics::rectangle::rectangle_by_corners(
            self.pos,
            self.holepos + self.space,
            self.pos + 30.0,
            300.0);

        gl.draw(args.viewport(), |c, gl|{
            let transform = c.transform;

            graphics::rectangle(ORANGE, upper_pillar, transform, gl);
            graphics::rectangle(ORANGE, lower_pillar, transform, gl);
        });
    }

    fn update(&mut self){
        self.pos -= 3.0;
    }

}

fn main() {
    let opengl = OpenGL::V3_3;
    let mut window: Window = WindowSettings::new(
        "flappy",
        (500, 300)
    ).graphics_api(opengl)
        .exit_on_esc(false)
        .build()
        .unwrap();
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        bird: Bird {
            pos: 140.0,
            velocity: 0.0,
        },
        pillar: Vec::new(),
    };
    let mut events = Events::new(EventSettings::new()).ups(30);
    while let Some(e) = events.next(&mut window){
        if let Some(args) = e.render_args(){
            game.render(&args);
        }
        if let Some(k) = e.update_args(){
            game.update();
        }
        if let Some(u) = e.button_args(){
            if u.state == ButtonState::Press{
                game.pressed(&u.button);
            }
        }
    }
}