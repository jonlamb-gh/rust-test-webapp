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
        println!("\n{:#?}\n", self.dimension.board_feet());
        &self.description
    }
}
