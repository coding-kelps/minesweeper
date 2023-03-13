mod grid;

fn main() {
    let grid = grid::Grid::make_empty();

    println!("{}", grid.to_debug_str());
}