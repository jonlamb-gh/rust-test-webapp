use chrono::prelude::*;
use data::Piece;
use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, Debug)]
pub struct Invoice {
    date: DateTime<Utc>,
    pieces: Vec<Piece>,
    estimated_shipping_cost: SmallMoney,
}

impl Invoice {
    pub fn new() -> Self {
        Self {
            date: Utc::now(),
            pieces: Vec::<Piece>::new(),
            // TODO
            estimated_shipping_cost: SmallMoney::zero(USD),
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.pieces.push(piece);
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.pieces
    }

    pub fn sub_total_cost(&self) -> SmallMoney {
        let mut sum = SmallMoney::zero(USD);

        for p in self.pieces.iter() {
            sum = sum + p.cost();
        }

        sum = sum + self.estimated_shipping_cost;

        sum
    }

    // TODO - sales tax provider, hard-coded to 8.8%
    pub fn sales_tax_cost(&self) -> SmallMoney {
        self.sub_total_cost() * 0.088
    }

    pub fn total_cost(&self) -> SmallMoney {
        self.sub_total_cost() + self.sales_tax_cost()
    }
}
