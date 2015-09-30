extern crate rustbox;
extern crate time;

use std::error::Error;
use std::default::Default;
use std::io::Read;
use time::Duration;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}",e),
    };

    let mut lines: Vec<&str> = Vec::new();
    lines.push("Started Simple Server Manager!");
    let mut status = String::new();
    status.push_str("Started Simple Server Manager");
    let mut input = String::new();

    // User Input
    let mut ui_buffer: Vec<u8> = Vec::new();
    let mut reader = std::io::stdin();
    let mut read_std_in = false;

    draw_interface(&rustbox, &status, &input, &lines);
    let mut cur = 0;
    loop {
        match rustbox.peek_event(Duration::milliseconds(33), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Ctrl('q')) => { break; },
                    Some(Key::Ctrl('i')) => { 
                        read_std_in = !read_std_in; 
                        status.clear(); 
                        status.push_str(&format!("Insert Mode: {}", read_std_in)); 
                    },
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
        if (read_std_in) {
            reader.read(&mut ui_buffer);
            ui_buffer.clear();
        }
        input.push_str(&String::from_utf8(ui_buffer).ok().unwrap());
        lines.push("Temp");
        draw_interface(&rustbox, &status, &input, &lines);
        cur += 1;
    }
}

fn draw_interface(rustbox: &RustBox, status: &str, input: &String, lines: &Vec<&str>) {
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
    rustbox.print(0,header_height, rustbox::RB_BOLD, Color::Black, Color::White, &center(status, max_output_width));

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
    rustbox.print(min_draw_width,max_draw_height, rustbox::RB_BOLD, Color::White, Color::Black, &right_justify("Press 'ctrl + i' to enter a command and 'ctrl + q' to quit.", max_output_width));
    
    //draw
    rustbox.present();
}

fn center(message: &str, width: usize) -> String {
    let left_spaces = (width/2)-(message.len()/2);
    let mut result = String::new();
    for _i in (0..left_spaces) {
        result.push_str(" ");
    }
    result.push_str(message);
    for _i in (result.len()..width) {
        result.push_str(" ");
    }
    result
}

fn right_justify(message: &str, width: usize) -> String {
    let left_spaces = width-message.len();
    let mut result = String::new();
    for _i in (0..left_spaces) {
        result.push_str(" ");
    }
    result.push_str(message);
    result
}