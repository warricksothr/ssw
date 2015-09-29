extern crate rustbox;

use std::error::Error;
use std::default::Default;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}",e),
    };

    let min_draw_width = 0 as usize;
    let max_draw_width = rustbox.width() - 1;
    let min_output_width = min_draw_width;
    let max_output_width = max_draw_width;
    
    let max_draw_height = rustbox.height() - 1;
    let min_draw_height = 0 as usize;
    let min_output_height = min_draw_height + 1;
    let max_output_height = max_draw_height - 3;
    let seperator_height = max_draw_height - 2;

    rustbox.print(min_output_width,min_output_height, rustbox::RB_BOLD, Color::White, Color::Black, "Hello World!");
    for i in (min_draw_width..max_draw_width) {
        rustbox.print_char(i,seperator_height, rustbox::RB_BOLD, Color::White, Color::White, ' ');
    }
    rustbox.print(min_draw_width,max_draw_height, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'q' to quit.");
    rustbox.present();
    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Char('q')) => { break; }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }
}
