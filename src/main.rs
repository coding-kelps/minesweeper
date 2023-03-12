mod grid;

fn main() {
    let grid_str = String::from("ABC\nDEF\nGHI");

    let grid = grid::Grid::from(grid_str);

    println!("grid:\n{}", grid);
}
