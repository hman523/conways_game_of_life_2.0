const SIZE: usize = 20;

pub struct Board {
	board: [[bool; SIZE]; SIZE]
}

impl Board {
	pub fn new() -> Board {
		Board { board: [[false; SIZE]; SIZE]}
	}
	
	pub fn next(&self){
		//let mut newmap = [[false; SIZE]; SIZE];
	}

	pub fn fill_random(&self){

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
