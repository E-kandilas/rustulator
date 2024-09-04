
use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;
// use fltk::app::{Sender, Receiver}; // Import Sender and Receiver
use frame::Frame;
use rand::Rng;
// use fltk::app::channel; // Import channel

use self::Message::*;


// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
  Spaz,
  Num0, Num1, Num2, Num3, Num4,
  Num5, Num6, Num7, Num8, Num9,
  Back, Mult, Div, 
}

impl Message {
  pub fn iterator() -> std::slice::Iter<'static, Message> {
      static MESSAGES: [Message; 14] = [Spaz,
      Num0, Num1, Num2, Num3, Num4,
      Num5, Num6, Num7, Num8, Num9,
      Back, Mult, Div];
      MESSAGES.iter()
  }
}

fn main() {
    let app = app::App::default().with_scheme(fltk::app::Scheme::Gtk);

    let mut win = fltk::window::Window::new(100, 100, 400, 300, "Rustulator");
    let mut frame = fltk::frame::Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);

    let (sender, reciever) = app::channel::<Message>();

    for buttonType in Message::iterator() {
      println!("{:?}", buttonType)
    }


    let mut frame = Frame::new(0, 0, 400, 300, "");
    frame.set_color(fltk::enums::Color::White);
    let mut but = Button::new(160, 200, 80, 40, "Click me!");
    but.set_color(Color::Cyan);
    but.set_label_color(Color::Red);
    but.set_frame(FrameType::FlatBox);
    but.set_label_size(20);
    // but.set_callback(move |_| frame.set_label("Hello World!"));

    but.handle(move |b, evt| match evt {
        Event::Enter => {
          b.set_color(Color::XtermBgYellow);
          b.redraw();
          return true;
        }
        Event::Leave => {
          b.set_color(Color::Cyan);
          b.redraw();
          return true;
        }
        // using resize changes the behavior of the button and thus needs different behavior
        Event::Push => {
            // let rand_x = rand::thread_rng().gen_range(0..(400 - 80));
            // let rand_y = rand::thread_rng().gen_range(0..100);
            // but.resize(rand_x, rand_y, 80, 40);
            // but.redraw();
            // return true;
            b.emit(sender, Message::Spaz);
            return true;
        }
        _ => {
          // print!("{:?}", evt);
          return false;
        },
    });
    
    
    win.end();
    win.show();
    
    
    while app.wait() {
      // Look into if lets again and is there another way / what is this syntactic sugar for?
      if let Some(msg) = reciever.recv() {
        match msg {
          Message::Spaz => {
            let rand_x = rand::thread_rng().gen_range(0..(400 - 80));
            let rand_y = rand::thread_rng().gen_range(0..100);
            but.resize(rand_x, rand_y, 80, 40);
            win.redraw();
          }
          _ => (), // dont need if all enums are handled btw
        }
      }
    }
    
    // but.emit
    
    
    app.run().unwrap();
}

