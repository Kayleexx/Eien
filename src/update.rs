use nannou::prelude::*;
use crate::model::Model;

const GRAVITY: f32 = -0.3;   // downward pull
const DAMPING: f32 = 0.9;    // energy loss on wall bounce
const FRICTION: f32 = 0.995; // air resistance

pub fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    if let Some(index) = model.dragging {
        let mouse = app.mouse.position();
        model.balls[index].pos = mouse;
        model.balls[index].vel = vec2(0.0, 0.0); // stop movement while dragging
    }

    for ball in model.balls.iter_mut() {
        // Apply gravity wells
        for well in &model.wells {
            let delta = well.pos - ball.pos;
            let dist_sq = delta.length_squared().max(1.0); // avoid div by 0
            let force = delta.normalize_or_zero() * well.strength / dist_sq;
            ball.vel += force;
        }

        // Gravity
        ball.vel.y += GRAVITY;

        // Update position
        ball.pos += ball.vel;

        // Bounce on walls
        if ball.pos.x - ball.radius < win.left() {
            ball.pos.x = win.left() + ball.radius;
            ball.vel.x *= -DAMPING;
        }
        if ball.pos.x + ball.radius > win.right() {
            ball.pos.x = win.right() - ball.radius;
            ball.vel.x *= -DAMPING;
        }
        if ball.pos.y - ball.radius < win.bottom() {
            ball.pos.y = win.bottom() + ball.radius;
            ball.vel.y *= -DAMPING;
        }
        if ball.pos.y + ball.radius > win.top() {
            ball.pos.y = win.top() - ball.radius;
            ball.vel.y *= -DAMPING;
        }

        // Air friction
        ball.vel *= FRICTION;
    }

    // Ball-ball collisions
    let len = model.balls.len();
    for i in 0..len {
        for j in (i + 1)..len {
            let (b1, b2) = {
                let (left, right) = model.balls.split_at_mut(j);
                (&mut left[i], &mut right[0])
            };

            let delta = b2.pos - b1.pos;
            let dist = delta.length();
            let min_dist = b1.radius + b2.radius;

            if dist < min_dist {
                let normal = delta.normalize_or_zero();

                // Relative velocity along normal
                let rel_vel = b2.vel - b1.vel;
                let vel_along_normal = rel_vel.dot(normal);

                // Skip if moving apart
                if vel_along_normal > 0.0 {
                    continue;
                }

                let restitution = 1.0; // perfectly elastic
                let j = -(1.0 + restitution) * vel_along_normal
                    / (1.0 / b1.mass + 1.0 / b2.mass);

                let impulse = normal * j;

                b1.vel -= impulse / b1.mass;
                b2.vel += impulse / b2.mass;

                // Positional correction to prevent sticking
                let penetration = min_dist - dist;
                let correction = normal * (penetration / (b1.mass + b2.mass) * 0.8);

                b1.pos -= correction * (b2.mass / (b1.mass + b2.mass));
                b2.pos += correction * (b1.mass / (b1.mass + b2.mass));
            }
        }
    }
}
