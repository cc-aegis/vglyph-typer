use std::collections::HashMap;
use std::sync::OnceLock;
use nannou::event::ElementState;
use vglyph::compile;
use nannou::prelude::*;
use nannou::winit::event::DeviceEvent::Key;
use nannou::winit::event::ScanCode;
use nannou::winit::event::KeyboardInput;

type InputSequence = Vec<String>;
type Line = ((f32, f32), (f32, f32));
type Character = (InputSequence, Vec<Line>);

struct Model {
    characters: HashMap<String, Character>,
    query: Vec<String>,
    text: Vec<String>,
}

fn hotkeys() -> &'static HashMap<ScanCode, &'static str> {
    static JWT_SECRET: OnceLock<HashMap<ScanCode, &'static str>> = OnceLock::new();
    JWT_SECRET.get_or_init(|| HashMap::from([
        (16, "time"),
        (17, "word"),
        (18, "not"),
        (19, "box"),
        (20, "way"),
        (21, "water"),
        (22, "animal"),
        (23, "yin"),
        (24, "sound"),
        (30, "rays"),
        (31, "question"),
        (32, "organ"),
        (33, "stroke"),
        (34, "tool"),
        (35, "tree"),
        (36, "human"),
        (37, "yang"),
        (38, "power"),
        (46, "plan"),
        (47, "room"),
        (48, "sun"),
        (49, "hands"),
        (50, "place"),
    ]))
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
        query: Vec::new(),
        text: Vec::new(),
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::DeviceEvent(_, Key(KeyboardInput { scancode, state: ElementState::Pressed, .. })) => {
            if let Some(name) = hotkeys().get(&scancode) {
                model.query.push(name.to_string());
            } else if scancode == 14 {
                if model.query.is_empty() {
                    let _ = model.text.pop();
                }
                let _ = model.query.pop();
            } else if scancode == 57 {
                if let Some((c, _)) = model.characters.iter().find(|(_, (i, _))| i.eq(&model.query)) {
                    model.text.push(c.clone());
                    model.query.clear();
                }
            } else {
                println!("scancode: {scancode}");
            }
        },
        _ => {},
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let top_left = app.main_window().rect().top_left();
    let draw = app.draw()
        .x_y(top_left.x, top_left.y)
        .scale_y(-1.0);

    draw.background().color(WHITE);

    for (idx, name) in model.text.iter().enumerate() {
        let lines = &model.characters.get(name).unwrap().1;
        for (start, end) in lines {
            draw.line()
                .x(idx as f32 * 52.0 + 24.0)
                .y(24.0)
                .color(BLACK)
                .start(Point2::new(start.0 * 48.0, start.1 * 48.0))
                .end(Point2::new(end.0 * 48.0, end.1 * 48.0));
        }
    }

    for (idx, (_, (_, lines))) in model
        .characters
        .iter()
        .filter(|(_, (i, _))| i.starts_with(&model.query))
        .enumerate() {
        for (start, end) in lines {
            draw.line()
                .x((idx % 20) as f32 * 52.0 + 24.0)
                .y((idx / 20) as f32 * 52.0 + 24.0)
                .color(BLACK)
                .start(Point2::new(start.0 * 48.0, start.1 * 48.0 + 96.0))
                .end(Point2::new(end.0 * 48.0, end.1 * 48.0 + 96.0));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}