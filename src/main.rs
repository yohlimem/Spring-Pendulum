use csv::Writer;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
mod pendulum;
use pendulum::SpringMass;

struct Model {
    // window: Window,
    egui: Egui,
    pendulum: SpringMass,
    best_pos: Vec2,
    file: Writer<std::fs::File>,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let pendulum = SpringMass::default();
    let mut file = Writer::from_path("hi.csv").unwrap();
    file.write_record(&["Pe", "Ke", "Te"]).unwrap();
    Model { egui, pendulum, best_pos: vec2(0.0, 0.0), file }
}

fn update(app: &App, model: &mut Model, update: Update) {
    // model.pendulum.collision();
    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);
        
        let ctx = egui.begin_frame();
        
        
        
        let (Pe, Ke) = model.pendulum.calculate_energy();
        model.pendulum.solver(0.1);
        egui::Window::new("Rum window").show(&ctx, |ui| {
            ui.label("res");
            // ui.add(egui::widgets::ProgressBar::new(Pe / Te).text("Potential energy"));
            // ui.add(egui::widgets::ProgressBar::new(Ke / Te).text("Kinetic energy"));
        });
        model.file.write_record(&[Pe.to_string(), Ke.to_string(), (Ke + Pe).to_string()]).unwrap();
        model.file.flush().unwrap();
    }

    if model.best_pos.length() < model.pendulum.bob_pos.length() {
        model.best_pos = model.pendulum.bob_pos;
    }
    

    move_pos(&mut model.pendulum, app);

    // println!("{:?}", model.pendulum);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.line()
        .start(model.pendulum.pos)
        .end(model.pendulum.bob_pos)
        .color(WHITE);
    draw.line()
        .start(vec2(-1000.0, model.pendulum.ground.y))
        .end(vec2(1000.0, model.pendulum.ground.y))
        .color(WHITE);
    draw.ellipse()
        .radius(model.pendulum.bob_mass)
        .xy(model.pendulum.bob_pos)
        .color(WHITE);


    draw.ellipse()
        .radius(20.0)
        .xy(model.best_pos)
        .color(RED);


    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn move_pos(spring: &mut SpringMass, app: &App){
    if app.mouse.buttons.left().is_down() {
        spring.pos = app.mouse.position();
    }
}