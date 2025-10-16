use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;

const CELLS: usize = 9;
const WIDTH: usize = CELLS * 2 + 1;
const HEIGHT: usize = CELLS * 2 + 1;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Wall,
    Path,
}

fn main() {
    let mut maze = vec![vec![Cell::Wall; WIDTH]; HEIGHT];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    random_maze_gen(&mut maze, 0, 0, &mut visited);

    maze[1][0] = Cell::Path;
    maze[HEIGHT - 2][WIDTH - 1] = Cell::Path;

    print_maze(&maze);
}

fn random_maze_gen(
    maze: &mut Vec<Vec<Cell>>,
    ci: usize,
    cj: usize,
    visited: &mut HashSet<(usize, usize)>
) {
    visited.insert((ci, cj));

    let r = ci * 2 + 1;
    let c = cj * 2 + 1;
    maze[r][c] = Cell::Path;

    let mut neighbors = Vec::new();
    if ci > 0 { neighbors.push((ci - 1, cj)); }
    if ci < CELLS - 1 { neighbors.push((ci + 1, cj)); }
    if cj > 0 { neighbors.push((ci, cj - 1)); }
    if cj < CELLS - 1 { neighbors.push((ci, cj + 1)); }

    neighbors.shuffle(&mut thread_rng());

    for &(ni, nj) in &neighbors {
        if !visited.contains(&(ni, nj)) {
            let wall_r = r as isize + (ni as isize - ci as isize);
            let wall_c = c as isize + (nj as isize - cj as isize);
            maze[wall_r as usize][wall_c as usize] = Cell::Path;

            random_maze_gen(maze, ni, nj, visited);
        }
    }
}

fn print_maze(maze: &Vec<Vec<Cell>>) {
    for row in maze {
        for cell in row {
            match cell {
                Cell::Wall => print!(" # "),
                Cell::Path => print!(" . "),
            }
        }
        println!();
    }
}
