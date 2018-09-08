// traits/rename/etc for enumerate/formatting for table view

use data::BoardDimensions;
use data::LumberType;
use steel_cent::SmallMoney;

#[derive(Clone, Debug)]
pub struct BillableItem {
    lumber_type: LumberType,
    description: String,
    board_dimensions: BoardDimensions,
    quantity: usize,
}

impl BillableItem {
    pub fn enumerate_headers() -> &'static [&'static str] {
        &[
            "Lumber Type",
            "Description",
            "Dimensions (T x W x L)",
            "Quantity",
            "BF",
            "fob <LOCATION>",
            "Cost",
            // this provides a column for the edit/delete buttons
            " ",
        ]
    }

    pub fn new() -> Self {
        Self {
            lumber_type: LumberType::DouglasFir,
            description: String::from("PIECE DESCRIPTION"),
            board_dimensions: BoardDimensions::new(),
            quantity: 1,
        }
    }

    pub fn enumerate(&self) -> [String; 7] {
        [
            self.lumber_type.to_str().to_string(),
            self.description.clone(),
            format!("{}", self.board_dimensions),
            self.quantity.to_string(),
            // TODO - config
            format!("{:.3}", self.board_dimensions.board_feet()),
            format!("{}", self.lumber_type.fob_price()),
            format!("{}", self.cost()),
        ]
    }

    pub fn cost(&self) -> SmallMoney {
        let fob_price = self.lumber_type.fob_price();

        (fob_price * self.board_dimensions.board_feet()) * (self.quantity as f64)
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }

    // TODO - where to enforce > 0?
    pub fn set_quantity(&mut self, quantity: usize) {
        self.quantity = quantity;
    }
}
