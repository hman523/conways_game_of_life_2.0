mod board;
use board::Board;
use std::{thread, time};

fn main() {
    println!("Starting the simulation");
	let mut b = Board::new();
	b = b.fill_random();
	let mut generation = 0;
	loop{	
		print!("{}[2J", 27 as char);
		println!("Generation: {}", generation);
		b.print();
		b = b.next();
		generation += 1;
		thread::sleep(time::Duration::from_millis(200));
	}
}
