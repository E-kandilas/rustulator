#[allow(dead_code)]

use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;
use frame::Frame;
use macros::display;

use self::Message::*;


// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    CE, Mult, Div, Dec, Eqs
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Num0 => write!(f, "0"),
            Message::Num1 => write!(f, "1"),
            Message::Num2 => write!(f, "2"),
            Message::Num3 => write!(f, "3"),
            Message::Num4 => write!(f, "4"),
            Message::Num5 => write!(f, "5"),
            Message::Num6 => write!(f, "6"),
            Message::Num7 => write!(f, "7"),
            Message::Num8 => write!(f, "8"),
            Message::Num9 => write!(f, "9"),
            Message::CE => write!(f, "CE"),
            Message::Mult => write!(f, "Mult"),
            Message::Div => write!(f, "Div"),
            Message::Dec => write!(f, "."),
            Message::Eqs => write!(f, "=")
        }
    }
}

impl Message {
  pub fn iterator() -> std::slice::Iter<'static, Message> {
      static MESSAGES: [Message; 15] = [
      Num0, Num1, Num2, Num3, Num4,
      Num5, Num6, Num7, Num8, Num9,
      Dec, CE, Mult, Div, Eqs,
      ];
      MESSAGES.iter()
  }
}
// TODO:
// Add All Calculator buttons %,+, -, Look at an actual calculator
// Look into ways to change the placements of the buttons
// Begin storing messages and displaying them on the display_frame
// Add some more constants and variables for window sizes

fn main() {
    // CONSTANTS
    const BUTT_WIDTH: i32 = 80;
    const BUTT_HEIGHT: i32 = 40;
    const EDGE_DIST: i32 = 80;


    let app = app::App::default().with_scheme(fltk::app::Scheme::Gtk);

    let mut win = fltk::window::Window::new(100, 100, 400, 400, "Rustulator").center_screen();
    // let mut control_frame = fltk::frame::Frame::new(30, 30, 400, 300, "Control");
    let mut display_frame =  fltk::frame::Frame::new(40, 10, 300, 30, "Display").with_label("Begin Typing");

    display_frame.set_color(Color::from_rgb(255, 255, 255));
    display_frame.set_frame(FrameType::BorderBox);


    let (sender, reciever) = app::channel::<Message>();

    // Button Generation
    let mut butt_count = 0;
    let mut butts_per_row = 0;
    let mut y_offset = 0;
    // let butt_body = group::Flex::default(); // This is an idea worth looking into to control button location placement
    for button_type in Message::iterator() {
      butts_per_row += 1;
      let mut butt_name= butt_count.to_string();
      
      if butt_count % 3 == 0 {
        butts_per_row = 0;
        y_offset = y_offset + BUTT_HEIGHT;
      }
      let butt_dist = EDGE_DIST + (butts_per_row * BUTT_WIDTH);
      
      if butt_count > 9 {
        butt_name = button_type.to_string();
      }

      Button::new(butt_dist, y_offset, BUTT_WIDTH, BUTT_HEIGHT,&*butt_name);
      butt_count = butt_count + 1;
    }


    win.end();
    win.show();
    
    
    while app.wait() {
      //Lesson! Look into if lets again and is there another way / what is this syntactic sugar for?
      if let Some(msg) = reciever.recv() {
        match msg {
          // Message::Spaz => {
          //   let rand_x = rand::thread_rng().gen_range(0..(400 - 80));
          //   let rand_y = rand::thread_rng().gen_range(0..100);
          //   but.resize(rand_x, rand_y, 80, 40);
          //   win.redraw();
          // }
          _ => (), // dont need if all enums are handled btw
        }
      }
    }

    app.run().unwrap();
}

