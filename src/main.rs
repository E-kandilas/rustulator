use fltk::{app, button::{self, Button}, frame::Frame, prelude::*, window::Window};


use rand::Rng;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Rustulator");
    // let mut frame = Frame::new(0, 0, 400, 200, "");

    let mut rand_x: i32 = 10;
    let mut rand_y: i32 = 210;

    let mut button_0 = Button::new(rand_x, rand_y, 80, 40, "CLICK!");

    button_0.clone().set_callback(move |_| {
        rand_x = rand::thread_rng().gen_range(0..100);
        rand_y = rand::thread_rng().gen_range(0..100);
        button_0.resize(rand_x, rand_y, 80, 40);
        print!("{}, {}\n", rand_x, rand_y);
    });

    let mut button_1 = Button::new(rand_x, rand_y + 50, 80, 40, "MOVE!");
    button_1.clone().set_callback(move |_| {
        let new_x = rand::thread_rng().gen_range(0..400);
        let new_y = rand::thread_rng().gen_range(0..300);
        button_1.resize(new_x, new_y, 80, 40);
    });


    wind.end();
    wind.show();
    app.run().unwrap();
}

