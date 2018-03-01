extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let title = String::from("rust-minimal-gui");

    // glium event loop
    let events_loop = glium::glutin::EventsLoop::new();

    // create window
    let window = glium::glutin::WindowBuilder::new()
	.with_title(title)
	.with_dimensions(WIDTH, HEIGHT);

    // create OpenGl context
    let context = glium::glutin::ContextBuilder::new()
	.with_vsync(true)
	.with_multisampling(4);

    // combine the above into an rendering target
    let _display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let _ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
}