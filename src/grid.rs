enum Status {
    Nothing,
    Mine,
    NearBomb(u8),
}

struct Cell {
    status: Status,
    discovered: bool,
}

type Grid = Vec<Vec<Cell>>;

pub fn print() {
    println!("Salut, Monde!");
}