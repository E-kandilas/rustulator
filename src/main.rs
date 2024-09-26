use fltk::{
  app,
  button::Button,
  enums::{self, Color, Event, FrameType},
  prelude::*,
};

use self::Message::*;

// Define the Message type
#[derive(Debug, Clone, Copy)]
enum Message {
  Num0, Num1, Num2, Num3, Num4,
  Num5, Num6, Num7, Num8, Num9, Mod, Pow, BkSp,
  C, CE, Plus, Minus, Mult, Div, Dec, Eqs
}

impl std::fmt::Display for Message {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let symbol = match self {
      Message::CE => "CE",
      Message::C => "C",
      Message::Pow => "^",
      Message::BkSp => "Back",
      Message::Mod => "%",
      Message::Div => "/",
      Message::Plus => "+",
      Message::Minus => "-",
      Message::Mult => "*",
      Message::Dec => ".",
      Message::Eqs => "=",
      Message::Num0 => "0",
      Message::Num1 => "1",
      Message::Num2 => "2",
      Message::Num3 => "3",
      Message::Num4 => "4",
      Message::Num5 => "5",
      Message::Num6 => "6",
      Message::Num7 => "7",
      Message::Num8 => "8",
      Message::Num9 => "9",
    };
    write!(f, "{}", symbol)
  }
}

impl Message {
  pub fn iterator() -> std::slice::Iter<'static, Message> {
    static MESSAGES: [Message; 21] = [
      CE, C, BkSp, Mod, Pow, Div, Plus, Minus, Mult,
      Num7, Num8, Num9, Num4, Num5, Num6, Num1, Num2, Num3,
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
const BUTTON_PADDING: i32 = 10;

// COLOR CONSTANTS
const COLOR_BLACK: Color = Color::from_rgb(0, 0, 0);
const COLOR_LIGHT_GRAY: Color = Color::from_rgb(240, 240, 240);
const COLOR_LIGHT_PINK: Color = Color::from_rgb(255, 182, 193);
const COLOR_ORANGE: Color = Color::from_rgb(255, 200, 150);
const COLOR_YELLOW: Color = Color::from_rgb(255, 239, 186);
const COLOR_SKY_BLUE: Color = Color::from_rgb(135, 206, 235);
const COLOR_GREEN: Color = Color::from_rgb(144, 238, 144);
const COLOR_WHITE: Color = Color::from_rgb(255, 255, 255);

// FONT CONSTANTS
const BUTTON_FONT_SIZE: i32 = 18;
const DISPLAY_FONT_SIZE: i32 = 24;
const FONT_TYPE: enums::Font = enums::Font::HelveticaBold;

// EVENT HANDLERS
fn handle_keyboard_event_closure(sender: app::Sender<Message>) -> impl FnMut(&mut fltk::window::DoubleWindow, fltk::enums::Event) -> bool {
  move |_, evt| match evt {
    Event::KeyDown => {
      let key = app::event_key();
      let text = app::event_text();
      match key {
        enums::Key::BackSpace => sender.send(BkSp),
        enums::Key::Enter => sender.send(Eqs),
        _ => {
          if let Some(msg) = match text.as_str() {
            "%" => Some(Message::Mod),
            "." => Some(Message::Dec),
            "+" => Some(Message::Plus),
            "-" => Some(Message::Minus),
            "*" => Some(Message::Mult),
            "/" => Some(Message::Div),
            "^" => Some(Message::Pow),
            "=" => Some(Message::Eqs),
            "0" => Some(Message::Num0),
            "1" => Some(Message::Num1),
            "2" => Some(Message::Num2),
            "3" => Some(Message::Num3),
            "4" => Some(Message::Num4),
            "5" => Some(Message::Num5),
            "6" => Some(Message::Num6),
            "7" => Some(Message::Num7),
            "8" => Some(Message::Num8),
            "9" => Some(Message::Num9),
            _ => None,
          } {
            sender.send(msg);
          }
        }
      }
      true
    }
    _ => false,
  }
}

fn handle_button_event_closure(sender: app::Sender<Message>, button_type: Message, orig_color: Color) -> impl FnMut(&mut Button, Event) -> bool {
  move |b, evt| match evt {
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
      sender.send(button_type);
      true
    }
    _ => false,
  }
}

// Display Creation Functions
fn create_button(sender: app::Sender<Message>, button_type: Message, x: i32, y: i32) -> Button {
  let butt_name = button_type.to_string();
  let mut butt = Button::new(x, y, BUTT_WIDTH, BUTT_HEIGHT, &*butt_name);
  butt.set_label_size(BUTTON_FONT_SIZE);
  butt.set_frame(FrameType::RFlatBox);
  butt.set_color(match button_type {
    Message::CE => COLOR_LIGHT_PINK,
    Message::C => COLOR_YELLOW,
    Message::BkSp => COLOR_ORANGE,
    Message::Eqs => COLOR_GREEN,
    _ => COLOR_LIGHT_GRAY,
  });
  butt.set_label_color(COLOR_BLACK);
  butt.set_label_font(FONT_TYPE);

  let orig_color = butt.color();
  butt.handle(handle_button_event_closure(sender, button_type, orig_color));
  butt
}

fn main() {
  // State
  let mut display_string = String::new();
  let mut previous_string = String::new();
  let mut operation = String::new();
  let mut solution_cache = String::new();

  // Emitter
  let (sender, receiver) = app::channel::<Message>();

  // App and Body
  let app = app::App::default();
  let mut win = fltk::window::DoubleWindow::new(100, 100, WIN_WIDTH, WIN_HEIGHT, "Rustulator").center_screen();

  // Calculator Screen
  let mut display_frame = fltk::frame::Frame::new(20, 10, 320, DISPLAY_HEIGHT, "Display").with_label(&display_string);
  display_frame.set_pos((win.width() - display_frame.width()) / 2, DISPLAY_MARGIN_TOP); // Center horizontally and set top margin
  display_frame.set_label_size(DISPLAY_FONT_SIZE);
  display_frame.set_label_color(COLOR_BLACK);
  display_frame.set_color(COLOR_WHITE);
  display_frame.set_frame(FrameType::RFlatBox);

  // Button Generation
  let mut button_count = 0;
  let mut buttons_per_row = 0;
  let mut y_offset = 50;

  for button_type in Message::iterator() {
    if button_count % 3 == 0 {
      buttons_per_row = 0;
      y_offset += BUTT_HEIGHT + BUTTON_PADDING;
    }
    let x_offset = EDGE_DIST + (buttons_per_row * (BUTT_WIDTH + BUTTON_PADDING));
    create_button(sender, *button_type, x_offset, y_offset);

    buttons_per_row += 1;
    button_count += 1;
  }

  win.end();
  win.show();
  win.handle(handle_keyboard_event_closure(sender));

  while app.wait() {
    if let Some(msg) = receiver.recv() {
      match msg {
        BkSp => {
          display_string.pop();
          display_frame.set_label(&display_string);
        }
        CE => {
          previous_string.clear();
          display_string.clear();
          operation.clear();
          solution_cache.clear();
          display_frame.set_label(&display_string);
        }
        C => {
          display_string.clear();
          display_frame.set_label(&display_string);
        }
        Div | Minus | Mod | Mult | Plus | Pow => {
          if !display_string.is_empty() {
            previous_string = display_string.clone();
            display_string.clear();
            operation = msg.to_string();
            display_frame.set_label(&display_string);
          }
          if !solution_cache.is_empty() {
            previous_string = solution_cache.clone();
            solution_cache.clear();
            display_string.clear();
            operation = msg.to_string();
            display_frame.set_label(&display_string);
          }
        }
        Eqs => {
          if !previous_string.is_empty() {
            let pre_op: f64 = previous_string.parse().unwrap();
            let post_op: f64 = display_string.parse().unwrap();

            let solution = match operation.as_str() {
              "+" => pre_op + post_op,
              "-" => pre_op - post_op,
              "*" => pre_op * post_op,
              "/" => pre_op / post_op,
              "^" => f64::powf(pre_op, post_op),
              "%" => pre_op % post_op,
              _ => post_op,
            };
            display_string = solution.to_string();
            display_frame.set_label(&display_string);
            operation.clear();
            solution_cache = display_string.clone();
          }
        }
        _ => {
          if !solution_cache.is_empty() {
            previous_string.clear();
            display_string.clear();
            operation.clear();
            solution_cache.clear();
            display_frame.set_label(&display_string);
          } else {
            display_string.push_str(&msg.to_string());
            display_frame.set_label(&display_string);
          }
        }
      }
    }
  }

  app.run().unwrap();
}
