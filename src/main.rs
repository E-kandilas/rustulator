
use enums::{Color, FrameType}; // Import the FrameType module

use fltk::*;
use fltk::{app, button::{self, Button}, frame::Frame, prelude::*, window::Window};
use rand::Rng;

fn main() {
    let app = fltk::app::App::default().with_scheme(fltk::app::Scheme::Gtk);
    let mut win = fltk::window::Window::new(100, 100, 400, 300, "Rustulator");
    let mut frame = fltk::frame::Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);

    let mut frame = Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);
    let mut but = Button::new(160, 200, 80, 40, "Click me!");
    but.set_color(Color::Cyan);
    but.set_label_color(Color::Red);
    but.set_frame(FrameType::FlatBox);
    but.set_color(Color::Blue);
    but.set_label_size(20);
    but.set_callback(move |_| frame.set_label("Hello World!"));

    but.clone().set_callback(move |_| {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(0..=300);
      let y = rng.gen_range(0..=200);
      but.resize(x, y, 80, 40);
    });

    win.end();
    win.show();
    app.run().unwrap();
}

