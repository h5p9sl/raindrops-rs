use crate::{Context, GliumGraphics};

pub struct Raindrop {
    pos: [f64; 3],
    vel: [f64; 2],
    color: [f32; 4],
}

fn get_random() -> f64 {
    rand::random::<f64>()
}

impl Raindrop {
    pub fn new(window_size: [f64; 2]) -> Raindrop {
        let mut drop = Raindrop {
            pos: [0.0; 3],
            vel: [0.0; 2],
            color: [1.0; 4],
        };
        drop.reset(window_size);
        drop
    }

    fn reset(&mut self, window_size: [f64; 2]) {
        let rand_x = get_random() * window_size[0];
        let rand_z = get_random();
        self.pos = [rand_x, -200.0, rand_z];
        self.vel = [get_random() * 500.0 - 100.0, get_random() * 1000.0];
    }

    pub fn update(&mut self, delta: f64, window_size: [f64; 2], color: [f32; 4]) {
        // Update velocity & position
        self.vel[1] += 1000.0 * delta * self.pos[2];
        for i in 0..2 {
            self.pos[i] += self.vel[i] * delta;
            if self.pos[i] > window_size[i] + 100.0 {
                self.reset(window_size);
            }
        }
        self.color = color;
    }

    pub fn draw(&self, c: Context, g: &mut GliumGraphics<'_, '_, glium::Frame>) {
        let pos = self.pos;
        let vel = self.vel;
        let (x1, y1) = (pos[0] + vel[0] * 0.01, pos[1] + vel[1] * 0.01);
        let (x2, y2) = (pos[0], pos[1]);

        graphics::line(
            self.color,
            self.pos[2],
            [x1, y1, x2, y2],
            c.transform,
            g
        );
    }
}
