use image::Rgba;

use crate::caengine::celltypes::CellType;
use crate::caengine::utils::rand_int;
use crate::carender::CaRender;

mod utils;
mod celltypes;

//Public structs

//Basic simulation unit. Holds a CellType, two 8-bit registers, one of which is initialized to a
//random value (100..50), and a clock register for being checking against SandBox.tick()
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    celltype: CellType,
    ra: u8,
    rb: u8,
    clock: u8,
}

//Cell methods
impl Cell {
    pub fn new(celltype: CellType) -> Cell {
        Cell {
            celltype,
            ra: 100 + (self::utils::rand_int(50)) as u8,
            rb: 0,
            clock: 0,
        }
    }

    pub fn update(&self, api: SandApi) {
        self.celltype.update(*self, api);
    }
}

//Pre-made cell using CellType::Empty
static EMPTY_CELL: Cell = Cell {
    celltype: CellType::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

//The container for all simulation elements. Holds width, height, a vector (growable list) of all
//Cells being simulated, and an 8-bit generation register to match against Cell.clock registers.
pub struct SandBox {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    generation: u8,
}

//Api for cells that lets them access neighboring cells. "It-it's... IT'S ALIIIIVVEEEEEEEEEEE..."
pub struct SandApi<'a> {
    x: i32,
    y: i32,
    sandbox: &'a mut SandBox,
}

impl<'a> SandApi<'a> {
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 1 || dx < -1 || dy > 2 || dy < -2 {
            panic!("OOB set")
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.sandbox.width - 1 || ny < 0 || ny > self.sandbox.height - 1 {
            return Cell {
                celltype: CellType::Wall,
                ra: 0,
                rb: 0,
                clock: self.sandbox.generation,
            };
        }
        self.sandbox.get_cell(nx, ny)
    }

    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 1 || dx < -1 || dy > 2 || dy < -2 {
            panic!("OOB set")
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.sandbox.width - 1 || ny < 0 || ny > self.sandbox.height - 1 {
            return;
        }
        let i = self.sandbox.get_index(nx, ny);
        //v.clock += 1
        self.sandbox.cells[i] = v;

        //figure out why clion complains about this being private. Rust docs show it as public

        self.sandbox.cells[i].clock =  self.sandbox.generation.wrapping_add(1);
    }
}

//General public methods
impl SandBox {
    pub fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let idx = self.get_index(x, y);
                self.cells[idx] = EMPTY_CELL;
            }
        }
        let new_cells = (0..self.width * self.height).map(|i| {
            if utils::rand_int(1000) <  100 {
                Cell::new(CellType::Wall)
            } else if i < self.width * self.height / 4 {
                Cell::new(CellType::Sand)
            } else if i < self.width * self.height / 3 {
                Cell::new(CellType::Water)
            } else {
                EMPTY_CELL
            }
        }).collect();
        self.cells = newCells;
    }


    pub fn tick(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        //let i = self.get_index(self.width/2, self.height - (self.height - 2));
        for x in 0..self.width {
            let scanx = if self.generation % 2 == 0 {
                self.width - (1 + x)
            } else {
                x
            };

            for y in 0..self.height {
                let cell = self.get_cell(scanx, y);
                SandBox::update_cell(
                    cell,
                    SandApi {
                        sandbox: self,
                        x: scanx,
                        y,
                    },
                );
            }
        }
        self.generation = self.generation.wrapping_add(1);
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn new(width: i32, height: i32) -> SandBox {
        let cells = (0..width * height).map(|i| {
            if utils::rand_int(1000) <  100 {
                Cell::new(CellType::Wall)
            } else if i < width * height / 4 {
                Cell::new(CellType::Sand)
            } else if i < width * height / 3 {
                Cell::new(CellType::Water)
            } else {
                EMPTY_CELL
            }
        }).collect();

        SandBox {
            width,
            height,
            cells,
            generation: 0,
        }
    }
}

//Encode/Decode
impl SandBox {
    pub fn to_rgba(&self, x: i32, y: i32) -> image::Rgba<u8> {
        return match self.cells[self.get_index(x, y)].celltype {
            CellType::Empty => Rgba([0x0, 0x0, 0x0, 0xFF]),
            CellType::Wall => Rgba([0x2D, 0x12, 0x14, 0xFF]),
            CellType::Sand => Rgba([0xC2, 0xB2, 0x80, 0xFF]),
            CellType::Water => Rgba([0x34, 0x9B, 0xEB, 0xFF]), //Cyan kinda blue
        }
    }
}

//Private methods
impl SandBox {
    fn get_index(&self, x: i32, y: i32) -> usize {
        (x + (y * self.width)) as usize
    }

    fn get_cell(&self, x: i32, y: i32) -> Cell {
        let i = self.get_index(x, y);
        return self.cells[i]
    }

    fn update_cell(cell: Cell, api: SandApi) {
        if cell.clock.wrapping_sub(api.sandbox.generation) == 1 {
            return;
        }
        cell.update(api);
    }
}

