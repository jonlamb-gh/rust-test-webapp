extern crate chrono;
extern crate dimensioned as dim;

mod data;

use data::Invoice;
use data::Piece;

fn main() {
    println!("Creating a new invoice");

    let mut invoice = Invoice::new();

    println!("Invoice date: {}", invoice.date());

    println!("Creating a new piece");

    let piece = Piece::new();

    println!("Piece description: {}", piece.description());

    invoice.add_piece(piece);

    println!("{:#?}", invoice);
}
