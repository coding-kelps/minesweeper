use rand::Rng;
use std::fmt;
use std::fs;
use std::io::Write;

mod cell {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Status {
        Nothing,
        Mine,
        NearMine(u8),
    }

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub struct Cell {
        pub status: Status,
        pub discovered: bool,
    }

    impl Cell {
        pub fn new() -> Self {
            Cell {
                status: Status::Nothing,
                discovered: false,
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Grid {
    dimensions: (usize, usize),
    rows: Vec<Vec<cell::Cell>>,
}

static STANDARD_DIMENSIONS: (usize, usize) = (10usize, 10usize);

impl Grid {
    pub fn make_empty() -> Self {
        Grid {
            dimensions: STANDARD_DIMENSIONS,
            rows: vec![vec![cell::Cell::new(); STANDARD_DIMENSIONS.1]; STANDARD_DIMENSIONS.0],
        }
    }

    fn generate_bombs(&mut self) {
        static STANDARD_MINE_RATE: i32 = 5i32;

        for cell in self.rows.iter_mut().flat_map(|row| row.iter_mut()) {
            let rand = rand::thread_rng().gen_range(1..=100);

            if rand <= STANDARD_MINE_RATE {
                cell.status = cell::Status::Mine;
            }
        }
    }

    fn count_near_mines(&self, position: (usize, usize)) -> u8 {
        static NEAR_CELL_MODIFIERS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 1),
            (-1, 1),
            (1, 0),
            (1, 1),
        ];
        let mut nb_mines = 0u8;

        for modifier in NEAR_CELL_MODIFIERS {
            let checking_position: (isize, isize) = (
                (position.0 as isize) + modifier.0,
                (position.1 as isize) + modifier.1,
            );

            if checking_position.0 >= 0
                && checking_position.0 < (self.dimensions.0 as isize)
                && checking_position.1 >= 0
                && checking_position.1 < (self.dimensions.1 as isize)
            {
                if self.rows[(checking_position.0 as usize)][(checking_position.1 as usize)].status
                    == cell::Status::Mine
                {
                    nb_mines += 1u8;
                }
            }
        }
        nb_mines
    }

    fn generate_hints(&mut self) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let nb_mines = self.count_near_mines((x, y));
                let cell: &mut cell::Cell = &mut self.rows[x][y];

                if nb_mines > 0u8 && cell.status != cell::Status::Mine {
                    cell.status = cell::Status::NearMine(nb_mines);
                }
            }
        }
    }

    pub fn make_random() -> Self {
        let mut grid = Grid::make_empty();

        grid.generate_bombs();
        grid.generate_hints();
        grid
    }

    pub fn from_debug_str(str: &str) -> Self {
        let mut dimensions = (0usize, 0usize);
        let mut rows: Vec<Vec<cell::Cell>> = Vec::new();
        let mut row: Vec<cell::Cell> = Vec::new();

        for c in str.chars() {
            match c {
                'X' => row.push(cell::Cell {
                    status: cell::Status::Mine,
                    discovered: false,
                }),
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    let nb_mines = c.to_digit(10).unwrap() as u8;

                    row.push(cell::Cell {
                        status: cell::Status::NearMine(nb_mines),
                        discovered: false,
                    });
                }
                '\n' => {
                    if row.len() > dimensions.1 {
                        dimensions.1 = row.len();
                    }
                    if row.len() > 0 {
                        rows.push(row.clone());
                        row.clear();
                    }
                }
                '*' | _ => row.push(cell::Cell {
                    status: cell::Status::Nothing,
                    discovered: false,
                }),
            }
        }
        if row.len() > 0 {
            rows.push(row.clone());
        }
        dimensions.0 = rows.len();
        Grid {
            dimensions: dimensions,
            rows: rows,
        }
    }

    pub fn from_debug_file(file_path: &str) -> Self {
        let contents = fs::read_to_string(file_path).expect("error: cannot open file");

        Grid::from_debug_str(&contents)
    }

    pub fn to_debug_file(&self, file_path: &str) {
        let mut file = fs::File::create(file_path).unwrap();

        writeln!(file, "{}", self.to_debug_str()).unwrap();
    }

    pub fn to_debug_str(&self) -> String {
        let mut str = String::new();

        for (x, row) in self.rows.iter().enumerate() {
            for cell in row {
                match cell.status {
                    cell::Status::Nothing => str.push_str("*"),
                    cell::Status::Mine => str.push_str("X"),
                    cell::Status::NearMine(nb_mines) => match nb_mines {
                        1..=9 => str.push_str(&nb_mines.to_string()),
                        _ => str.push_str("?"),
                    },
                };
            }
            if !(x == (self.dimensions.0 - 1)) {
                str.push_str("\n");
            }
        }
        str
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.rows {
            for cell in row {
                if cell.discovered {
                    match cell.status {
                        cell::Status::Nothing => write!(f, " ")?,
                        cell::Status::Mine => write!(f, "X")?,
                        cell::Status::NearMine(nb_mines) => match nb_mines {
                            1..=9 => write!(f, "{}", nb_mines)?,
                            _ => write!(f, "?")?,
                        },
                    };
                } else {
                    write!(f, "#")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_from_debug_file_empty() {
        let grid_ref = Grid::make_empty();
        let grid_cmp = Grid::from_debug_file("./config/grid/debug/without_hints/empty");

        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_from_debug_file_empty_with_final_break_line() {
        let grid_ref = Grid::make_empty();
        let grid_cmp =
            Grid::from_debug_file("./config/grid/debug/without_hints/empty_with_final_break_line");

        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_from_debug_file_empty_with_hole_lines() {
        let grid_ref = Grid::make_empty();
        let grid_cmp =
            Grid::from_debug_file("./config/grid/debug/without_hints/empty_with_hole_lines");

        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_from_debug_str_empty() {
        let grid_ref = Grid::from_debug_file("./config/grid/debug/without_hints/empty");
        let debug_str = "\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                            ";
        let grid_cmp = Grid::from_debug_str(debug_str);

        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_from_debug_str_many_mines() {
        let grid_ref = Grid::from_debug_file("./config/grid/debug/without_hints/many_mines");
        let debug_str = "\
                                **********\n\
                                *X********\n\
                                **********\n\
                                *****X****\n\
                                ****X*X***\n\
                                *****X****\n\
                                ******X***\n\
                                **********\n\
                                X*********\n\
                                XXX*******\n\
                            ";
        let grid_cmp = Grid::from_debug_str(debug_str);

        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_count_near_mines_one() {
        // grid with mine in position 5,4
        let debug_str = "\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                ****X*****\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                            ";
        let grid = Grid::from_debug_str(debug_str);

        let nb_mines = grid.count_near_mines((5, 5));
        assert_eq!(1, nb_mines);
    }

    #[test]
    fn test_count_near_mines_many() {
        // grid with mines in position 6,4; 4,4; 6,6; 6, 5
        let debug_str = "\
                                **********\n\
                                **********\n\
                                **********\n\
                                **********\n\
                                ****X*****\n\
                                **********\n\
                                ****XXX***\n\
                                **********\n\
                                **********\n\
                                **********\n\
                            ";
        let grid = Grid::from_debug_str(debug_str);

        let nb_mines = grid.count_near_mines((5, 5));
        assert_eq!(4, nb_mines);
    }

    #[test]
    fn test_generates_hints_one_mine() {
        let grid_ref = Grid::from_debug_file("./config/grid/debug/with_hints/one_mine");
        let mut grid_cmp =
            Grid::from_debug_file("./config/grid/debug/without_hints/one_mine");

        grid_cmp.generate_hints();
        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_generates_hints_corner_mines() {
        let grid_ref = Grid::from_debug_file("./config/grid/debug/with_hints/corner_mines");
        let mut grid_cmp =
            Grid::from_debug_file("./config/grid/debug/without_hints/corner_mines");

        grid_cmp.generate_hints();
        assert_eq!(grid_ref, grid_cmp);
    }

    #[test]
    fn test_generates_hints_many_mines() {
        let grid_ref = Grid::from_debug_file("./config/grid/debug/with_hints/many_mines");
        let mut grid_cmp =
            Grid::from_debug_file("./config/grid/debug/without_hints/many_mines");

        grid_cmp.generate_hints();
        println!("{}", grid_cmp.to_debug_str());
        assert_eq!(grid_ref, grid_cmp);
    }
}
