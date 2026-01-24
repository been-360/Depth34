
#[derive(Clone, Copy)]
pub struct State {
    pub l: L,
    pub r: R,
}

#[derive(Clone, Copy)]
pub struct L {
    pub lx: f32,
    pub ly: f32,
}

#[derive(Clone, Copy)]
pub struct R {
    pub rx: f32,
    pub ry: f32,
}