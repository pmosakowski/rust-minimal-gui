#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate rand;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

// the GUI state
struct DiceApp {
    roll: u8,
}

impl DiceApp {
    fn new() -> DiceApp {
        DiceApp {
            roll: 0,
        }
    }
}

// In conrod, each widget must have its own unique identifier so that the `Ui` can keep track of
// its state between updates.
//
// To make this easier, conrod provides the `widget_ids` macro. This macro generates a new type
// with a unique `widget::Id` field for each identifier given in the list. See the `widget_ids!`
// documentation for more details.
widget_ids! {
    struct Ids {
        canvas,
        text,
        roll_button,
    }
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod::render::Primitives` to the
/// screen.
///
/// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
/// glutin+glium event loop that works efficiently with conrod.
struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {

    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }

}

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let title = String::from("rust-minimal-gui");

    // glium event loop
    let mut events_loop = glium::glutin::EventsLoop::new();
    let display = create_glium_display(WIDTH, HEIGHT, title, &events_loop);

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    load_font(&mut ui);

    let mut app_state = DiceApp::new();
    let ids = Ids::new(ui.widget_id_generator());

    // Poll events from the window.
    let mut event_loop = EventLoop::new();

    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // We'll set all our widgets in a single function called `set_widgets`.
        {
            let mut ui = ui.set_widgets();
            set_widgets(&mut ui, &mut app_state, &ids);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

}

fn create_glium_display(width: u32, height: u32, title: String, events_loop: &glium::glutin::EventsLoop) -> glium::Display {

    // create window
    let window = glium::glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height);

    // create OpenGl context
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);

    // combine the above into an rendering target
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    display
}

fn load_font(ui: &mut conrod::Ui) {
   let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
   let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
   ui.fonts.insert_from_file(&font_path).unwrap();
   println!("Loaded font {:?}", &font_path);
}

/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(ui: &mut conrod::UiCell, app: &mut DiceApp, ids: &Ids) {
    use conrod::{color, widget, Colorable, Positionable, Widget, Sizeable, Borderable, Labelable};
    use rand::random;

    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        .pad(30.0)
        .color(color::BLUE)
        .set(ids.canvas, ui);
    // Text example.
    widget::Text::new(&app.roll.to_string())
        .top_left_with_margins_on(ids.canvas, 0.0, 20.0)
        .font_size(32)
        .color(color::BLUE.plain_contrast())
        .set(ids.text, ui);

    if widget::Button::new()
        .w_h(200.0, 50.0)
        .down_from(ids.text, 45.0)
        .rgb(0.4, 0.75, 0.6)
        .border(2.0)
        .label("Roll d6")
        .set(ids.roll_button, ui)
        .was_clicked()
    {
        app.roll = random::<u8>() % 6 + 1;
        println!("Rolled {}", &app.roll);
    }
}
