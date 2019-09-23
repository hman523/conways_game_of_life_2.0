extern crate colored;
mod board;
use board::Board;
use std::{thread, time};
use colored::*;

fn main() {
    println!("Starting the simulation");
	let mut b = Board::new();
	b = b.fill_random();
	let mut generation = 0;
	let mut maxpop = 0;
	loop{	
		clear();
		let count = b.count();
		maxpop = std::cmp::max(maxpop, count);
		print!("{}{}\t\t", "Generation: ".red(), 
		generation.to_string().blue());
		print!("{}{}\t\t", "Population: ".red(), 
		count.to_string().blue());
		print!("{}{}\n", "Max Population: ".red(), 
		maxpop.to_string().blue());
		b.print();
		b = b.next();
		generation += 1;
		pause();
	}
}

fn pause(){
	thread::sleep(time::Duration::from_millis(200));
}

fn clear() {
	print!("{}[2J", 27 as char);
}
