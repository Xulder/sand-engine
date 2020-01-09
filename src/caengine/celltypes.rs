#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellType {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}