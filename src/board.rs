extern crate rand;
extern crate colored;
use rand::Rng;
use colored::*;

const SIZEX: usize = 80;
const SIZEY: usize = 22;

pub struct Board {
	board: [[bool; SIZEX]; SIZEY]
}

impl Board {
	pub fn new() -> Board {
		return Board { board: [[false; SIZEX]; SIZEY]};
	}
	
	pub fn count(&self) -> usize{
		let mut count = 0;
		for(_i, row) in self.board.iter().enumerate() {
			count += row.iter().filter(|&x| *x == true).count();
		}
		return count;
	}
	
	pub fn next(&self) -> Board{
		let mut newmap = [[false; SIZEX]; SIZEY];
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
		if y != 0 && x != SIZEY-1 && self.board[x+1][y-1]{
			count += 1;
		}
		if y != SIZEX-1 && x != SIZEY-1 && self.board[x+1][y+1]{
			count += 1;
		}
		if x != SIZEY-1 && self.board[x+1][y]{
			count += 1;
		}
		if x != 0 && y != SIZEX-1 && self.board[x-1][y+1]{
			count += 1;
		}
		if y != SIZEX-1 && self.board[x][y+1]{
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
		let mut newmap = [[false; SIZEX]; SIZEY];
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
					print!("{}", "#".green());
				}
				else{
					print!(" ");
				}
			}
			println!();
		}
	}
}
