use piston::{WindowSettings, Events, EventSettings};
use piston::{UpdateArgs, UpdateEvent, RenderArgs, RenderEvent};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::Context;

fn on_update(args: UpdateArgs) {
}

fn on_render(args: RenderArgs, c: Context, g: &mut GlGraphics) {
    graphics::clear([0.2, 0.2, 0.2, 1.0], g);
}

fn main() {
    let gl_version = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Piston Test", [800, 600])
        .automatic_close(true)
        .resizable(false)
        .graphics_api(gl_version)
        .build()
        .unwrap();
    let mut gl: GlGraphics = GlGraphics::new(gl_version);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.update_args() {
            on_update(args)
        }
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                on_render(args, c, g);
            });
        }
    }
}
