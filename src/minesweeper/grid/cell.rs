/// The possible status that can have a minesweeper cell
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Status {
    /// There is no mine on this cell or next to this one
    Nothing,
    /// There is a mine on this cell
    Mine,
    /// There is one or multiple mine next to this cell,
    /// the number of close mine is associated to this Status
    NearMine(u8),
}

/// A cell of the minesweeper grid
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Cell {
    /// The status (or content) of a cell (is there is a mine, nothing, an hints?)
    pub status: Status,
    /// Does the cell have been discovered by the player
    pub discovered: bool,
}

impl Cell {
    /// Returns an empty cell which has not been discovered
    pub fn new() -> Self {
        Cell {
            status: Status::Nothing,
            discovered: false,
        }
    }
}
