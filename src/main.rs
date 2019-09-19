mod board;
use board::Board;
fn main() {
    println!("Starting the simulation");
	let b = Board::new();
	b.print();
}
