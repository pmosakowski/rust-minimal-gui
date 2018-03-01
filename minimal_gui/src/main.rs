extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;
    let title = String::from("rust-minimal-gui");

    let _display = create_glium_display(WIDTH, HEIGHT, title);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    load_font(&mut ui);
}

fn create_glium_display(width: u32, height: u32, title: String) -> glium::Display {
    // glium event loop
    let events_loop = glium::glutin::EventsLoop::new();

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