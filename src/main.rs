extern crate sdl2;

use sdl2::pixels::Color;

#[macro_use]
mod events;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
    }
}

struct Position {
    x: i32,
    y: i32,
}

struct Size {
    w: i32,
    h: i32,
}

enum Widget {
    Form {      position: Position, 
                size: Size,
                },
    Label {     position: Position,
                size: Size,
                text: String
                },
    Button {    position: Position,
                size: Size,
                pressed: bool,
                },
    Textbox {   position: Position,
                size: Size,
                text: String
                },
}

fn new_form(ix: i32, iy: i32, iw: i32, ih: i32) -> Widget {
    Widget::Form {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
    }
}

fn new_label(ix: i32, iy: i32, iw: i32, ih: i32, itext: &str) -> Widget {
    Widget::Label {
        position: Position {x: ix, y: iy},
        size: Size {w: iw, h: ih},
        text: itext.to_string(),
    }
} 

struct UI {
    widgets: Vec<(i32, Widget)>,
}

impl UI {
    pub fn new() -> UI {
        UI {
            widgets: Vec::new()
        }
    }

    pub fn add_widget(&mut self, parent: i32, w: Widget) -> i32 {
        let index = self.widgets.len();
        let maxi32 = std::i32::MAX as usize;

        if index >= maxi32 {
            panic!("Too many widgets");
        }

        let safe_index = index as i32;

        self.widgets.push((parent, w));

        safe_index
    }
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create the window
    let window = video.window("ArcadeRS Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // Prepare the events record
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    //Create user interface
    let mut ui = UI::new();
    let main_form = ui.add_widget(0, new_form(50, 50, 100, 100));
    let main_label = ui.add_widget(main_form, new_label(10, 10, 55, 12, "Hello."));

    println!("main_form: {}, main_label: {}", main_form, main_label);

    loop {
        events.pump();

        if events.now.key_escape == Some(true) {
            break;
        }

        // Render a fully black window
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}


