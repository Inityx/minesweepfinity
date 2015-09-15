#![allow(dead_code)]
extern crate rand;

pub struct World {
	size: u64,
}

impl World {
	// enmine();
	// calc_neighbors()
}

pub struct Chunk {
	mines: [u16;16], // mines
	vis: [u16;16], // visibility
	nhb: [u64;16], // neighbors
}

impl Chunk {
	pub fn new() -> Chunk {
		Chunk {
			mines: [0;16],
			vis: [0;16],
			nhb: [0;16],
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


#[test]
fn test_chunk_construct() {
	let c = Chunk::new();
	for i in 0..15 { assert_eq!(c.mines[i], 0); }
	for i in 0..15 { assert_eq!(c.vis[i], 0); }
	for i in 0..15 { assert_eq!(c.nhb[i], 0); }
}

#[test]
fn test_chunk_click() {
	let mut c = Chunk::new();
	c.click(0, 2);
	assert_eq!(c.vis[0], 8192);
}

#[test]
fn test_chunk_is_clicked() {
	let mut vec = Vec::new();
	let mut c = Chunk::new();
	for _ in 0..15 { vec.push((rand::random::<u8>()%15, rand::random::<u8>()%15)); }
	for i in &vec { c.click(i.0, i.1); };
	for i in &vec { assert!(c.is_clicked(i.0, i.1)); }
}


