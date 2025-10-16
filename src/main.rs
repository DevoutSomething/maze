use std::collections::HashSet;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::thread::sleep;
use std::time::Duration;
use minifb::{Window, WindowOptions, Key};

const CELLS: usize = 20;
const WIDTH: usize = CELLS * 2 + 1;
const HEIGHT: usize = CELLS * 2 + 1;
const SCALE: usize = 20; 
const FRAME_DELAY: u64 = 30; 

#[derive(Clone, Copy, Debug)]
enum Cell {
    Wall,
    Path,
}

fn main() {
    // Initialize maze
    let mut maze = vec![vec![Cell::Wall; WIDTH]; HEIGHT];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // Setup window
    let window_width = WIDTH * SCALE;
    let window_height = HEIGHT * SCALE;
    let mut window = Window::new(
        "Animated Maze Generator",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();
    let mut buffer: Vec<u32> = vec![0; window_width * window_height];

    random_maze_gen(&mut maze, 0, 0, &mut visited, &mut window, &mut buffer);

    maze[1][0] = Cell::Path;
    maze[HEIGHT-2][WIDTH-1] = Cell::Path;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_maze(&maze, &mut buffer);
        window.update_with_buffer(&buffer, window_width, window_height).unwrap();
    }
}

// DFS with animation
fn random_maze_gen(
    maze: &mut Vec<Vec<Cell>>,
    ci: usize,
    cj: usize,
    visited: &mut HashSet<(usize, usize)>,
    window: &mut Window,
    buffer: &mut [u32]
) {
    visited.insert((ci, cj));
    let r = ci * 2 + 1;
    let c = cj * 2 + 1;
    maze[r][c] = Cell::Path;

    // Draw current state
    draw_maze(maze, buffer);
    window.update_with_buffer(buffer, WIDTH * SCALE, HEIGHT * SCALE).unwrap();
    sleep(Duration::from_millis(FRAME_DELAY));

    let mut neighbors = Vec::new();
    if ci > 0 { neighbors.push((ci-1, cj)); }
    if ci < CELLS-1 { neighbors.push((ci+1, cj)); }
    if cj > 0 { neighbors.push((ci, cj-1)); }
    if cj < CELLS-1 { neighbors.push((ci, cj+1)); }

    neighbors.shuffle(&mut thread_rng());

    for &(ni, nj) in &neighbors {
        if !visited.contains(&(ni, nj)) {
            let wall_r = r as isize + (ni as isize - ci as isize);
            let wall_c = c as isize + (nj as isize - cj as isize);
            maze[wall_r as usize][wall_c as usize] = Cell::Path;

            random_maze_gen(maze, ni, nj, visited, window, buffer);
        }
    }
}

fn draw_maze(maze: &Vec<Vec<Cell>>, buffer: &mut [u32]) {
    let window_width = WIDTH * SCALE;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = match maze[y][x] {
                Cell::Wall => 0x000000,  
                Cell::Path => 0xFFFFFF,  
            };
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    let px = x * SCALE + dx;
                    let py = y * SCALE + dy;
                    buffer[py * window_width + px] = color;
                }
            }
        }
    }
}
