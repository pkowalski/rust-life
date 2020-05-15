extern crate rand;

mod grid;
mod app;

fn main() {
    let g = grid::Grid::new_random(50, 50);
    app::main(g);
}
