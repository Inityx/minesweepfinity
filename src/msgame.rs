#![allow(dead_code)]
extern crate rand;
use std::collections::HashMap;

const MIN_M: u8 = 16;
const MAX_M: u8 = 64;

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
	
	fn chunk_create(&mut self, x: i32, y: i32) {
		// TODO: add current chunk, surrounding chunks if not exist
		//		 calculate neighbors for current chunk
		//		 increment size
	}
	
	fn chunk_add(&mut self, x: i32, y: i32) {
		self.board.insert((x,y), Chunk::new());
	}

	fn calc_neighbors(&mut self) {}
}

enum ChunkStat {
	Mined,
	Neighbored,
	Won,
}

pub struct Chunk {
	stat: ChunkStat, // status
	mines: [u16;16], // mines
	vis: [u16;16], // visibility
	nhb: [u64;16], // neighbors
}

impl Chunk {
	pub fn new() -> Chunk {
		let mut c = Chunk {
			stat: ChunkStat::Mined,
			mines: [0;16],
			vis: [0;16],
			nhb: [0;16],
		};

		for _ in 1..(rand::random::<u8>()%(MAX_M-MIN_M)+MIN_M) {
			// duplicate entries are not of consequence.
			c.enmine(rand::random::<u8>()%16, rand::random::<u8>()%16);
		}
		
		return c;
	}
	
	fn enmine(&mut self, row: u8, col: u8) {
		if (row < 15) & (col < 15) {
			self.mines[row as usize] = self.mines[row as usize] | (1u16 << (15-col));
		}
	}
	
	pub fn click (&mut self, row: u8, col: u8) {
		if (row < 15) & (col < 15) {
			self.vis[row as usize] = self.vis[row as usize] | (1u16 << (15-col));
		}
	}

	pub fn is_clicked (&self, row: u8, col: u8) -> bool {
		if (row < 15) & (col < 15) {
			(self.vis[row as usize] & (1u16 << (15-col))) == (1u16 << (15-col))
		}
		else { false }
	}
}
