use chrono::prelude::*;
use data::Piece;

#[derive(Clone, Debug)]
pub struct Invoice {
    date: DateTime<Utc>,
    pieces: Vec<Piece>,
}

impl Invoice {
    pub fn new(pieces: Vec<Piece>) -> Self {
        Self {
            date: Utc::now(),
            pieces,
        }
    }
}
