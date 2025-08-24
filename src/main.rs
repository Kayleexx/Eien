use nannou::prelude::*;
mod model;
mod update;
mod view;
mod events;

use model::{Model, Ball};
use update::update;
use view::view;
use events::event;

fn main() {
    nannou::app(model)
        .update(update)
        .event(event) // unified event handler
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn model(app: &App) -> Model {
    let win = app.main_window().rect();
    let mut balls = Vec::new();

    for _ in 0..10 {
        let radius = random_range(10.0, 40.0);
        balls.push(Ball {
            pos: vec2(
                random_range(win.left() + radius, win.right() - radius),
                random_range(win.bottom() + radius, win.top() - radius),
            ),
            vel: vec2(random_range(-3.0, 3.0), random_range(-3.0, 3.0)),
            radius,
            mass: radius, // mass proportional to radius
            color: rgb8(
                (random_f32() * 255.0) as u8,
                (random_f32() * 255.0) as u8,
                (random_f32() * 255.0) as u8,
            ),
        });
    }

    Model {
        balls,
        dragging: None,
        last_mouse_pos: vec2(0.0, 0.0),
        wells: Vec::new(),
    }
}
