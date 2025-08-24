use nannou::prelude::*;
use crate::model::Model;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for ball in &model.balls {
        draw.ellipse()
            .xy(ball.pos)
            .radius(ball.radius)
            .color(ball.color);
    }

    draw.to_frame(app, &frame).unwrap();

}
