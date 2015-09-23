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
	chunks_won: u64,
	scroll: (i32,i32),
}

impl Game {
	pub fn new() -> Game {
		initscr();	// create ncurses screen
		cbreak();	// enforce terminal cbreak mode
		start_color(); // initialize colors

		// checkerboard colors
		init_pair(10, COLOR_WHITE, COLOR_GREEN);
		init_pair(11, COLOR_WHITE, COLOR_BLUE);
		
		// clicked colors
		let click_back = COLOR_BLACK;
		init_pair(1, COLOR_BLACK, click_back);
		init_pair(2, COLOR_BLACK, click_back);
		init_pair(3, COLOR_BLACK, click_back);
		init_pair(4, COLOR_BLACK, click_back);
		init_pair(5, COLOR_BLACK, click_back);
		init_pair(6, COLOR_BLACK, click_back);
		init_pair(7, COLOR_BLACK, click_back);
		init_pair(8, COLOR_BLACK, click_back);

		Game {
			world: World::new(),
			lose: false,
			chunks_won: 0,
			scroll: (0,0),
		}
	}

	pub fn print(&mut self) {
		Game::checkerboard();
		refresh();
	}

	fn checkerboard() {
		let rows = LINES;
		let cols = if COLS%2 == 0 { COLS/2 } else { COLS/2+1 };

		for i in 0..rows { for j in 0..cols {
			attron(COLOR_PAIR(((i+j)%2+10) as i16));
			mvaddch(i,j*2,  ' ' as u64);
			mvaddch(i,j*2+1,' ' as u64);
			attroff(COLOR_PAIR(((i+j)%2+10) as i16));
		} }
   	}
}

impl Drop for Game {
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
	fn calc_neighbors(&mut self, row: i32, col: i32) {}

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
