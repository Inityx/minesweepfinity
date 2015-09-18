#![allow(dead_code)]
extern crate rand;

use std::collections::HashMap;

const MMIN: u8 = 16;
const MMAX: u8 = 64;

pub struct World {
	lose: bool,
	allocated: u64,
	activated: u64,
	board: HashMap<(i32,i32),Chunk>,
}

impl World {
	// Constructor
	pub fn new() -> World {
		World {
			lose: false,
			allocated: 0,
			activated: 0,
			board: HashMap::new(),
		}
	}
	
	pub fn touch(&mut self, row: i32, col: i32) {
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

	pub fn chunk_view(&self, show_mines: bool, row: i32, col: i32) -> (bool, Vec<SquareView>) {
		let mut v: Vec<SquareView> = Vec::new();
		let c: &Chunk = self.board.get(&(row, col)).unwrap();
		
		for i in 0..16 { for j in 0..16 {
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

enum SquareView {
	Unclicked { flag: bool, mine: bool},
	Clicked(u8),
}
enum ChunkStat {
	Mined,
	Neighbored,
	Won,
}

struct Chunk {
	status: ChunkStat,  // status
	mines: [u16;16],    // mines
	vis: [u16;16],      // visibility
	flags: [u16;16],    // flags
	nhb: [u64;16],// neighbors
}

impl Chunk {
	// Constructor
	fn new() -> Chunk {
		let mut c = Chunk {
			status: ChunkStat::Mined,
			mines: [0;16],
			vis: [0;16],
			flags: [0;16],
			nhb: [0;16],
		};
		for _ in 1..(rand::random::<u8>()%(MMAX - MMIN) + MMIN+ 1) {
			// duplicate entries are not of consequence.
			c.enmine(rand::random::<u8>()%16, rand::random::<u8>()%16);
		}
		return c;
	}
	
	// Setters
	fn enmine(&mut self, row: u8, col: u8) {
		self.mines[row as usize] |= 1u16<<(15-col);
	}
	
	fn click (&mut self, row: u8, col: u8) {
		self.vis  [row as usize] |= 1u16<<(15-col);
	}

	fn enflag (&mut self, row: u8, col: u8) {
		self.flags[row as usize] |= 1u16<<(15-col);
	}

	fn set_neighbors(&mut self, row: u8, col: u8, n: u8) {
		self.nhb[row as usize] = (self.nhb[row as usize] & !(15u64<<((15-col)*4))) | (n as u64) << ((15-col)*4);
	}
	
	// Getters
	fn is_mine (&self, row: u8, col: u8) -> bool {
		self.mines[row as usize] & 1u16<<(15-col) == 1u16<<(15-col)
	}

	fn is_clicked (&self, row: u8, col: u8) -> bool {
		self.vis  [row as usize] & 1u16<<(15-col) == 1u16<<(15-col)
	}
	
	fn is_flag (&self, row: u8, col: u8) -> bool {
		self.flags[row as usize] & 1u16<<(15-col) == 1u16<<(15-col)
	}

	fn get_neighbors(&self, row: u8, col: u8) -> u8 {
		((self.nhb[row as usize] & 15u64<<((15-col)*4))>>((15-col)*4)) as u8
	}
}

#[test]
fn test_neighbors_accessors() {
	let mut c = Chunk::new();

	c.set_neighbors(0,15,10);
	c.set_neighbors(0,14,5);

	assert_eq!(format!("{:b}", c.nhb[0]), "1011010");
	assert_eq!(c.get_neighbors(0,15), 10);
	assert_eq!(c.get_neighbors(0,14), 5);
}
