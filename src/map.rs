use crate::types::TileType::{self, *};

pub fn create_farm() -> Vec<Vec<TileType>> {
    let b = Boundary;
    let g = Grass;
    let h = House;
    let pe = PathEast;

    vec![
        vec![b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), h,        g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), pe      ],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone()],
    ]
}

pub fn create_east_path() -> Vec<Vec<TileType>> {
    let b = Boundary;
    let g = Grass;
    let pf = PathFarm;
    let ps = PathSquare;
    let psr = PathSouthRiver;
    let m = Mushroom;

    vec![
        vec![b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), ps,       b.clone(), b.clone(), b.clone(), b.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![pf,        g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), m,         b.clone()],
        vec![b.clone(), b.clone(), psr,       b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone()],
    ]
}

pub fn create_square() -> Vec<Vec<TileType>> {
    let b = Boundary;
    let g = Grass;
    let f = Flower;
    let fn_ = Fountain;
    let ps = PathSquare;

    vec![
        vec![b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone(), b.clone()],
        vec![b.clone(), f,         g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), fn_,       g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), b.clone()],
        vec![b.clone(), b.clone(), b.clone(), b.clone(), ps,        b.clone(), b.clone(), b.clone(), b.clone()],
    ]
}

pub fn create_south_river() -> Vec<Vec<TileType>> {
    let g = Grass;
    let pg = PathSouthRiverGate;
    let r = River;

    vec![
        vec![g.clone(), g.clone(), pg,        g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone()],
        vec![g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone(), g.clone()],
        vec![r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone()],
        vec![r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone(), r.clone()],
    ]
}
