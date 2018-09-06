#[cfg(feature = "web-spa")]
#[macro_use]
extern crate yew;
#[cfg(feature = "web-spa")]
extern crate stdweb;
extern crate chrono;
extern crate dimensioned as dim;
extern crate steel_cent;

mod data;

use data::Invoice;
use data::Piece;

#[cfg(not(feature = "web-spa"))]
fn main() {
    // TODO - update this with something
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

#[cfg(feature = "web-spa")]
mod web;

#[cfg(feature = "web-spa")]
fn main() {
    use web::*;
    use yew::prelude::*;
    use yew::services::console::ConsoleService;
    use yew::services::storage::{Area, StorageService};

    let context = Context {
        console: ConsoleService::new(),
        local_store: StorageService::new(Area::Local),
    };

    yew::initialize();

    let app: App<_, RootModel> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
