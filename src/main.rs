extern crate chrono;
extern crate dimensioned as dim;

mod data;

use data::Invoice;
use data::Piece;

fn main() {
    let pieces = Vec::<Piece>::new();
    let invoice = Invoice::new(pieces);

    println!("{:#?}", invoice);
}
