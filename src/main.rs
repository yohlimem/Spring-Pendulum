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
    zoom: f32,
    apply_collision: bool,
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
    // just to create a graph after wards of the energy
    file.write_record(&["Pe", "Ke", "Te"]).unwrap();
    Model { 
        egui,
        pendulum,
        best_pos: vec2(0.0, 0.0),
        file, zoom: 10.0,
        apply_collision: false
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(update.since_start);
        
        let ctx = egui.begin_frame();
        
        
        
        let (pe, ke) = model.pendulum.calculate_energy();
        model.pendulum.solver(0.1);
        egui::Window::new("Rum window").show(&ctx, |ui| {
            ui.label("Spring Controls");
            ui.add(egui::Slider::new(&mut model.zoom, 2.0..=50.0).text("Zoom"));
            ui.add(egui::Slider::new(&mut model.pendulum.bob_mass, 1.0..=50.0).text("Mass"));
            ui.add(egui::Slider::new(&mut model.pendulum.stiffness, 1.0..=50.0).text("Stiffness"));
            ui.add(egui::Slider::new(&mut model.pendulum.length, 1.0..=50.0).text("Length"));
            ui.add(egui::Slider::new(&mut model.pendulum.damping, 0.0..=1.0).text("Damping"));
            
            
            ui.add(egui::widgets::Checkbox::new(&mut model.apply_collision, "Collision"));

            ui.label("\n Spring Energy");
            ui.add(egui::widgets::ProgressBar::new(pe / (pe + ke)).text("Potential energy"));
            ui.add(egui::widgets::ProgressBar::new(ke / (pe + ke)).text("Kinetic energy"));
            ui.add(egui::widgets::ProgressBar::new((pe + ke)/ 10000.0).text("Total energy"));
        });
        model.file.write_record(&[pe.to_string(), ke.to_string(), (ke + pe).to_string()]).unwrap();
        model.file.flush().unwrap();
    }
    // apply the collision to the ground
    // ground needed so the height doesn't go below 0
    if model.apply_collision {
        // apply said collision by inverting the y velocity
        // the size_function is just so the drawing matches with the collision
        model.pendulum.collision(&nannou::prelude::float::Float::ln);
    }

    // debuging
    // if model.best_pos.length() < model.pendulum.bob_pos.length() {
    //     model.best_pos = model.pendulum.bob_pos;
    // }
    
        // move the bob with the mouse
    move_pos(model, app);

    // println!("{:?}", model.pendulum);
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // normal line
    // draw.line()
    //     .start(model.pendulum.pos)
    //     .end(model.pendulum.bob_pos * model.size)
    //     .color(WHITE);

    // spring connection (fancy line)
    draw_spring(&draw, model.pendulum.pos, model.pendulum.bob_pos * model.zoom, 20, 50.0);

    // draw the ground
    draw.line()
        .start(vec2(-5000.0, model.pendulum.ground.y * model.zoom))
        .end(vec2(5000.0, model.pendulum.ground.y * model.zoom))
        .color(WHITE);
    // draw the bob
    draw.ellipse()
        .radius((model.pendulum.bob_mass).ln() * model.zoom)
        .xy(model.pendulum.bob_pos * model.zoom)
        .color(WHITE);
    // draw the mass
    draw.text(&model.pendulum.bob_mass.to_string())
        .xy(model.pendulum.bob_pos * model.zoom)
        .color(BLACK);

    // draw.ellipse()
    //     .radius(20.0)
    //     .xy(model.best_pos * model.size)
    //     .color(RED);


    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn move_pos(model: &mut Model, app: &App){
    if app.mouse.buttons.right().is_down()  {
        model.pendulum.bob_pos = app.mouse.position() / model.zoom;
        model.pendulum.velocity = vec2(0.0, 0.0);
    }
}

/// Draws a spring-like shape between two points with a given number of segments and width.
/// 
/// # Arguments
/// 
/// * `draw` - A mutable reference to the nannou draw object.
/// * `start_pos` - The starting position of the spring.
/// * `end_pos` - The ending position of the spring.
/// * `segment_num` - The number of segments that make up the spring.
/// * `segment_width` - The width of each segment of the spring.
fn draw_spring(draw: &Draw, start_pos: Vec2, end_pos: Vec2, segment_num: u32, segment_width: f32){
    let spacing = start_pos.distance(end_pos) / segment_num as f32;
    let to_point = end_pos - start_pos;
    let to_normal = to_point.normalize_or_zero();
    let angle = to_point.angle();
    for i in 1..segment_num {
        // find the position of the two points that make up the current segment
        // one on the left and the other on the right                                                    spaces them appart as needed
        let pos1 = start_pos + (to_normal * i as f32 * spacing as f32 + vec2(-segment_width/2.0,spacing as f32).rotate(angle + PI/2.0));
        let pos2 = start_pos + (to_normal * i as f32 * spacing as f32 + vec2(segment_width/2.0,spacing * 1.2 as f32).rotate(angle + PI/2.0));

        // find the next position to make a connection line
        let nextpos = start_pos + (to_normal * (i + 1) as f32 * spacing as f32 + vec2(-segment_width/2.0,spacing).rotate(angle + PI/2.0));

        // draw the lines
        draw.line()
            .start(pos1)
            .end(pos2)
            .color(WHITE);
        // if this is the last segment
        if i == segment_num - 1 {
            // draw a special line to the middle point
            let pos1 = start_pos + (to_normal * (i) as f32 * spacing as f32);
            
            draw.line()
                .start(pos2)
                .end(pos1)
                .color(WHITE);
            draw.line()
                .start(pos1)
                .end(end_pos)
                .color(WHITE);
        }
        else {
            draw.line()
                .start(nextpos)
                .end(pos2)
                .color(WHITE);

        }
    }

}