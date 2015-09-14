struct World {

}

impl World {
	chunk_gen () {};
}

struct Chunk {
	mns: [u16;16], // mines
	vis: [u16;16], // visibility
	nhb: [u64;16], // neighbors
}

impl Chunk {
	fn click (&self, row: u8, col: u8) {
		if row < 15 & col < 15 {
			self.vis[row] = self.vis[row] | ( 1 << (15-col));
		}
	}
}


