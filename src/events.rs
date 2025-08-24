use crate::model::Model;
use nannou::prelude::*;

pub fn event(app: &App, model: &mut Model, ev: Event) {
    match ev {
        // Mouse pressed
        Event::WindowEvent { simple: Some(WindowEvent::MousePressed(button)), .. } => {
            let mouse_pos = app.mouse.position();
            model.last_mouse_pos = mouse_pos;

            // Check if a ball is under the mouse for dragging
            for (i, ball) in model.balls.iter().enumerate() {
                if mouse_pos.distance(ball.pos) < ball.radius {
                    model.dragging = Some(i);
                    break;
                }
            }

            // Left-click also spawns a new ball
            if button == MouseButton::Left {
                let radius = random_range(10.0, 40.0);
                let pos = mouse_pos;
                model.balls.push(crate::model::Ball {
                    pos,
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
        }

        // Mouse released
        Event::WindowEvent { simple: Some(WindowEvent::MouseReleased(_)), .. } => {
            if let Some(index) = model.dragging {
                let ball = &mut model.balls[index];
                // compute velocity based on drag
                ball.vel = (app.mouse.position() - model.last_mouse_pos) * 0.5;
            }
            model.dragging = None;
        }

        // Mouse moved
        Event::WindowEvent { simple: Some(WindowEvent::MouseMoved(pos)), .. } => {
            // Dragging logic
            if let Some(i) = model.dragging {
                model.balls[i].pos = pos;
            }
            model.last_mouse_pos = pos;
        }

        _ => {}
    }
}
