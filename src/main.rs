use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    // 0 is the index of what I want to print, b means print it in binary and 08 means add padding 0s to reach a length of 8
                    // The \r means return, it takes the cursor to the start of the line, and avoids the
                    // printed lines to become staggered, the lines being printed like a staircase
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b); 
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }
                if c == 'q' {
                    break;
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
    disable_raw_mode().unwrap();
}
