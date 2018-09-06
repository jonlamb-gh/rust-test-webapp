use chrono::prelude::*;
use data::Piece;

#[derive(Clone, Debug)]
pub struct Invoice {
    date: DateTime<Utc>,
    pieces: Vec<Piece>,
}

impl Invoice {
    pub fn new() -> Self {
        Self {
            date: Utc::now(),
            pieces: Vec::<Piece>::new(),
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
}
