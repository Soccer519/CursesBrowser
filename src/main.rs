extern crate ncurses;

use ncurses::*;

fn main() {
	initscr();
	noecho();

	printw("Hello, World!");

	getch();
	endwin();
}