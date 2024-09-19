#[allow(dead_code)]
use fltk::{prelude::*, button::Button, enums::{Color, FrameType}, app, *};
use fltk::enums::Event;

use self::Message::*;

// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
  Num0, Num1, Num2, Num3, Num4,
  Num5, Num6, Num7, Num8, Num9, LPar, RPar, Pow, BkSp,
  CE, Plus, Minus, Mult, Div, Dec, Eqs
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
      Message::LPar => write!(f, "("),
      Message::RPar => write!(f, ")"),
      Message::Pow => write!(f, "^"),
      Message::CE => write!(f, "CE"),
      Message::BkSp => write!(f, "Back"),
      Message::Plus => write!(f, "+"),
      Message::Minus => write!(f, "-"),
      Message::Mult => write!(f, "*"),
      Message::Div => write!(f, "/"),
      Message::Dec => write!(f, "."),
      Message::Eqs => write!(f, "=")
    }
  }
}

impl Message {
  pub fn iterator() -> std::slice::Iter<'static, Message> {
    static MESSAGES: [Message; 21] = [
      CE, Pow, BkSp,
      LPar, RPar, Div,
      Plus, Minus, Mult,
      Num7, Num8, Num9,
      Num4, Num5,
      Num6, Num1,
      Num2, Num3,
      Num0, Dec, Eqs
    ];
    MESSAGES.iter()
  }
}

// SIZE CONSTANTS
const BUTT_HEIGHT: i32 = 42;
const BUTT_WIDTH: i32 = 88;
const DISPLAY_HEIGHT: i32 = 60;
const DISPLAY_MARGIN_TOP: i32 = 20;
const EDGE_DIST: i32 = 60;
const WIN_HEIGHT: i32 = 480;
const WIN_WIDTH: i32 = 400;

// COLOR CONSTANTS
const COLOR_BLACK: Color = Color::from_rgb(0, 0, 0);
const COLOR_LIGHT_GRAY: Color = Color::from_rgb(240, 240, 240);
const COLOR_LIGHT_PINK: Color = Color::from_rgb(255, 182, 193);
const COLOR_ORANGE: Color = Color::from_rgb(255, 200, 150);
const COLOR_SKY_BLUE: Color = Color::from_rgb(135, 206, 235);
const COLOR_GREEN: Color = Color::from_rgb(144, 238, 144);
const COLOR_WHITE: Color = Color::from_rgb(255, 255, 255);

// FONT CONSTANTS
const BUTTON_FONT_SIZE: i32 = 18;
const DISPLAY_FONT_SIZE: i32 = 24;
const FONT_TYPE: enums::Font = enums::Font::HelveticaBold;

fn main() {
  // APP
  let mut display_string = String::new();

  let app = app::App::default().with_scheme(fltk::app::Scheme::Gtk);

  let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH, WIN_HEIGHT, "Rustulator").center_screen();
  let mut display_frame = fltk::frame::Frame::new(20, 10, 320, DISPLAY_HEIGHT, "Display").with_label(&display_string);
  display_frame.set_pos((win.width() - display_frame.width()) / 2, DISPLAY_MARGIN_TOP); // Center horizontally and set top margin
  display_frame.set_label_size(DISPLAY_FONT_SIZE);
  display_frame.set_label_color(COLOR_BLACK); 
  display_frame.set_color(COLOR_WHITE);
  display_frame.set_frame(FrameType::RFlatBox);

  let (sender, reciever) = app::channel::<Message>();

  // Button Generation
  let mut butt_count = 0;
  let mut butts_per_row = 0;
  let mut y_offset = 50;
  let button_padding = 10;
  for button_type in Message::iterator() {
    let butt_name = button_type.to_string();

    if butt_count % 3 == 0 {
      butts_per_row = 0;
      y_offset += BUTT_HEIGHT + button_padding;
    }
    let butt_dist = EDGE_DIST + (butts_per_row * (BUTT_WIDTH + button_padding));
    let mut butt = Button::new(butt_dist, y_offset, BUTT_WIDTH, BUTT_HEIGHT, &*butt_name);
    butt.set_label_size(BUTTON_FONT_SIZE);
    butt.set_frame(FrameType::RFlatBox);
    butt.set_color(match button_type {
      Message::CE => COLOR_LIGHT_PINK,
      Message::BkSp => COLOR_ORANGE,
      Message::Eqs => COLOR_GREEN,
      _ => COLOR_LIGHT_GRAY,
    });
    butt.set_label_color(COLOR_BLACK);
    butt.set_label_font(FONT_TYPE);

    let orig_color = butt.color();
    butt.handle(move |b, evt| match evt {
      Event::Enter => {
        b.set_color(COLOR_SKY_BLUE);
        b.set_label_color(COLOR_WHITE);
        b.redraw();
        true
      }
      Event::Leave => {
        b.set_color(orig_color);
        b.set_label_color(COLOR_BLACK);
        b.redraw();
        true
      }
      Event::Push => {
        sender.send(*button_type);
        true
      }
      _ => false,
    });

    butts_per_row += 1;
    butt_count += 1;
  }

  win.end();
  win.show();

  while app.wait() {
    if let Some(msg) = reciever.recv() {
      match msg {
        BkSp => {
          display_string.pop();
          display_frame.set_label(&display_string);
        }
        CE => {
          display_string.clear();
          display_frame.set_label(&display_string);
        }
        _ => {
          display_string.push_str(&msg.to_string());
          display_frame.set_label(&display_string);
        }
      }
    }
  }

  app.run().unwrap();
}
