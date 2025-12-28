use std::collections::HashMap;
use vglyph::compile;
use nannou::prelude::*;

type InputSequence = Vec<String>;
type Line = ((f32, f32), (f32, f32));
type Character = (InputSequence, Vec<Line>);

struct Model {
    characters: HashMap<String, Character>,
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    Model {
        characters: compile(include_str!("../res/v.glyph")),
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn view(app: &App, model: &Model, frame: Frame) {
    let top_left = app.main_window().rect().top_left();
    let draw = app.draw()
        .x_y(top_left.x, top_left.y)
        .scale_y(-1.0);

    for (idx, (_, (_, lines))) in model.characters.iter().enumerate() {
        for (start, end) in lines {
            draw.line()
                .x((idx % 20) as f32 * 52.0 + 24.0)
                .y((idx / 20) as f32 * 52.0 + 24.0)
                .color(BLUE)
                .start(Point2::new(start.0 * 48.0, start.1 * 48.0))
                .end(Point2::new(end.0 * 48.0, end.1 * 48.0));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}