extern crate chrono;
extern crate dimensioned as dim;
extern crate steel_cent;

mod data;

use data::Invoice;
use data::Piece;

fn main() {
    println!("Creating a new invoice");

    let mut invoice = Invoice::new();

    println!("Invoice date: {}", invoice.date());

    println!("Creating a new piece");

    let piece_a = Piece::new();
    invoice.add_piece(piece_a);

    let piece_b = Piece::new();
    invoice.add_piece(piece_b);

    println!("{:#?}", invoice);

    println!("sub_total_cost = {}", invoice.sub_total_cost());
    println!("sales_tax_cost = {}", invoice.sales_tax_cost());
    println!("total_cost = {}", invoice.total_cost());
}
