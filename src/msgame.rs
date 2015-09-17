#![allow(dead_code)]
extern crate rand;

use std::collections::HashMap;

const MMIN: u8 = 16;
const MMAX: u8 = 64;

pub struct World {
	allocated: u64,
	activated: u64,
	board: HashMap<(i32,i32),Chunk>,
}

impl World {
	pub fn new() -> World {
		World {
			allocated: 0,
			activated: 0,
			board: HashMap::new(),
		}
	}
	
	pub fn touch(&mut self, row: i32, col: i32) {
		for i in -1..2 {
			for j in -1..2 { 
				if !self.board.contains_key(&(row+i, col+j)) {
					self.board.insert((row+i,col+j), Chunk::new());
					self.allocated += 1;
				}
			}
		}
		self.calc_neighbors(row, col);
		self.activated += 1;
	}

	#[allow(unused_variables)]
	fn calc_neighbors(&mut self, row: i32, col: i32) {}
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


enum ChunkStat {
	Mined,
	Neighbored,
	Won,
}

struct Chunk {
	stat: ChunkStat, // status
	mines: [u16;16], // mines
	vis: [u16;16], // visibility
	nhb: [u64;16], // neighbors
}

impl Chunk {
	fn new() -> Chunk {
		let mut c = Chunk {
			stat: ChunkStat::Mined,
			mines: [0;16],
			vis: [0;16],
			nhb: [0;16],
		};

		for _ in 1..(rand::random::<u8>()%(MMAX - MMIN) + MMIN+ 1) {
			// duplicate entries are not of consequence.
			c.enmine(rand::random::<u8>()%16, rand::random::<u8>()%16);
		}
		
		return c;
	}
	
	fn enmine(&mut self, row: u8, col: u8) {
		if row<15 && col<15 { self.mines[row as usize] |= 1u16<<(15-col); }
	}
	
	fn click (&mut self, row: u8, col: u8) {
		if row<15 && col<15 {   self.vis[row as usize] |= 1u16<<(15-col); }
	}

	fn is_clicked (&self, row: u8, col: u8) -> bool {
		if row<15 && col<15 {
			self.vis[row as usize] & 1u16<<(15-col) == 1u16<<(15-col)
		}
		else { false }
	}
}
