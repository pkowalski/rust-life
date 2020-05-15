use rand::Rng;

pub struct Grid {
    pub cells: Vec<Vec<i32>>,
}

impl Grid {
    pub fn new(cells: Vec<Vec<i32>>) -> Grid {
        let g = Grid { cells };
        g
    }
    pub fn new_random(x: usize, y: usize) -> Grid {
        let mut cells_vec = Vec::<Vec<i32>>::new();
        for _ in 0..x {
            let mut new_vec = Vec::<i32>::new();
            for _ in 0..y {
                let num: i32 = rand::thread_rng().gen_range(0, 10);
                if num > 8 {
                    new_vec.push(1);
                } else {
                    new_vec.push(0);
                }
            }
            cells_vec.push(new_vec);
        }
        let g = Grid { cells: cells_vec };
        g
    }

    pub fn run_rules(&mut self) {
        let mut new_grid = self.cells.clone();

        for (i, val) in self.cells.iter().enumerate() {
            for (j, cell) in val.iter().enumerate() {
                new_grid[i][j] = get_cell_value(*cell, count_neighbors(&self.cells, i, j))
            }
        }
        self.cells = new_grid;
    }

    pub fn display(&self) {
        for (i, val) in self.cells.iter().enumerate() {
            println!("");
            for (j, cell) in val.iter().enumerate() {
                print!("{}",cell)
            }
        }
    }
}

fn count_neighbors(cells: &Vec<Vec<i32>>, i1: usize, j1: usize) -> i32 {
    let mut counter = 0;
    let i = i1 as i32;
    let j = j1 as i32;

    let neighbors = vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    neighbors.iter().for_each(|(i, j)| {
        let ii = *i;
        let jj = *j;
        let cells_len = cells.len() as i32;

        if ii >= 0 && ii < cells_len {
            if jj >= 0 && (jj as usize) < cells[ii as usize].len() {
                if cells[ii as usize][jj as usize] == 1 {
                    counter = counter + 1;
                }
            }
        }
    });
    counter
}

fn get_cell_value(cell_val: i32, neighbors: i32) -> i32 {
    if cell_val == 1 && (neighbors == 2 || neighbors == 3) {
        return 1;
    }

    if cell_val == 0 && neighbors == 3 {
        return 1;
    }

    0
}
