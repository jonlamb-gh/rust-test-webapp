use data::BoardDimension;

#[derive(Clone, Debug)]
pub struct Piece {
    description: String,
    dimension: BoardDimension,
}

impl Piece {
    pub fn new() -> Self {
        Self {
            description: String::from("ADD PIECE DESCRIPTION"),
            dimension: BoardDimension::new(),
        }
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn dimension(&self) -> &BoardDimension {
        &self.dimension
    }
}
