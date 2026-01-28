
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

pub async fn get_default_state() -> State {
    let l = L { lx: 0.0, ly: 0.0 };
    let r = R { rx: 0.0, ry: 0.0 };
    let spec_default = Special {
        mode: false,
    };
    let quad_default = Quad {
        up: false,
        down: false,
        left: false,
        right: false,
    };

    let state = State {
        l,
        r,
        dpad: quad_default,
        face: quad_default,
        special: spec_default,
    };

    state
}