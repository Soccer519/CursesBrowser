extern crate ncurses;

use std::char;
use std::fs::{read};
use std::io::{ErrorKind};

use ncurses::*;

fn display_file_contents(path: &String) {
	let content_bytes = match read(path) {
		Ok(p) => p,
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
			println!("Unable to find file: {}.", path);
			return;
		},
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error)
		},
	};

	let mut s = String::new();
	let mut ch: char;
	for c in &content_bytes {
		ch = char::from_u32(*c as u32).unwrap();
		if ch != '\n' {
			s.push(ch);
		} else {
			if s.len() != 0 {
				println!("{}", s);
				s.clear();
			}
		}
	}

	if s.len() != 0 {
		println!("{}", s);
	}
}

fn main() {
	
	initscr();
	raw(); // Allows input to be read right away

	keypad(stdscr(), true);
	noecho();

	let mut ch: i32;
	let mut ch_char: char;
	printw("Enter the filename: ");

	let mut path = String::new();
	loop {
		ch = getch();
		ch_char = char::from_u32(ch as u32).expect("Invalid character.");
		
		if ch_char == '\n' {
			break;
		}

		path.push(ch_char);

		printw(format!("{}", ch_char).as_ref());
		refresh();
	}
	printw("\n");

	endwin();

	display_file_contents(&path);

	// attron(A_BOLD() | A_BLINK());
	// printw(format!("{}\n", char::from_u32(ch as u32).expect("Invalid character!")).as_ref());
	// attroff(A_BOLD() | A_BLINK());
}