extern crate ncurses;
extern crate hyper;

use ncurses::*;

fn main() {
	initscr();
	noecho();

	printw("Hello, World!");

	getch();
	endwin();
}