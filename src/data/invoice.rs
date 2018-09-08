//use chrono::prelude::*;
use data::BillableItem;
use steel_cent::currency::USD;
use steel_cent::SmallMoney;

#[derive(Clone, Debug)]
pub struct Invoice {
    // TODO - how to get time with our wasm target?
    //date: DateTime<Utc>,
    items: Vec<BillableItem>,
    estimated_shipping_cost: SmallMoney,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Summary {
    total_pieces: usize,
    estimated_shipping_cost: SmallMoney,
    sub_total_cost: SmallMoney,
    sales_tax_cost: SmallMoney,
    total_cost: SmallMoney,
}

impl Invoice {
    pub fn enumerate_headers() -> &'static [&'static str] {
        &[
            "Total Pieces",
            "Estimated Shipping",
            "Sub Total",
            "Sales Tax 8.8%",
            "Total Cost",
        ]
    }

    pub fn new() -> Self {
        Self {
            // this panics in the chrome console
            //date: Utc::now(),
            items: Vec::<BillableItem>::new(),
            // TODO
            estimated_shipping_cost: SmallMoney::zero(USD),
        }
    }

    pub fn summary(&self) -> Summary {
        Summary {
            // TODO - once pieces have quantity this needs to change
            total_pieces: self.items.len(),
            estimated_shipping_cost: self.estimated_shipping_cost,
            sub_total_cost: self.sub_total_cost(),
            sales_tax_cost: self.sales_tax_cost(),
            total_cost: self.total_cost(),
        }
    }

    /*
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
    */

    pub fn add_billable_item(&mut self, item: BillableItem) {
        self.items.push(item);
    }

    pub fn items(&self) -> &[BillableItem] {
        &self.items
    }

    pub fn sub_total_cost(&self) -> SmallMoney {
        let mut sum = SmallMoney::zero(USD);

        for i in self.items.iter() {
            sum = sum + i.cost();
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

impl Default for Summary {
    fn default() -> Self {
        Summary {
            total_pieces: 0,
            estimated_shipping_cost: SmallMoney::zero(USD),
            sub_total_cost: SmallMoney::zero(USD),
            sales_tax_cost: SmallMoney::zero(USD),
            total_cost: SmallMoney::zero(USD),
        }
    }
}

impl Summary {
    pub fn enumerate(&self) -> [String; 5] {
        [
            self.total_pieces.to_string(),
            format!("{}", self.estimated_shipping_cost),
            format!("{}", self.sub_total_cost),
            format!("{}", self.sales_tax_cost),
            format!("{}", self.total_cost),
        ]
    }
}
