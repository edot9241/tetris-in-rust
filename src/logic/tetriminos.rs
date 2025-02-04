pub(super) struct Tetrimino {
    pub shape: Shape,
    pub position: _,
    pub rotation: SuperRotation,
}
impl Tetrimino {}
pub enum Shape {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl Shape {}

pub enum SuperRotation {}
