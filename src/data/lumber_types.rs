use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, Debug, PartialEq)]
pub enum LumberType {
    DouglasFir,
    RedPine,
}

// TODO - toml lookup instead?
impl LumberType {
    pub fn to_str(&self) -> &'static str {
        match self {
            LumberType::DouglasFir => "Douglas Fir",
            LumberType::RedPine => "Red Pine",
        }
    }

    pub fn fob_price(&self) -> SmallMoney {
        match self {
            LumberType::DouglasFir => SmallMoney::of_major_minor(USD, 2, 60),
            LumberType::RedPine => SmallMoney::of_major_minor(USD, 1, 13),
        }
    }
}
