// TODO - quantity here?

use data::BoardDimensions;
use data::{Lumber, LumberType};
use steel_cent::SmallMoney;

#[derive(Clone, Debug)]
pub struct Piece {
    description: String,
    board_dimensions: BoardDimensions,
    lumber_type: LumberType,
}

impl Piece {
    pub fn new() -> Self {
        Self {
            description: String::from("ADD PIECE DESCRIPTION"),
            board_dimensions: BoardDimensions::new(),
            lumber_type: LumberType::DouglasFir,
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn board_dimensions(&self) -> &BoardDimensions {
        &self.board_dimensions
    }

    pub fn cost(&self) -> SmallMoney {
        let lumber = Lumber::new(self.lumber_type.clone());
        let fob_price = lumber.fob_price();

        fob_price * self.board_dimensions.board_feet()
    }
}
