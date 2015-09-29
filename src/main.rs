extern crate rustbox;
extern crate time;

use std::error::Error;
use std::default::Default;
use time::Duration;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}",e),
    };

    let mut lines: Vec<&str> = Vec::new();
    lines.push("Hello World!");

    draw_interface(&rustbox, &lines);
    let mut cur = 0;
    loop {
        match rustbox.peek_event(Duration::milliseconds(33), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Ctrl('q')) => { break; }
                    Some(Key::Ctrl('i')) => {  }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
        lines.push("Temp");
        draw_interface(&rustbox, &lines);
        cur += 1;
    }
}

fn draw_interface(rustbox: &RustBox, lines: &Vec<&str>) {
    let min_draw_width = 0 as usize;
    let max_draw_width = rustbox.width() - 1;
    let min_output_width = min_draw_width;
    let max_output_width = max_draw_width;
    
    let max_draw_height = rustbox.height() - 1;
    let min_draw_height = 0 as usize;
    let min_output_height = min_draw_height + 1;
    let max_output_height = max_draw_height - 3;
    
    let header_height = min_draw_height;
    let seperator_height = max_draw_height - 2;

    //Clean the interface
    rustbox.clear();
    
    // Print a header
    for i in (min_draw_width..max_draw_width) {
        rustbox.print_char(i,header_height, rustbox::RB_BOLD, Color::White, Color::White, ' ');
    }

    // Determine how much we can print
    let mut lines_print_start = 0;
    let lines_print_end = lines.len();
    if lines.len() > max_output_height {
        lines_print_start = lines.len() - max_output_height - min_output_height;
    }

    let mut line = min_output_height;
    for index in (lines_print_start..lines_print_end) {
        rustbox.print(min_output_width,line, rustbox::RB_BOLD, Color::White, Color::Black, lines[index]);
        line += 1;
    }
    
    // Print a footer seperator
    for i in (min_draw_width..max_draw_width) {
        rustbox.print_char(i,seperator_height, rustbox::RB_BOLD, Color::White, Color::White, ' ');
    }

    // Print a control line
    rustbox.print(min_draw_width,max_draw_height, rustbox::RB_BOLD, Color::White, Color::Black, "Press 'ctrl + i' to enter a command and 'ctrl + q' to quit.");
    
    //draw
    rustbox.present();
}
