mod grid;

fn main() {
    let grid = grid::Grid::make_random();

    println!("{}", grid);
}