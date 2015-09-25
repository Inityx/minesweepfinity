#![allow(dead_code)]
extern crate rand;
extern crate ncurses;
use ncurses::*;

use std::collections::HashMap;

const MMIN: u8 = 8;
const MMAX: u8 = 16;

enum SquareView {
	Unclicked { flag: bool, mine: bool},
	Clicked(u8),
}
enum ChunkStat {
	Mined,
	Neighbored,
	Won,
}



pub struct Game {
	world: World,
	lose: bool,
	chunks_won: u32,
	printer: Printer,
}

impl Game {
	pub fn new() -> Game {
		Game {
			world: World::new(),
			lose: false,
			chunks_won: 0,
			printer: Printer::new(),
		}
	}

	pub fn print(&self) {
		self.printer.print(self.chunks_won);
	}
}



struct Printer {
	scroll: (i32,i32),
}

impl Printer {
	fn new() -> Printer {
		initscr();	// create ncurses screen
		cbreak();	// enforce terminal cbreak mode
		start_color(); // initialize colors

		// overlay colors
		init_pair(20, COLOR_WHITE, COLOR_BLACK);
		init_pair(21, COLOR_GREEN, COLOR_BLACK);

		// checkerboard colors
		init_pair(10, COLOR_WHITE, COLOR_YELLOW);
		init_pair(11, COLOR_WHITE, COLOR_BLUE);
		
		// clicked colors
		let click_back = COLOR_BLACK;
		init_pair(0, COLOR_WHITE, click_back);
		init_pair(1, COLOR_WHITE, click_back);
		init_pair(2, COLOR_WHITE, click_back);
		init_pair(3, COLOR_WHITE, click_back);
		init_pair(4, COLOR_WHITE, click_back);
		init_pair(5, COLOR_WHITE, click_back);
		init_pair(6, COLOR_WHITE, click_back);
		init_pair(7, COLOR_WHITE, click_back);
		init_pair(8, COLOR_WHITE, click_back);

		Printer { scroll: (0,0) }
	}

	fn print(&self, won: u32) {
		Printer::print_checkerboard();
		Printer::print_chunks();
		Printer::print_overlay(won);
		refresh();
	}
	
	fn print_checkerboard() {
		let rows = LINES;
		let cols = if COLS%2 == 0 { COLS/2 } else { COLS/2+1 };

		for i in 0..rows { for j in 0..cols {
			attron(COLOR_PAIR(((i+j)%2+10) as i16));
			// mvaddstr(i,j*2,"５"); // ncurses no likey :(
			mvaddch(i,j*2,  ' ' as u64);
			mvaddch(i,j*2+1,' ' as u64);
			attroff(COLOR_PAIR(((i+j)%2+10) as i16));
		} }
   	}

	fn print_chunks() {}

	fn print_overlay(won: u32) {
		attron(COLOR_PAIR(20));
		
		// top LH corner
		mvaddstr(0,0,"   ");
		
		// top edge
		for i in 0..(if COLS%2 == 0 { COLS/2 } else { COLS/2+1 }) {
			attron(COLOR_PAIR(21));
			mvaddch(0,(i*2+3) as i32,(b'a'+(i/26) as u8) as u64);
			attroff(COLOR_PAIR(21));
			mvaddch(0,(i*2+4) as i32,(b'a'+(i%26) as u8) as u64);
		}
		
		// left edge
		// (padding done manually because it's a separate library)
		for i in 0..LINES-2 {
			let temp = 
				match i {
					0...9 =>       format!("  {}", i),
					10...99 =>     format!(" {}", i),
					100...999 =>   format!("{}", i),
					_ => unreachable!("You must have gone to a lot of trouble to get a terminal that tall."),
				};
			mvaddstr(i+1,0,&temp);
		}

		// bottom bar
		for i in 0..COLS {
			mvaddch(LINES-1,i,' ' as u64);
		}
		let temp = format!("Chunks solved: {}", won);
		mvaddstr(LINES-1,4,&temp);

		attroff(COLOR_PAIR(20));
		mv(0,0);
	}
}

impl Drop for Printer {
	fn drop(&mut self) {
		endwin();	// destroy ncurses screen
	}
}



struct World {
	allocated: u64,
	activated: u64,
	board: HashMap<(i32,i32),Chunk>,
}

impl World {
	// Constructor
	fn new() -> World {
		World {
			allocated: 0,
			activated: 0,
			board: HashMap::new(),
		}
	}
	
	// generate chunks given (row, col) click
	fn touch(&mut self, row: i32, col: i32) {
		for i in -1..2 { for j in -1..2 { 
			if !self.board.contains_key(&(row+i, col+j)) {
				self.board.insert((row+i,col+j), Chunk::new());
				self.allocated += 1;
			}
		} }
		self.calc_neighbors(row, col);
		self.activated += 1;
	}

	#[allow(unused_variables)]
	fn calc_neighbors(&mut self, row: i32, col: i32) {
		// assert neighbors are mined
		for i in -1..2 { for j in -1..2 { assert!(self.board.contains_key(&(row+i, col+j))); } }
		let c: &mut Chunk = self.board.get(&(row, col)).unwrap();

		// interior
		for i in 1..7 { for j in 1..7 { c.set_neighbors(i, j, c.calc_square_neighbors(i,j)); }
		
		//edges

		//corners
	}

	// return vector of cells as viewed by player
	fn chunk_view(&self, show_mines: bool, row: i32, col: i32) -> (bool, Vec<SquareView>) {
		let mut v: Vec<SquareView> = Vec::new();
		let c: &Chunk = self.board.get(&(row, col)).unwrap();
		
		for i in 0..8 { for j in 0..8 {
			v.push(
				if c.is_clicked(i,j) {
					SquareView::Clicked(c.get_neighbors(i,j))
				} else {
					SquareView::Unclicked {
						mine: show_mines && c.is_mine(i,j),
						flag: c.is_flag(i,j),
					}
				}
			);
		} }
		return (match c.status { ChunkStat::Won => true, _ => false }, v);
	}
}



struct Chunk {
	status: ChunkStat, // status
	mines: [u8;8],     // mines
	vis: [u8;8],       // visibility
	flags: [u8;8],     // flags
	nhb: [u32;8],      // neighbors
}

impl Chunk {
	// Constructor
	fn new() -> Chunk {
		let mut c = Chunk {
			status: ChunkStat::Mined,
			mines: [0;8],
			vis: [0;8],
			flags: [0;8],
			nhb: [0;8],
		};
		for _ in 1..(rand::random::<u8>()%(MMAX - MMIN) + MMIN+ 1) {
			// duplicate entries are not of consequence.
			c.enmine(rand::random::<u8>()%8, rand::random::<u8>()%8);
		}
		return c;
	}
	
	// Setters
	#[inline]
	fn enmine(&mut self, row: u8, col: u8) {
		self.mines[row as usize] |= 1u8<<(7-col);
	}
	
	#[inline]
	fn click (&mut self, row: u8, col: u8) {
		self.vis  [row as usize] |= 1u8<<(7-col);
	}

	#[inline]
	fn enflag (&mut self, row: u8, col: u8) {
		self.flags[row as usize] |= 1u8<<(7-col);
	}

	#[inline]
	fn set_neighbors(&mut self, row: u8, col: u8, n: u8) {
		self.nhb[row as usize] = (self.nhb[row as usize] & !(15u32<<((7-col)*4))) | (n as u32) << ((7-col)*4);
	}
	
	// Getters
	#[inline]
	fn is_mine (&self, row: u8, col: u8) -> bool {
		self.mines[row as usize] & 1u8<<(7-col) == 1u8<<(7-col)
	}

	#[inline]
	fn is_clicked (&self, row: u8, col: u8) -> bool {
		self.vis  [row as usize] & 1u8<<(7-col) == 1u8<<(7-col)
	}
	
	#[inline]
	fn is_flag (&self, row: u8, col: u8) -> bool {
		self.flags[row as usize] & 1u8<<(7-col) == 1u8<<(7-col)
	}

	#[inline]
	fn get_neighbors(&self, row: u8, col: u8) -> u8 {
		((self.nhb[row as usize] & 15u32<<((7-col)*4))>>((7-col)*4)) as u8
	}

	// neighbor calculation per square
	#[inline]
	fn calc_square_neighbors(&self, row: u8, col: u8) -> u8 {
		1 // TODO
	}
}


#[test]
fn test_neighbors_accessors() {
	let mut c = Chunk::new();

	c.set_neighbors(0,7,10);
	c.set_neighbors(0,6,5);

	assert_eq!(format!("{:b}", c.nhb[0]), "1011010");
	assert_eq!(c.get_neighbors(0,7), 10);
	assert_eq!(c.get_neighbors(0,6), 5);
}

#[test]
fn test_chunk_cascade() {
	let mut w = World::new();
	w.touch(0,0);
	w.touch(0,2);
	w.touch(1,3);
	w.touch(2,1);
	w.touch(1,1);
	assert_eq!(w.allocated, 25);
	assert_eq!(w.activated, 5);
}
