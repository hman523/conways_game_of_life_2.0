extern crate rand;
use rand::Rng;

const SIZE: usize = 20;

pub struct Board {
	board: [[bool; SIZE]; SIZE]
}

impl Board {
	pub fn new() -> Board {
		return Board { board: [[false; SIZE]; SIZE]};
	}
	
	pub fn next(&self) -> Board{
		let mut newmap = [[false; SIZE]; SIZE];
		for (i, row) in self.board.iter().enumerate() {
			for (j, _col) in row.iter().enumerate() {
				newmap[i][j] = self.will_live(i, j);
			}
		}

		return Board { board: newmap };
	}

	fn count_neighbors(&self, x: usize, y: usize) -> i32 {
		let mut count = 0;
		if x != 0 && self.board[x-1][y]{
			count += 1;
		}
		if x != 0 && y != 0 && self.board[x-1][y-1]{
			count += 1;
		}
		if y != 0 && self.board[x][y-1]{
			count += 1;
		}
		if y != 0 && x != SIZE-1 && self.board[x+1][y-1]{
			count += 1;
		}
		if y != SIZE-1 && x != SIZE-1 && self.board[x+1][y+1]{
			count += 1;
		}
		if x != SIZE-1 && self.board[x+1][y]{
			count += 1;
		}
		if x != 0 && y != SIZE-1 && self.board[x-1][y+1]{
			count += 1;
		}
		if y != SIZE-1 && self.board[x][y+1]{
			count += 1;
		}

		return count;
	}	

	fn will_live(&self, x: usize, y: usize) -> bool{
		let neighbors = self.count_neighbors(x, y);
		
		let is_alive = self.board[x][y];
		if !is_alive {
			if neighbors == 3{
				return true;
			}
		}
		else{
			if neighbors == 2 || neighbors == 3 {
				return true;
			}
		}
		return false;
	}

	fn random(&self) -> bool {
		let mut rng = rand::thread_rng();
		return rng.gen_range(0, 2) == 1;
	}

	pub fn fill_random(&self) -> Board {
		let mut newmap = [[false; SIZE]; SIZE];
		for (i, row) in self.board.iter().enumerate() {
			for (j, _col) in row.iter().enumerate() {
				newmap[i][j] = self.random();
			}
		}

		return Board { board: newmap };

	}

	pub fn print(&self){
		for (_i, row) in self.board.iter().enumerate() {
			for (_j, col) in row.iter().enumerate() {
				if *col{
					print!("#");
				}
				else{
					print!(" ");
				}
			}
			println!();
		}
	}
}
