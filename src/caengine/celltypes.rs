use caengine::Cell;
use caengine::SandApi;

use crate::caengine;
use crate::caengine::EMPTY_CELL;

use super::utils::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
}

impl CellType {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            CellType::Empty => {}
            CellType::Wall => {}
            CellType::Sand => update_sand(cell, api),
            CellType::Water => update_water(cell, api)
        }
    }
}

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let dx = rand_dir2();

    let mut nbr = api.get(0, 1);
    if nbr.celltype == CellType::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if nbr.celltype == CellType::Water {
        api.set(0, 0, nbr);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).celltype == CellType::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 1).celltype == CellType::Water {
        nbr = api.get(dx, 1);
        api.set(0, 0, nbr);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_water(cell: Cell, mut api: SandApi) {
    let dx = rand_dir2();

    let nbr = api.get(0, 1);
    if nbr.celltype == CellType::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).celltype == CellType::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).celltype == CellType::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}