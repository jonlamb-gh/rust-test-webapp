use dim::fps;

// TODO - some sort of BoardDimension type?

#[derive(Clone, Debug)]
pub struct Piece {
    length: fps::Foot<f64>,
    width: fps::Foot<f64>,
    thickness: fps::Foot<f64>,
}
