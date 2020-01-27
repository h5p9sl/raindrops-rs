use graphics::Context;
use piston::{EventSettings, Events, WindowSettings};
use piston::{RenderEvent, ResizeEvent, UpdateEvent};

use glium_graphics::{Glium2d, GliumGraphics, GliumWindow, OpenGL};

use std::time::Instant;

mod raindrop;
use raindrop::Raindrop;

struct App {
    raindrops: Vec<Raindrop>,
    counter: f32,
}

impl App {
    pub fn new(window_size: [f64; 2]) -> App {
        let mut app = App {
            raindrops: Vec::with_capacity(100),
            counter: 0.0,
        };
        app.create_drops(window_size);
        app
    }

    fn create_drops(&mut self, window_size: [f64; 2]) {
        while self.raindrops.len() < 100 {
            self.raindrops.push(Raindrop::new(window_size));
        }
    }

    pub fn on_update(&mut self, dt: f64, window_size: [f64; 2]) {
        self.counter += dt as f32;
        let r = f32::sin(self.counter + 2.0) + 1.0 / 2.0;
        let g = f32::sin(self.counter + 0.0) + 1.0 / 2.0;
        let b = f32::sin(self.counter + 4.0) + 1.0 / 2.0;
        for drop in &mut self.raindrops {
            drop.update(dt, window_size, [r, g, b, 1.0]);
        }
    }

    pub fn on_render(&self, c: Context, g: &mut GliumGraphics<'_, '_, glium::Frame>) {
        use graphics::*;
        clear([0.1; 4], g);
        for drop in &self.raindrops {
            drop.draw(c, g)
        }
    }
}

fn main() {
    let gl_version = OpenGL::V3_2;
    let mut window_size: [f64; 2] = [800.0, 600.0];
    let mut window: GliumWindow = WindowSettings::new("Piston Test", window_size)
        .automatic_close(true)
        .resizable(true)
        .graphics_api(gl_version)
        .build()
        .unwrap();

    let mut app = App::new(window_size);

    let mut g2d = Glium2d::new(gl_version, &window);
    let mut events = Events::new(EventSettings::new());
    let mut clock = Instant::now();
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.resize_args() {
            window_size = args.window_size;
        }
        if let Some(_args) = e.update_args() {
            let delta = clock.elapsed();
            app.on_update(delta.as_secs_f64(), window_size);
            clock = Instant::now();
        }
        if let Some(args) = e.render_args() {
            let mut frame = window.draw();
            g2d.draw(&mut frame, args.viewport(), |c, mut g| {
                app.on_render(c, &mut g);
            });
            frame.finish().unwrap();
        }
    }
}
