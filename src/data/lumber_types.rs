use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, Debug, PartialEq)]
pub enum LumberType {
    DouglasFir,
}

#[derive(Clone, Debug)]
pub struct Lumber {
    description: String,
    fob_price: SmallMoney,
}

// TODO - lookup from somewhere
impl Lumber {
    pub fn new(_lumber_type: LumberType) -> Self {
        Self {
            description: String::from("Douglas Fir"),
            fob_price: SmallMoney::of_major_minor(USD, 2, 60),
        }
    }

    pub fn fob_price(&self) -> &SmallMoney {
        &self.fob_price
    }
}
