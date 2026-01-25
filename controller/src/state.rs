
#[derive(Clone, Copy, Debug)]
pub struct State {
    pub l: L,
    pub r: R,
    pub dpad: Quad,
    pub face: Quad,
    pub special: Special,
}

#[derive(Clone, Copy, Debug)]
pub struct L {
    pub lx: f32,
    pub ly: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct R {
    pub rx: f32,
    pub ry: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Quad {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Special {
    pub mode: bool,
}