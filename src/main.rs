#[allow(dead_code)]

use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;
use frame::Frame;
use rand::Rng;

use self::Message::*;


// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
    Spaz,
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,
    Back, Mult, Div,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Spaz => write!(f, "Spaz"),
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
            Message::Back => write!(f, "Back"),
            Message::Mult => write!(f, "Mult"),
            Message::Div => write!(f, "Div"),
        }
    }
}

// impl Message {
//     pub fn iterator() -> std::slice::Iter<'static, Message> {
//         static MESSAGES: [Message; 14] = [
//             Message::Spaz,
//             Message::Num0,
//             Message::Num1,
//             Message::Num2,
//             Message::Num3,
//             Message::Num4,
//             Message::Num5,
//             Message::Num6,
//             Message::Num7,
//             Message::Num8,
//             Message::Num9,
//             Message::Back,
//             Message::Mult,
//             Message::Div,
//         ];
//         MESSAGES.iter()
//     }
// }

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

    let mut buttCount = 0;
    for buttonType in Message::iterator() {
      let mut buttName = buttCount.to_string();
      let buttDist = buttCount * 160;
      if buttCount <= 9 {
          buttName = buttCount.to_string();
      } else {
          buttName = buttonType.to_string();
      }
      let mut but = Button::new(160, 200+buttDist, 80, 40,&*buttName );
      buttCount = buttCount + 1;
    }


    // let mut frame = Frame::new(0, 0, 400, 300, "");
    // frame.set_color(fltk::enums::Color::White);
    // let mut but = Button::new(160, 200, 80, 40, "Click me!");
    // but.set_color(Color::Cyan);
    // but.set_label_color(Color::Red);
    // but.set_frame(FrameType::FlatBox);
    // but.set_label_size(20);

    // but.handle(move |b, evt| match evt {
    //     Event::Enter => {
    //       b.set_color(Color::XtermBgYellow);
    //       b.redraw();
    //       return true;
    //     }
    //     Event::Leave => {
    //       b.set_color(Color::Cyan);
    //       b.redraw();
    //       return true;
    //     }
    //     // using resize changes the behavior of the button and thus needs different behavior
    //     Event::Push => {
    //         b.emit(sender, Message::Spaz);
    //         return true;
    //     }
    //     _ => {
    //       // print!("{:?}", evt);
    //       return false;
    //     },
    // });
    
    
    win.end();
    win.show();
    
    
    while app.wait() {
      // Look into if lets again and is there another way / what is this syntactic sugar for?
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

