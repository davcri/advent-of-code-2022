use core::fmt;

fn main() {
    let input_file = include_str!("input.txt");

    let mut grid: Vec<Vec<Tree>> = Vec::new();

    for (i, line) in input_file.lines().enumerate() {
        let mut row: Vec<Tree> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match String::from(c).parse::<usize>() {
                Ok(val) => {
                    let t = Tree {
                        height: val,
                        visible: false,
                        x: i,
                        y: j,
                    };
                    row.push(t);
                }
                Err(_) => {}
            }
        }
        grid.push(row);
    }

    let mut visiblity_grid: Vec<Vec<bool>> = Vec::new();
    let mut visible_count = 0;
    let mut highest_scenic_score = 0;

    let mut grid_clone = grid.clone();

    for (i, rows) in grid_clone.iter_mut().enumerate() {
        let init_row: Vec<bool> = Vec::new();
        visiblity_grid.push(init_row);
        for (j, el) in rows.iter().enumerate() {
            let score = get_scenic_score(el, grid.clone());
            highest_scenic_score = highest_scenic_score.max(score);
            let tree_visible = is_tree_visible(&el, grid.clone());
            visiblity_grid[i].push(tree_visible);
            if tree_visible {
                visible_count += 1;
            }
        }
    }

    println!("--- {} trees visible", visible_count);
    println!("--- {} scenic score", highest_scenic_score);

    // for rows in visiblity_grid {
    //     for el in rows {
    //         if el {
    //             print!("1");
    //         } else {
    //             print!("0");
    //         }
    //     }
    //     println!();
    // }
}

#[derive(Clone)]
struct Tree {
    height: usize,
    visible: bool,
    x: usize,
    y: usize,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {} {} {}] ",
            self.height, self.visible, self.x, self.y
        )
    }
}

fn is_tree_on_border(t: &Tree, width: usize, height: usize) -> bool {
    t.x == 0 || t.y == 0 || t.x == width - 1 || t.y == height - 1
}

enum Dir {
    Left,
    Right,
    Top,
    Bottom,
}

//A tree's scenic score is found by multiplying together its viewing distance in
//each of the four directions
fn get_scenic_score(t: &Tree, grid: Vec<Vec<Tree>>) -> i32 {
    let mut scenic_score = 1;
    let grid_w = grid.len();
    let grid_h = grid[0].len();

    for dir in [Dir::Left, Dir::Right, Dir::Top, Dir::Bottom] {
        match dir {
            Dir::Left => {
                let mut trees = 0;
                for i in (0..t.y).rev() {
                    let cur_tree = &grid[t.x][i];
                    trees += 1;
                    if t.height > cur_tree.height {
                        continue;
                    } else {
                        break;
                    }
                }
                scenic_score *= trees;
            }
            Dir::Right => {
                let mut trees = 0;
                for i in t.y + 1..grid_w {
                    let cur_tree = &grid[t.x][i];
                    trees += 1;
                    if t.height > cur_tree.height {
                        continue;
                    } else {
                        break;
                    }
                }
                scenic_score *= trees;
            }
            Dir::Top => {
                let mut trees = 0;
                for i in (0..(t.x)).rev() {
                    let cur_tree = &grid[i][t.y];
                    trees += 1;
                    if t.height > cur_tree.height {
                        continue;
                    } else {
                        break;
                    }
                }
                scenic_score *= trees;
            }
            Dir::Bottom => {
                let mut trees = 0;
                for i in t.x + 1..grid_h {
                    let cur_tree = &grid[i][t.y];
                    trees += 1;
                    if t.height > cur_tree.height {
                        continue;
                    } else {
                        break;
                    }
                }
                scenic_score *= trees;
            }
        }
    }
    return scenic_score;
}

// A tree is visible if all of the other trees between it and an edge of the
// grid are shorter than it. Only consider trees in the same row or column;
// that is, only look up, down, left, or right from any given tree.
fn is_tree_visible(t: &Tree, grid: Vec<Vec<Tree>>) -> bool {
    let grid_w = grid.len();
    let grid_h = grid[0].len();
    if is_tree_on_border(&t, grid_w, grid_h) {
        return true;
    }

    for dir in [Dir::Left, Dir::Right, Dir::Top, Dir::Bottom] {
        match dir {
            Dir::Left => {
                for i in (0..t.y).rev() {
                    let cur_tree = &grid[t.x][i];
                    if t.height > cur_tree.height {
                        if i == 0 {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
            }
            Dir::Right => {
                for i in t.y + 1..grid_w {
                    let cur_tree = &grid[t.x][i];
                    if t.height > cur_tree.height {
                        if i == grid_w - 1 {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
            }
            Dir::Top => {
                for i in (0..(t.x)).rev() {
                    let cur_tree = &grid[i][t.y];
                    if t.height > cur_tree.height {
                        if i == 0 {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
            }
            Dir::Bottom => {
                for i in t.x + 1..grid_h {
                    let cur_tree = &grid[i][t.y];
                    if t.height > cur_tree.height {
                        if i == grid_h - 1 {
                            return true;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return false;
}
